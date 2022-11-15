use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
};
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

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct User {
    pub public_key: Vec<u8>,
    pub transactions: Vec<Transaction>,
}

impl Default for User {
    fn default() -> Self {
        User {
            public_key: vec![],
            transactions: vec![],
        }
    }
}
