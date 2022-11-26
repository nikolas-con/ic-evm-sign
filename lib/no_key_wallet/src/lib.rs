#[cfg(not(test))]
use ic_cdk::api::time as ic_timestamp;
#[cfg(not(test))]
use ic_cdk::call as ic_call;
use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
    Principal,
};
#[cfg(test)]
mod mocks;
#[cfg(test)]
use mocks::{ic_call, ic_timestamp};

pub mod utils;
use utils::{compute_address, get_derivation_path, get_rec_id};

pub mod ecdsa;
use ecdsa::reply::*;
use ecdsa::request::*;

pub mod state;
use state::*;

mod transaction;

use std::cell::RefCell;
use std::collections::HashMap;

#[derive(CandidType, Serialize, Debug)]
pub struct CreateResponse {
    pub address: String,
}
#[derive(CandidType, Deserialize, Debug)]
pub struct SignResponse {
    pub sign_tx: Vec<u8>,
}
#[derive(CandidType, Deserialize, Debug)]
pub struct CallerTransactionsResponse {
    pub transactions: Vec<Transaction>,
}
#[derive(CandidType, Deserialize, Debug)]
pub struct CallerResponse {
    pub address: String,
    pub transactions: Vec<Transaction>,
}

#[derive(Default, CandidType, Deserialize, Debug)]
struct State {
    users: HashMap<Principal, User>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

pub async fn create(principal_id: Principal) -> Result<CreateResponse, String> {
    let users = STATE.with(|s| s.borrow().users.clone());
    let user = users.get(&principal_id);

    if let Some(_) = user {
        return Err("this wallet already exist".to_string());
    }

    let key_id = EcdsaKeyId {
        curve: EcdsaCurve::Secp256k1,
        name: "dfx_test_key".to_string(),
    };

    let caller = get_derivation_path(principal_id);

    let request = ECDSAPublicKey {
        canister_id: None,
        derivation_path: vec![caller],
        key_id: key_id.clone(),
    };

    let (res,): (ECDSAPublicKeyReply,) = ic_call(
        Principal::management_canister(),
        "ecdsa_public_key",
        (request,),
    )
    .await
    .map_err(|e| format!("Failed to call ecdsa_public_key {}", e.1))?;

    let address = compute_address(res.public_key.clone());

    let mut user = User::default();
    user.public_key = res.public_key;

    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.users.insert(principal_id, user);
    });

    Ok(CreateResponse { address })
}

pub async fn sign(
    hex_raw_tx: Vec<u8>,
    chain_id: u64,
    principal_id: Principal,
) -> Result<SignResponse, String> {
    let users = STATE.with(|s| s.borrow().users.clone());
    let user;

    if let Some(i) = users.get(&principal_id) {
        user = i.clone();
    } else {
        return Err("this user does not exist".to_string());
    }
    let tx = transaction::get_transaction(&hex_raw_tx, chain_id).unwrap();

    let message = tx.get_message_to_sign().unwrap();

    assert!(message.len() == 32);

    let key_id = EcdsaKeyId {
        curve: EcdsaCurve::Secp256k1,
        name: "dfx_test_key".to_string(),
    };

    let caller = get_derivation_path(principal_id);

    let request = SignWithECDSA {
        message_hash: message.clone(),
        derivation_path: vec![caller],
        key_id: key_id.clone(),
    };

    let (res,): (SignWithECDSAReply,) = ic_call(
        Principal::management_canister(),
        "sign_with_ecdsa",
        (request,),
    )
    .await
    .map_err(|e| format!("Failed to call sign_with_ecdsa {}", e.1))?;

    let rec_id = get_rec_id(&message, &res.signature, &user.public_key).unwrap();

    let signed_tx = tx
        .sign(res.signature.clone(), u64::try_from(rec_id).unwrap())
        .unwrap();

    STATE.with(|s| {
        let mut state = s.borrow_mut();
        let user = state.users.get_mut(&principal_id).unwrap();
        let mut tx = Transaction::default();
        tx.data = signed_tx.clone();
        tx.timestamp = ic_timestamp();
        user.transactions.push(tx);
    });

    Ok(SignResponse { sign_tx: signed_tx })
}

pub fn get_caller_data(principal_id: Principal) -> Result<CallerResponse, String> {
    let users = STATE.with(|s| s.borrow().users.clone());
    let user;
    if let Some(i) = users.get(&principal_id) {
        user = i.clone();
    } else {
        return Err("this user does not exist".to_string());
    }

    let address = compute_address(user.public_key.clone());

    Ok(CallerResponse {
        address,
        transactions: user.transactions,
    })
}

pub fn clear_caller_history(principal_id: Principal) -> Result<(), String> {
    let users = STATE.with(|s| s.borrow().users.clone());

    if let None = users.get(&principal_id) {
        return Err("this user does not exist".to_string());
    }

    STATE.with(|s| {
        let mut state = s.borrow_mut();
        let user = state.users.get_mut(&principal_id).unwrap();
        user.transactions.clear();
    });

    Ok(())
}

#[ic_cdk_macros::pre_upgrade]
fn pre_upgrade() {
    STATE.with(|s| {
        ic_cdk::storage::stable_save((s,)).unwrap();
    });
}

#[ic_cdk_macros::post_upgrade]
fn post_upgrade() {
    let (s_prev,): (State,) = ic_cdk::storage::stable_restore().unwrap();
    STATE.with(|s| {
        *s.borrow_mut() = s_prev;
    });
}

#[cfg(test)]
mod tests;
