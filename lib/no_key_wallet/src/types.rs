use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
    Principal,
};

type CanisterId = Principal;

pub mod response {
    use super::*;
    use state::Transaction;

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
}

pub mod reply {
    use super::*;

    #[derive(CandidType, Serialize, Debug)]
    pub struct PublicKeyReply {
        pub public_key: Vec<u8>,
    }

    #[derive(CandidType, Deserialize, Debug)]
    pub struct ECDSAPublicKeyReply {
        pub public_key: Vec<u8>,
        pub chain_code: Vec<u8>,
    }

    #[derive(CandidType, Deserialize, Debug)]
    pub struct SignWithECDSAReply {
        pub signature: Vec<u8>,
    }
}
pub mod request {
    use super::*;
    #[derive(CandidType, Serialize, Debug)]
    pub struct ECDSAPublicKey {
        pub canister_id: Option<CanisterId>,
        pub derivation_path: Vec<Vec<u8>>,
        pub key_id: EcdsaKeyId,
    }
    #[derive(CandidType, Serialize, Debug, Deserialize)]
    pub struct SignWithECDSA {
        pub message_hash: Vec<u8>,
        pub derivation_path: Vec<Vec<u8>>,
        pub key_id: EcdsaKeyId,
    }
    #[derive(CandidType, Serialize, Debug, Clone, Deserialize)]
    pub struct EcdsaKeyId {
        pub curve: EcdsaCurve,
        pub name: String,
    }

    #[derive(CandidType, Serialize, Debug, Clone, Deserialize)]
    pub enum EcdsaCurve {
        #[serde(rename = "secp256k1")]
        Secp256k1,
    }
}

pub mod state {
    use super::*;
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
}
