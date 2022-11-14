#[cfg(not(test))]
use ic_cdk::api::time as ic_timestamp;
#[cfg(not(test))]
use ic_cdk::call as ic_call;
use ic_cdk::export::{candid::CandidType, serde::Deserialize, Principal};
#[cfg(test)]
mod mocks;
#[cfg(test)]
use mocks::{ic_call, ic_timestamp};
pub mod utils;
use rlp;
use utils::{compute_address, get_derivation_path, get_rec_id};

use easy_hasher::easy_hasher;

use std::cell::RefCell;

use std::collections::HashMap;

pub mod types;

use types::reply::*;
use types::request::*;
use types::response::*;
use types::state::*;

#[derive(Default, CandidType, Deserialize, Debug)]
pub struct State {
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
    chain_id: u8,
    principal_id: Principal,
) -> Result<SignResponse, String> {
    let users = STATE.with(|s| s.borrow().users.clone());
    let user;

    if let Some(i) = users.get(&principal_id) {
        user = i.clone();
    } else {
        return Err("this user does not exist".to_string());
    }

    // todo
    let message = get_message_to_sign(hex_raw_tx.clone(), &chain_id).unwrap();

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

    // todo
    let signed_tx = sign_tx(res.signature.clone(), hex_raw_tx, chain_id, rec_id).unwrap();

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

// utilities functions

pub fn get_message_to_sign(hex_raw_tx: Vec<u8>, chain_id: &u8) -> Result<Vec<u8>, String> {
    let tx_type = get_transaction_type(&hex_raw_tx).unwrap();
    if tx_type == TransactionType::Legacy {
        let rlp = rlp::Rlp::new(&hex_raw_tx[..]);

        let mut stream = rlp::RlpStream::new_list(9);
        for i in 0..=8 {
            let bytes: Vec<u8>;
            if i == 6 {
                bytes = vec![chain_id.clone()];
            } else {
                bytes = rlp.at(i).as_val::<Vec<u8>>();
            }
            stream.append(&bytes);
        }
        let encoded_tx = stream.out();

        let keccak256 = easy_hasher::raw_keccak256(encoded_tx);

        Ok(keccak256.to_vec())
    } else if tx_type == TransactionType::EIP1559 {
        let rlp = rlp::Rlp::new(&hex_raw_tx[1..]);

        let mut stream = rlp::RlpStream::new_list(9);
        for i in 0..=8 {
            if i == 8 {
                let item = rlp.at(i);
                let raw = item.as_raw();
                let item_count: usize = 1;
                stream.append_raw(raw, item_count);
            } else {
                let item = rlp.at(i).as_val::<Vec<u8>>();
                stream.append(&item);
            }
        }

        let decode_tx = stream.out();

        let msg = [&hex_raw_tx[..1], &decode_tx[..]].concat();
        let keccak256 = easy_hasher::raw_keccak256(msg);
        Ok(keccak256.to_vec())
    } else if tx_type == TransactionType::EPI2930 {
        let rlp = rlp::Rlp::new(&hex_raw_tx[1..]);

        let mut stream = rlp::RlpStream::new_list(8);

        for i in 0..=7 {
            if i == 7 {
                let item = rlp.at(i);
                let raw = item.as_raw();
                let item_count: usize = 1;
                stream.append_raw(raw, item_count);
            } else {
                let item = rlp.at(i).as_val::<Vec<u8>>();
                stream.append(&item);
            }
        }
        let decode_tx = stream.out();

        let msg = [&hex_raw_tx[..1], &decode_tx[..]].concat();
        let keccak256 = easy_hasher::raw_keccak256(msg);
        Ok(keccak256.to_vec())
    } else {
        Err(String::from("something went wrong get_message_to_sign"))
    }
}

fn get_transaction_type(hex_raw_tx: &Vec<u8>) -> Result<TransactionType, String> {
    if hex_raw_tx[0] == 0xf8 {
        return Ok(TransactionType::Legacy);
    } else if hex_raw_tx[0] == 0x01 {
        return Ok(TransactionType::EPI2930);
    } else if hex_raw_tx[0] == 0x02 {
        return Ok(TransactionType::EIP1559);
    } else {
        return Err(String::from("Invalid type"));
    }
}

fn sign_tx(
    signature: Vec<u8>,
    hex_raw_tx: Vec<u8>,
    chain_id: u8,
    rec_id: usize,
) -> Result<Vec<u8>, String> {
    let tx_type = get_transaction_type(&hex_raw_tx).unwrap();
    if tx_type == TransactionType::Legacy {
        let r = &signature[..32];
        let s = &signature[32..];
        let v = u8::try_from(chain_id * 2 + 35 + u8::try_from(rec_id).unwrap()).unwrap();

        let rlp = rlp::Rlp::new(&hex_raw_tx[..]);

        let mut stream = rlp::RlpStream::new_list(9);
        for i in 0..=8 {
            let bytes: Vec<u8>;
            if i == 6 {
                bytes = vec![v];
            } else if i == 7 {
                bytes = r.to_vec();
            } else if i == 8 {
                bytes = s.to_vec();
            } else {
                bytes = rlp.at(i).as_val::<Vec<u8>>();
            }
            stream.append(&bytes);
        }

        Ok(stream.out())
    } else if tx_type == TransactionType::EIP1559 {
        let r = &signature[..32];
        let s = &signature[32..];
        let rlp = rlp::Rlp::new(&hex_raw_tx[1..]);
        let mut stream = rlp::RlpStream::new_list(12);

        for i in 0..12 {
            if i == 8 {
                let val = rlp.at(i).as_raw();

                stream.append_raw(&val, 1);
            } else if i == 9 {
                if rec_id == 0 {
                    stream.append_empty_data();
                } else {
                    let v = vec![0x01];
                    stream.append(&v);
                }
            } else if i == 10 {
                stream.append(&r);
            } else if i == 11 {
                stream.append(&s);
            } else {
                let bytes = rlp.at(i).as_val::<Vec<u8>>();

                stream.append(&bytes);
            }
        }
        Ok([&hex_raw_tx[..1], &stream.out()].concat())
    } else if tx_type == TransactionType::EPI2930 {
        let r = &signature[..32];
        let s = &signature[32..];
        let rlp = rlp::Rlp::new(&hex_raw_tx[1..]);
        let mut stream = rlp::RlpStream::new_list(11);

        for i in 0..11 {
            if i == 7 {
                let val = rlp.at(i).as_raw();

                stream.append_raw(&val, 1);
            } else if i == 8 {
                if rec_id == 0 {
                    stream.append_empty_data();
                } else {
                    let v = vec![0x01];
                    stream.append(&v);
                }
            } else if i == 9 {
                stream.append(&r);
            } else if i == 10 {
                stream.append(&s);
            } else {
                let bytes = rlp.at(i).as_val::<Vec<u8>>();

                stream.append(&bytes);
            }
        }
        Ok([&hex_raw_tx[..1], &stream.out()].concat())
    } else {
        Err("Not valid TransactionType".to_string())
    }
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
