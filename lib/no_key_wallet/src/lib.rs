use crate::rlp::RlpStream;
#[cfg(test)]
use crate::tests::{ic_call, ic_timestamp};
#[cfg(not(test))]
use ic_cdk::api::time as ic_timestamp;
#[cfg(not(test))]
use ic_cdk::call as ic_call;
use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
    Principal,
};
use rlp;

use easy_hasher::easy_hasher;

use std::cell::RefCell;

use libsecp256k1;
use std::collections::HashMap;

#[derive(CandidType, Serialize, Debug)]
pub struct PublicKeyReply {
    pub public_key: Vec<u8>,
}

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

type CanisterId = Principal;

#[derive(CandidType, Serialize, Debug)]
struct ECDSAPublicKey {
    pub canister_id: Option<CanisterId>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: EcdsaKeyId,
}

#[derive(CandidType, Serialize, Debug, Deserialize)]
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

#[derive(CandidType, Serialize, Debug, Clone, Deserialize)]
struct EcdsaKeyId {
    pub curve: EcdsaCurve,
    pub name: String,
}

#[derive(CandidType, Serialize, Debug, Clone, Deserialize)]
pub enum EcdsaCurve {
    #[serde(rename = "secp256k1")]
    Secp256k1,
}
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

    let message = get_message_to_sign(hex_raw_tx.clone(), &chain_id);

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

    let signed_tx = sign_tx(res.signature.clone(), hex_raw_tx, chain_id, rec_id);

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
fn get_derivation_path(caller: Principal) -> Vec<u8> {
    caller.as_slice().to_vec()
}

fn get_message_to_sign(hex_raw_tx: Vec<u8>, chain_id: &u8) -> Vec<u8> {
    let mut raw_tx = hex_raw_tx.clone();

    raw_tx.insert(0, 0x83);

    let mut decoded_tx = decode_tx(raw_tx.clone());

    decoded_tx[0] = match decoded_tx[0][decoded_tx[0].len() - 1] == 128 {
        true => vec![],
        false => vec![decoded_tx[0][decoded_tx[0].len() - 1]],
    };

    decoded_tx[6] = vec![u8::from(chain_id.clone())];

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
        if decode_data.len() == 1 {
            index = index + decode_data.len();
        } else {
            index = index + decode_data.len() + 1;
        }
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

fn sign_tx(signature: Vec<u8>, hex_raw_tx: Vec<u8>, chain_id: u8, rec_id: usize) -> Vec<u8> {
    let r = &signature[..32];
    let s = &signature[32..];
    let v = &[u8::try_from(chain_id * 2 + 35 + u8::try_from(rec_id).unwrap()).unwrap()];

    let removed_last = &hex_raw_tx[0..hex_raw_tx.len() - 3];

    let hex = [removed_last, v, &[u8::from(160)], r, &[u8::from(160)], s].concat();

    let msg_length = u8::try_from(hex[2..].len()).unwrap();

    [&hex[..1], &[msg_length], &hex[2..]].concat()
}

fn compute_address(public_key: Vec<u8>) -> String {
    let pub_key_arr: [u8; 33] = public_key[..].try_into().unwrap();
    let pub_key = libsecp256k1::PublicKey::parse_compressed(&pub_key_arr)
        .unwrap()
        .serialize();

    let keccak256 = easy_hasher::raw_keccak256(pub_key[1..].to_vec());
    let keccak256_hex = keccak256.to_hex_string();
    let address: String = "0x".to_owned() + &keccak256_hex[24..];

    address
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
mod tests {

    use super::*;
    use candid::de::IDLDeserialize;
    use candid::utils::{ArgumentDecoder, ArgumentEncoder};
    use candid::{Decode, Encode};
    use ic_cdk::api::call::{CallResult, RejectionCode};
    use libsecp256k1::{PublicKey, SecretKey};
    use rand::{thread_rng, RngCore};
    use std::future::Future;

    pub struct State {
        private_key: SecretKey,
    }

    thread_local! {
        static STATE_TEST: RefCell<State> = RefCell::new(State { private_key: SecretKey::default() });
    }
    use futures::executor::block_on;

    pub fn ic_timestamp() -> u64 {
        u64::from(1667817318 as u64)
    }

    pub fn ic_call<T: ArgumentEncoder, R: for<'a> ArgumentDecoder<'a>>(
        _id: Principal,
        method: &str,
        args: T,
    ) -> impl Future<Output = CallResult<R>> + '_ {
        let args_raw = candid::encode_args(args).expect("Failed to encode arguments.");

        async move {
            if method == "ecdsa_public_key" {
                let private_key = generate_random_private_key();

                STATE_TEST.with(|s| {
                    let mut state = s.borrow_mut();

                    state.private_key = private_key;
                });

                let public_key = PublicKey::from_secret_key(&private_key).serialize_compressed();

                let obj = ECDSAPublicKeyReply {
                    public_key: public_key.to_vec(),
                    chain_code: vec![0, 1],
                };

                let bytes = Encode!(&obj).unwrap();
                let mut de = IDLDeserialize::new(&bytes).unwrap();
                let res: R = ArgumentDecoder::decode(&mut de).unwrap();
                return Ok(res);
            }
            if method == "sign_with_ecdsa" {
                let private_key = STATE_TEST.with(|s| s.borrow().private_key);
                let args = Decode!(&args_raw, SignWithECDSA).unwrap();

                let msg: [u8; 32] = args.message_hash[..32].try_into().unwrap();

                let message = libsecp256k1::Message::parse(&msg);

                let signature = libsecp256k1::sign(&message, &private_key);

                let obj = SignWithECDSAReply {
                    signature: signature.0.serialize().to_vec(),
                };
                let bytes = Encode!(&obj).unwrap();
                let mut de = IDLDeserialize::new(&bytes).unwrap();
                let res: R = ArgumentDecoder::decode(&mut de).unwrap();
                return Ok(res);
            } else {
                return Err((RejectionCode::CanisterReject, String::from("no method")));
            }
        }
    }

    fn generate_random_private_key() -> SecretKey {
        let mut rng = thread_rng();

        loop {
            let mut ret = [0u8; 32];
            rng.fill_bytes(&mut ret);
            if let Ok(key) = SecretKey::parse(&ret) {
                return key;
            }
        }
    }
    #[test]
    fn create_new_user() {
        let text = "aaaaa-aa";
        let principal_id = Principal::from_text(text).unwrap();

        let res = block_on(create(principal_id)).unwrap();
        assert_eq!(42, res.address.len());
    }
    #[test]
    fn sign_tx() {
        let text = "aaaaa-aa";
        let principal_id = Principal::from_text(text).unwrap();
        let raw_tx = vec![
            248, 81, 10, 134, 9, 24, 78, 114, 160, 0, 130, 117, 48, 148, 112, 153, 121, 112, 197,
            24, 18, 220, 58, 1, 12, 125, 1, 181, 14, 13, 23, 220, 121, 200, 136, 13, 224, 182, 179,
            167, 100, 0, 0, 164, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 128, 128,
        ];
        let chain_id: u8 = 1;

        let res0 = block_on(create(principal_id)).unwrap();
        println!("{:?}", res0.address);

        let res = block_on(sign(raw_tx.clone(), chain_id, principal_id)).unwrap();

        let mut hex_raw_tx: Vec<u8> = res.sign_tx.clone();
        hex_raw_tx.insert(0, 0x83);

        let decoded_tx = decode_tx(hex_raw_tx);

        let v = decoded_tx[6].clone();
        let r = decoded_tx[7].clone();
        let s = decoded_tx[8].clone();

        let signature = [r, s].concat();

        let signature_bytes: [u8; 64] = signature[..].try_into().unwrap();
        let signature_bytes_64 = libsecp256k1::Signature::parse_standard(&signature_bytes).unwrap();

        let recovery_id =
            -1 * ((i8::try_from(chain_id).unwrap() * 2) + 35 - i8::try_from(v[0]).unwrap());
        let recovery_id_byte =
            libsecp256k1::RecoveryId::parse(u8::try_from(recovery_id).unwrap()).unwrap();

        let msg = get_message_to_sign(raw_tx.clone(), &chain_id);

        let message_bytes: [u8; 32] = msg[..].try_into().unwrap();
        let message_bytes_32 = libsecp256k1::Message::parse(&message_bytes);

        let key = libsecp256k1::recover(&message_bytes_32, &signature_bytes_64, &recovery_id_byte)
            .unwrap();

        assert_eq!(
            res0.address,
            compute_address(key.serialize_compressed().to_vec())
        )
    }
}
