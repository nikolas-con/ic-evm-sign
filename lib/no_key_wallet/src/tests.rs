use super::*;
use crate::utils::create_raw_tx_legacy;
use ic_cdk::export::Principal;

use futures::executor::block_on;
pub struct EVMLegacyTransaction {
    pub nonce: usize,
    pub gas_price: usize,
    pub gas_limit: usize,
    pub to: String,
    pub value: usize,
    pub data: String,
}

#[test]
fn create_new_user() {
    let text = "aaaaa-aa";
    let principal_id = Principal::from_text(text).unwrap();

    let res = block_on(create(principal_id)).unwrap();
    assert_eq!(42, res.address.len());
}
#[test]
fn sign_legacy_tx() {
    let text = "aaaaa-aa";
    let principal_id = Principal::from_text(text).unwrap();
    let tx_parm = EVMLegacyTransaction {
        nonce: 0,
        gas_price: 36935555629,
        gas_limit: 31272,
        to: "0x70997970C51812dc3A010C7d01b50e0d17dc79C8".to_string(),
        value: 1000000000000000000,
        data: "0x000000000000000000000000000000000000000000000000000000000000000000000000"
            .to_string(),
    };
    let raw_tx = create_raw_tx_legacy(tx_parm);

    let chain_id: usize = 1;

    let res0 = block_on(create(principal_id)).unwrap();

    let res = block_on(sign(raw_tx.clone(), chain_id, principal_id)).unwrap();

    let tx = transaction::get_transaction(&res.sign_tx, chain_id).unwrap();

    let signature = get_signature(&res.sign_tx);

    let msg = tx.get_message_to_sign().unwrap();

    let recovery_id = get_recovery_id(&res.sign_tx, chain_id);
    let address = recover_address(signature, recovery_id, msg);

    assert_eq!(res0.address, address)
}

fn get_signature(raw_tx: &Vec<u8>) -> Vec<u8> {
    let rlp = rlp::Rlp::new(&raw_tx[..]);

    let r = rlp.at(7).as_val::<Vec<u8>>();
    let s = rlp.at(8).as_val::<Vec<u8>>();

    let signature = [r, s].concat();

    signature
}

fn get_recovery_id(raw_tx: &Vec<u8>, chain_id: usize) -> u8 {
    let rlp = rlp::Rlp::new(&raw_tx[..]);
    let v = rlp.at(6).as_val::<Vec<u8>>();
    let recovery_id =
        -1 * ((i8::try_from(chain_id).unwrap() * 2) + 35 - i8::try_from(v[0]).unwrap());
    u8::try_from(recovery_id).unwrap()
}
fn recover_address(signature: Vec<u8>, recovery_id: u8, message: Vec<u8>) -> String {
    let signature_bytes: [u8; 64] = signature[..].try_into().unwrap();
    let signature_bytes_64 = libsecp256k1::Signature::parse_standard(&signature_bytes).unwrap();

    let recovery_id_byte =
        libsecp256k1::RecoveryId::parse(u8::try_from(recovery_id).unwrap()).unwrap();

    let message_bytes: [u8; 32] = message[..].try_into().unwrap();
    let message_bytes_32 = libsecp256k1::Message::parse(&message_bytes);

    let public_key =
        libsecp256k1::recover(&message_bytes_32, &signature_bytes_64, &recovery_id_byte).unwrap();

    compute_address(public_key.serialize_compressed().to_vec())
}
