use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
    Principal,
};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(CandidType, Serialize, Debug, Clone, Deserialize)]
pub struct Transaction {
    pub data: Vec<u8>,
    pub timestamp: u64,
}

impl Default for Transaction {
    fn default() -> Self {
        Transaction {
            data: vec![],
            timestamp: u64::from(0 as u64),
        }
    }
}

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct TransactionChainData {
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}

impl Default for TransactionChainData {
    fn default() -> Self {
        TransactionChainData {
            nonce: 0 as u64,
            transactions: vec![],
        }
    }
}
#[derive(Clone, Debug, CandidType, Deserialize, Default)]
pub struct UserData {
    pub public_key: Vec<u8>,
    pub transactions: HashMap<u64, TransactionChainData>,
}

#[derive(Default, CandidType, Deserialize, Debug)]
pub struct State {
    pub users: HashMap<Principal, UserData>,
}

thread_local! {
    pub static STATE: RefCell<State> = RefCell::new(State::default());
}
