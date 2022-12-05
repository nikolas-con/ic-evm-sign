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
pub struct ChainData {
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}
impl Default for ChainData {
    fn default() -> Self {
        ChainData {
            nonce: 0 as u64,
            transactions: vec![],
        }
    }
}

pub type ChainId = u64;
#[derive(Clone, Debug, CandidType, Deserialize, Default)]
pub struct User {
    pub public_key: Vec<u8>,
    pub transactions: HashMap<ChainId, ChainData>,
}

#[derive(Default, CandidType, Deserialize, Debug)]
pub struct State {
    pub users: HashMap<Principal, User>,
}

thread_local! {
    pub static STATE: RefCell<State> = RefCell::new(State::default());
}
