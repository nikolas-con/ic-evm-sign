use crate::ecdsa::reply::{ECDSAPublicKeyReply, SignWithECDSAReply};
use crate::ecdsa::request::SignWithECDSA;
use crate::utils::generate_random_private_key;
use candid::de::IDLDeserialize;
use candid::utils::{ArgumentDecoder, ArgumentEncoder};
use candid::{Decode, Encode};
use ic_cdk::api::call::{CallResult, RejectionCode};
use ic_cdk::export::Principal;
use libsecp256k1::{PublicKey, SecretKey};
use std::cell::RefCell;
use std::future::Future;

struct State {
    private_key: SecretKey,
}

thread_local! {
    static STATE_TEST: RefCell<State> = RefCell::new(State { private_key: SecretKey::default() });
}

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
