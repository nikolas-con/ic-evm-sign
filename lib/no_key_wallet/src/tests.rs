use super::*;
use ic_cdk::export::Principal;

use futures::executor::block_on;

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
        248, 81, 10, 134, 9, 24, 78, 114, 160, 0, 130, 117, 48, 148, 112, 153, 121, 112, 197, 24,
        18, 220, 58, 1, 12, 125, 1, 181, 14, 13, 23, 220, 121, 200, 136, 13, 224, 182, 179, 167,
        100, 0, 0, 164, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 128, 128,
    ];
    let tx = EVMLegacyTransaction {
        nonce: 0,
        gas_price: 36935555629,
        gas_limit: 31272,
        to: "0x70997970C51812dc3A010C7d01b50e0d17dc79C8".to_string(),
        value: 1000000000000000000,
        data: "0x000000000000000000000000000000000000000000000000000000000000000000000000"
            .to_string(),
    };
    let chain_id: u8 = 1;

    let res0 = block_on(create(principal_id)).unwrap();

    let res = block_on(sign(raw_tx.clone(), chain_id, principal_id)).unwrap();

    let rlp = rlp::Rlp::new(&res.sign_tx[..]);

    let v = rlp.at(6).as_val::<Vec<u8>>();
    let r = rlp.at(7).as_val::<Vec<u8>>();
    let s = rlp.at(8).as_val::<Vec<u8>>();

    let signature = [r, s].concat();

    let signature_bytes: [u8; 64] = signature[..].try_into().unwrap();
    let signature_bytes_64 = libsecp256k1::Signature::parse_standard(&signature_bytes).unwrap();

    let recovery_id =
        -1 * ((i8::try_from(chain_id).unwrap() * 2) + 35 - i8::try_from(v[0]).unwrap());
    let recovery_id_byte =
        libsecp256k1::RecoveryId::parse(u8::try_from(recovery_id).unwrap()).unwrap();

    let msg = get_message_to_sign(raw_tx.clone(), &chain_id).unwrap();

    let message_bytes: [u8; 32] = msg[..].try_into().unwrap();
    let message_bytes_32 = libsecp256k1::Message::parse(&message_bytes);

    let key =
        libsecp256k1::recover(&message_bytes_32, &signature_bytes_64, &recovery_id_byte).unwrap();

    assert_eq!(
        res0.address,
        compute_address(key.serialize_compressed().to_vec())
    )
}
