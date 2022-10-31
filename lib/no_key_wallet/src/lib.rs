use crate::rlp::RlpStream;
use ic_cdk::api::call::CallResult;
use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
    Principal,
};
use rlp;

use easy_hasher::easy_hasher;

#[derive(CandidType, Serialize, Debug)]
pub struct PublicKeyReply {
    pub public_key: Vec<u8>,
}

#[derive(CandidType, Serialize, Debug)]
pub struct CreateResponse {
    pub public_key: Vec<u8>,
    pub address: String,
}
#[derive(CandidType, Deserialize, Debug)]
pub struct SignResponse {
    pub sign_tx: Vec<u8>,
}

type CanisterId = Principal;

#[derive(CandidType, Serialize, Debug)]
struct ECDSAPublicKey {
    pub canister_id: Option<CanisterId>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: EcdsaKeyId,
}

#[derive(CandidType, Serialize, Debug)]
struct SignWithECDSA {
    pub message_hash: Vec<u8>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: EcdsaKeyId,
}
#[derive(CandidType, Deserialize, Debug)]
struct ECDSAPublicKeyReply {
    pub public_key: Vec<u8>,
    pub chain_code: Vec<u8>,
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

pub async fn create() -> Result<CreateResponse, String> {
    let key_id = EcdsaKeyId {
        curve: EcdsaCurve::Secp256k1,
        name: "dfx_test_key".to_string(),
    };

    let request = ECDSAPublicKey {
        canister_id: None,
        derivation_path: vec![vec![0, 0, 0, 0]],
        key_id: key_id.clone(),
    };
    let (res,): (ECDSAPublicKeyReply,) = ic_cdk::call(
        Principal::management_canister(),
        "ecdsa_public_key",
        (request,),
    )
    .await
    .map_err(|e| format!("Failed to call ecdsa_public_key {}", e.1))?;

    let pub_key_arr: [u8; 33] = res.public_key[..].try_into().unwrap();
    let pub_key = libsecp256k1::PublicKey::parse_compressed(&pub_key_arr)
        .unwrap()
        .serialize();

    let keccak256 = easy_hasher::raw_keccak256(pub_key[1..].to_vec());
    let keccak256_hex = keccak256.to_hex_string();
    let address: String = "0x".to_owned() + &keccak256_hex[24..];

    Ok(CreateResponse {
        address,
        public_key: res.public_key,
    })
}

pub async fn sign(hex_raw_tx: Vec<u8>, public_key: Vec<u8>) -> Result<SignResponse, String> {
    let message = get_message_to_sign(hex_raw_tx.clone());

    assert!(message.len() == 32);

    let key_id = EcdsaKeyId {
        curve: EcdsaCurve::Secp256k1,
        name: "dfx_test_key".to_string(),
    };

    let request = SignWithECDSA {
        message_hash: message.clone(),
        derivation_path: vec![vec![0, 0, 0, 0]],
        key_id: key_id.clone(),
    };
    let (res,): (SignWithECDSAReply,) = ic_cdk::api::call::call(
        Principal::management_canister(),
        "sign_with_ecdsa",
        (request,),
    )
    .await
    .map_err(|e| format!("Failed to call sign_with_ecdsa {}", e.1))?;

    let rec_id = get_rec_id(&message, &res.signature, &public_key).unwrap();

    let signed_tx = sign_tx(res.signature.clone(), hex_raw_tx, rec_id);

    Ok(SignResponse { sign_tx: signed_tx })
}
fn get_message_to_sign(hex_raw_tx: Vec<u8>) -> Vec<u8> {
    let mut raw_tx = hex_raw_tx.clone();

    raw_tx.insert(0, 0x83);

    let mut decoded_tx = decode_tx(raw_tx.clone());

    decoded_tx[0] = vec![];

    decoded_tx[6] = vec![u8::from(1)];

    let encoded_tx = encode_tx(decoded_tx);

    hash_tx(&encoded_tx)
}

fn get_rec_id(
    message: &Vec<u8>,
    signature: &Vec<u8>,
    public_key: &Vec<u8>,
) -> Result<usize, String> {
    for i in 0..3 {
        let recovery_id = libsecp256k1::RecoveryId::parse_rpc(27 + i).unwrap();

        let signature_bytes: [u8; 64] = signature[..].try_into().unwrap();
        let signature_bytes_64 = libsecp256k1::Signature::parse_standard(&signature_bytes).unwrap();

        let message_bytes: [u8; 32] = message[..].try_into().unwrap();
        let message_bytes_32 = libsecp256k1::Message::parse(&message_bytes);

        let key =
            libsecp256k1::recover(&message_bytes_32, &signature_bytes_64, &recovery_id).unwrap();
        if key.serialize_compressed() == public_key[..] {
            return Ok(i as usize);
        }
    }
    return Err("Not found".to_string());
}

fn decode_tx(hex_raw_tx: Vec<u8>) -> Vec<Vec<u8>> {
    let mut index = 0;
    let data_len = hex_raw_tx.len();
    let mut decode_tx: Vec<Vec<u8>> = vec![];

    while index < data_len {
        let decode_data: Vec<u8> = rlp::decode(&hex_raw_tx[index..]);
        index = index + decode_data.len() + 1;
        decode_tx.push(decode_data);
    }

    decode_tx
}
fn encode_tx(decoded_txt: Vec<Vec<u8>>) -> Vec<u8> {
    let mut stream = RlpStream::new_list(decoded_txt.len());

    for chucks in decoded_txt {
        stream.append(&chucks);
    }

    let out = stream.out();

    out
}

fn hash_tx(hex_raw_tx: &Vec<u8>) -> Vec<u8> {
    let keccak256 = easy_hasher::raw_keccak256(hex_raw_tx[..].to_vec());

    keccak256.to_vec()
}

fn sign_tx(signature: Vec<u8>, hex_raw_tx: Vec<u8>, rec_id: usize) -> Vec<u8> {
    let r = &signature[..32];
    let s = &signature[32..];
    let v = &[u8::try_from(37 + rec_id).unwrap()];

    let removed_last = &hex_raw_tx[0..hex_raw_tx.len() - 3];

    let hex = [removed_last, v, &[u8::from(160)], r, &[u8::from(160)], s].concat();

    let msg_length = u8::try_from(hex[2..].len()).unwrap();

    [&hex[..1], &[msg_length], &hex[2..]].concat()
}
