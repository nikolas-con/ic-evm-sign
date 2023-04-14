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


#[derive(CandidType, Deserialize, Debug, Clone, PartialEq, Default)]
pub enum Environment {
    #[default] Development,
    Staging,
    Production,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct Config {
    pub env: Environment,
    pub key_name: String,
    pub sign_cycles: u64 
}

impl Default for Config {
    fn default() -> Self {
        Self::from(Environment::Development)
    }
}

impl From<Environment> for Config {
     fn from(env: Environment) -> Self {
        if env == Environment::Staging {
            Self {
                env: Environment::Staging,
                key_name: "test_key_1".to_string(),
                sign_cycles: 10_000_000_000
            }
        }  else if env == Environment::Production {
            Self {
                env: Environment::Production,
                key_name: "key_1".to_string(),
                sign_cycles: 26_153_846_153
            }
        } else {
            Self {
                env: Environment::Development,
                key_name: "dfx_test_key".to_string(),
                sign_cycles: 0
            }
        }
     }
}

#[derive(Default, CandidType, Deserialize, Debug, Clone)]
pub struct State {
    pub users: HashMap<Principal, UserData>,
    pub config: Config
}

thread_local! {
    pub static STATE: RefCell<State> = RefCell::new(State::default());
}
