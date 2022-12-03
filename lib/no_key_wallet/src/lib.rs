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
use utils::{get_address_from_public_key, get_derivation_path};

pub mod ecdsa;
use ecdsa::reply::*;
use ecdsa::request::*;

pub mod state;
use state::*;

mod transaction;
use transaction::*;

#[derive(CandidType, Serialize, Debug)]
pub struct CreateResponse {
    pub address: String,
}
#[derive(CandidType, Deserialize, Debug)]
pub struct SignResponse {
    pub sign_tx: Vec<u8>,
}
#[derive(CandidType, Deserialize, Debug)]
pub struct DeployContractResponse {
    pub tx: Vec<u8>,
}
#[derive(CandidType, Deserialize, Debug)]
pub struct TransferERC20Response {
    pub tx: Vec<u8>,
}
#[derive(CandidType, Deserialize, Debug)]
pub struct CallerTransactionsResponse {
    pub transactions: Vec<Transaction>,
}
#[derive(CandidType, Deserialize, Debug)]
pub struct CallerResponse {
    pub address: String,
    pub transactions: ChainData,
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

    let address = get_address_from_public_key(res.public_key.clone()).unwrap();

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
    let mut tx = transaction::get_transaction(&hex_raw_tx, chain_id.clone()).unwrap();

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

    let signed_tx = tx.sign(res.signature.clone(), user.public_key).unwrap();

    STATE.with(|s| {
        let mut state = s.borrow_mut();
        let user = state.users.get_mut(&principal_id).unwrap();

        let mut transaction = Transaction::default();
        transaction.data = signed_tx.clone();
        transaction.timestamp = ic_timestamp();

        if let Some(user_tx) = user.transactions.get_mut(&chain_id) {
            user_tx.transactions.push(transaction);
            user_tx.nonce = tx.get_nonce().unwrap() + 1;
        } else {
            let mut chain_data = ChainData::default();
            chain_data.nonce = tx.get_nonce().unwrap() + 1;
            chain_data.transactions.push(transaction);

            user.transactions.insert(chain_id, chain_data);
        }
    });

    Ok(SignResponse { sign_tx: signed_tx })
}

pub async fn deploy_contract(
    principal_id: Principal,
    bytecode: Vec<u8>,
    chain_id: u64,
    max_priority_fee_per_gas: u64,
    gas_limit: u64,
    max_fee_per_gas: u64,
) -> Result<DeployContractResponse, String> {
    let users = STATE.with(|s| s.borrow().users.clone());
    let user;

    if let Some(i) = users.get(&principal_id) {
        user = i.clone();
    } else {
        return Err("this user does not exist".to_string());
    }

    let nonce: u64;
    if let Some(user_transactions) = user.transactions.get(&chain_id) {
        nonce = user_transactions.nonce;
    } else {
        nonce = 0;
    }
    let data = "0x".to_owned() + &utils::vec_u8_to_string(&bytecode);
    let tx = transaction::Transaction1559 {
        nonce,
        chain_id,
        max_priority_fee_per_gas,
        gas_limit,
        max_fee_per_gas,
        to: "0x".to_string(),
        value: 0,
        data,
        access_list: vec![],
        v: "0x00".to_string(),
        r: "0x00".to_string(),
        s: "0x00".to_string(),
    };

    let raw_tx = tx.serialize().unwrap();
    let res = sign(raw_tx, chain_id, principal_id).await.unwrap();

    Ok(DeployContractResponse { tx: res.sign_tx })
}

pub async fn transfer_erc_20(
    principal_id: Principal,
    chain_id: u64,
    max_priority_fee_per_gas: u64,
    gas_limit: u64,
    max_fee_per_gas: u64,
    address: String,
    value: u64,
    contract_address: String,
) -> Result<TransferERC20Response, String> {
    let users = STATE.with(|s| s.borrow().users.clone());
    let user;

    if let Some(i) = users.get(&principal_id) {
        user = i.clone();
    } else {
        return Err("this user does not exist".to_string());
    }

    let nonce: u64;
    if let Some(user_transactions) = user.transactions.get(&chain_id) {
        nonce = user_transactions.nonce;
    } else {
        nonce = 0;
    }

    let data = "0x".to_owned() + &utils::get_transfer_data(&address, value).unwrap();

    let tx = transaction::Transaction1559 {
        nonce,
        chain_id,
        max_priority_fee_per_gas,
        gas_limit,
        max_fee_per_gas,
        to: contract_address,
        value: 0,
        data,
        access_list: vec![],
        v: "0x00".to_string(),
        r: "0x00".to_string(),
        s: "0x00".to_string(),
    };

    let raw_tx = tx.serialize().unwrap();

    let res = sign(raw_tx, chain_id, principal_id).await.unwrap();

    Ok(TransferERC20Response { tx: res.sign_tx })
}

pub fn get_caller_data(principal_id: Principal, chain_id: u64) -> Result<CallerResponse, String> {
    let users = STATE.with(|s| s.borrow().users.clone());
    let user;
    if let Some(i) = users.get(&principal_id) {
        user = i.clone();
    } else {
        return Err("this user does not exist".to_string());
    }

    let address = get_address_from_public_key(user.public_key.clone()).unwrap();

    let transaction_data = user
        .transactions
        .get(&chain_id)
        .cloned()
        .unwrap_or_else(|| ChainData::default());

    Ok(CallerResponse {
        address,
        transactions: transaction_data,
    })
}

pub fn clear_caller_history(principal_id: Principal, chain_id: u64) -> Result<(), String> {
    let users = STATE.with(|s| s.borrow().users.clone());

    if let None = users.get(&principal_id) {
        return Err("this user does not exist".to_string());
    }

    STATE.with(|s| {
        let mut state = s.borrow_mut();
        let user = state.users.get_mut(&principal_id).unwrap();
        let user_tx = user.transactions.get_mut(&chain_id);
        if let Some(user_transactions) = user_tx {
            user_transactions.transactions.clear();
        }
    });

    Ok(())
}

pub fn pre_upgrade() {
    STATE.with(|s| {
        ic_cdk::storage::stable_save((s,)).unwrap();
    });
}

pub fn post_upgrade() {
    let (s_prev,): (State,) = ic_cdk::storage::stable_restore().unwrap();
    STATE.with(|s| {
        *s.borrow_mut() = s_prev;
    });
}

#[cfg(test)]
mod tests;
