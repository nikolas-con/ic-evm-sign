use ic_cdk::api::call::CallResult;
use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
    Principal,
};

use rlp::Encodable;

use std::vec::Vec;

use sha3::{Digest, Keccak256};

#[derive(CandidType, Serialize, Debug)]
pub struct PublicKeyReply {
    pub public_key: Vec<u8>,
}

#[derive(CandidType, Serialize, Debug)]
pub struct SignatureReply {
    pub sign_tx: Vec<u8>,
}

type CanisterId = Principal;

#[derive(CandidType, Serialize, Debug)]
struct ECDSAPublicKey {
    pub canister_id: Option<CanisterId>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: EcdsaKeyId,
}

#[derive(CandidType, Deserialize, Debug)]
struct ECDSAPublicKeyReply {
    pub public_key: Vec<u8>,
    pub chain_code: Vec<u8>,
}

#[derive(CandidType, Serialize, Debug)]
struct SignWithECDSA {
    pub message_hash: Vec<u8>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: EcdsaKeyId,
}

#[derive(CandidType, Deserialize, Debug)]
struct SignWithECDSAReply {
    pub signature: Vec<u8>,
}

#[derive(CandidType, Serialize, Debug, Clone)]
struct EcdsaKeyId {
    pub curve: EcdsaCurve,
    pub name: String,
}

#[derive(CandidType, Serialize, Debug, Clone)]
pub enum EcdsaCurve {
    #[serde(rename = "secp256k1")]
    Secp256k1,
}
use std::str::FromStr;
use std::vec;
pub async fn public_key(caller: Vec<u8>) -> CallResult<PublicKeyReply> {
    let key_id = EcdsaKeyId {
        curve: EcdsaCurve::Secp256k1,
        name: "dfx_test_key".to_string(),
    };
    let ic_canister_id = "aaaaa-aa";
    let ic = CanisterId::from_str(&ic_canister_id).unwrap();

    // let caller = ic_cdk::caller().as_slice().to_vec();
    let request = ECDSAPublicKey {
        canister_id: None,
        derivation_path: vec![caller],
        key_id: key_id.clone(),
    };
    let (res,): (ECDSAPublicKeyReply,) = ic_cdk::call(ic, "ecdsa_public_key", (request,))
        .await
        .map_err(|e| format!("Failed to call ecdsa_public_key {}", e.1))
        .unwrap();

    Ok(PublicKeyReply {
        public_key: res.public_key,
    })
}

pub async fn sign(hex_raw_tx: Vec<u8>, msg_hash: Vec<u8>) -> CallResult<SignatureReply> {
    assert!(msg_hash.len() == 32);

    let key_id = EcdsaKeyId {
        curve: EcdsaCurve::Secp256k1,
        name: "dfx_test_key".to_string(),
    };
    let ic_canister_id = "aaaaa-aa";
    let ic = CanisterId::from_str(&ic_canister_id).unwrap();

    let caller = ic_cdk::caller().as_slice().to_vec();

    let msg_hash1 = get_message_to_sign(hex_raw_tx.clone());

    let request = SignWithECDSA {
        message_hash: msg_hash1.clone(),
        derivation_path: vec![caller],
        key_id,
    };
    let (res,): (SignWithECDSAReply,) =
        ic_cdk::api::call::call_with_payment(ic, "sign_with_ecdsa", (request,), 10_000_000_000)
            .await
            .map_err(|e| format!("Failed to call sign_with_ecdsa {}", e.1))
            .unwrap();

    let signed_tx = sign_tx(res.signature, hex_raw_tx);

    Ok(SignatureReply { sign_tx: signed_tx })
}

fn get_message_to_sign(hex_raw_tx: Vec<u8>) -> Vec<u8> {
    let mut raw_tx = hex_raw_tx.clone();

    raw_tx[80] = u8::from(1);

    let mut hasher = Keccak256::new();

    hasher.update(&hex_raw_tx[..]);

    let result = hasher.finalize();

    result.to_vec()
}

fn sign_tx(signature: Vec<u8>, hex_raw_tx: Vec<u8>) -> Vec<u8> {
    let r = &signature[..32];
    let s = &signature[32..];
    let v = &[u8::from(37)];

    let removed_last = &hex_raw_tx[0..hex_raw_tx.len() - 3];

    let hex = [removed_last, v, &[u8::from(160)], r, &[u8::from(160)], s].concat();

    let msg_length = u8::try_from(hex[2..].len()).unwrap();

    [&hex[..1], &[msg_length], &hex[2..]].concat()
}
