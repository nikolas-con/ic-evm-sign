use super::*;
use crate::transaction::Sign;
use crate::utils::{recover_address, vec_u8_to_string};
use ic_cdk::export::Principal;

use futures::executor::block_on;

#[test]
fn create_new_user() {
    let text = "aaaaa-aa";
    let principal_id = Principal::from_text(text).unwrap();

    let res = block_on(create(principal_id)).unwrap();
    assert_eq!(res.address.len(), 42);
}
#[test]
fn sign_legacy_transaction() {
    let tx = transaction::TransactionLegacy {
        nonce: 0,
        gas_price: 0,
        gas_limit: 0,
        to: "0x0000000000000000000000000000000000000000".to_string(),
        value: 0,
        data: "0x00".to_string(),
        chain_id: 1,
        v: "0x00".to_string(),
        r: "0x00".to_string(),
        s: "0x00".to_string(),
    };
    assert_eq!(tx.is_signed(), false);

    assert_eq!(
        tx.get_signature(),
        Err("This is not  a signed transaction".to_string())
    );
    assert_eq!(
        tx.get_recovery_id(),
        Err("This is not  a signed transaction".to_string())
    );

    let text = "aaaaa-aa";
    let principal_id = Principal::from_text(text).unwrap();
    let res0 = block_on(create(principal_id)).unwrap();
    let raw_tx = tx.serialize().unwrap();
    let chain_id: u64 = 1;
    let res = block_on(sign(raw_tx.clone(), chain_id, principal_id)).unwrap();

    let tx_signed = transaction::get_transaction(&res.sign_tx, chain_id).unwrap();
    assert_eq!(tx_signed.is_signed(), true);

    let signature = tx_signed.get_signature().unwrap();
    assert_eq!(vec_u8_to_string(&signature), "c9e2682ec5084986365523c4268c5956c064c1ee85dc208364cb71e93edabab612ffab0eaed3e34865b225e9f349945599f8641cd806dc43029e0f92fdca23cb");
    let msg = tx_signed.get_message_to_sign().unwrap();
    assert_eq!(
        vec_u8_to_string(&msg),
        "eb86127620fbc047c6b6c2fcedea010143538e452dc7cb67a7fb1f8a00abdbd9"
    );
    let recovery_id = tx_signed.get_recovery_id().unwrap();
    assert_eq!(recovery_id, 0);
    let address = recover_address(signature, recovery_id, msg);
    assert_eq!(address, "0x907dc4d0be5d691970cae886fcab34ed65a2cd66");
    assert_eq!(res0.address, address)
}
#[test]
fn sign_eip2930_transaction() {
    let tx = transaction::Transaction2930 {
        chain_id: 1,
        nonce: 0,
        gas_price: 0,
        gas_limit: 0,
        to: "0x0000000000000000000000000000000000000000".to_string(),
        value: 0,
        data: "0x00".to_string(),
        access_list: vec![0xc0],
        v: "0x00".to_string(),
        r: "0x00".to_string(),
        s: "0x00".to_string(),
    };

    assert_eq!(tx.is_signed(), false);

    assert_eq!(
        tx.get_signature(),
        Err("This is not  a signed transaction".to_string())
    );
    assert_eq!(
        tx.get_recovery_id(),
        Err("This is not  a signed transaction".to_string())
    );

    let text = "aaaaa-aa";
    let principal_id = Principal::from_text(text).unwrap();
    let res0 = block_on(create(principal_id)).unwrap();

    let raw_tx = tx.serialize().unwrap();
    let chain_id: u64 = 1;
    let res = block_on(sign(raw_tx.clone(), chain_id, principal_id)).unwrap();

    let tx_signed = transaction::get_transaction(&res.sign_tx, chain_id).unwrap();
    assert_eq!(tx_signed.is_signed(), true);
    let signature = tx_signed.get_signature().unwrap();
    assert_eq!(vec_u8_to_string(&signature), "31cf08411809b04f8a82d2b07d6c33f7aa46d805e833f832464fd237c00a11d35104f49a601cf90fd5fe6297ec403959b7f649b5125ea3bcde084e9893fee5c6");

    let message = tx_signed.get_message_to_sign().unwrap();
    assert_eq!(
        vec_u8_to_string(&message),
        "1db9b0174e2b28a2073c88acbc792a5445407c5a8bf7bc5c65a047d45885eb89"
    );
    let recovery_id = tx_signed.get_recovery_id().unwrap();
    assert_eq!(recovery_id, 1);

    let address = recover_address(signature, recovery_id, message);
    assert_eq!(address, "0x907dc4d0be5d691970cae886fcab34ed65a2cd66");

    assert_eq!(res0.address, address)
}

#[test]
fn sign_eip1559_transaction() {
    let tx = transaction::Transaction1559 {
        chain_id: 1,
        nonce: 0,
        max_priority_fee_per_gas: 0,
        gas_limit: 0,
        max_fee_per_gas: 0,
        to: "0x0000000000000000000000000000000000000000".to_string(),
        value: 0,
        data: "0x00".to_string(),
        access_list: vec![0xc0],
        v: "0x00".to_string(),
        r: "0x00".to_string(),
        s: "0x00".to_string(),
    };
    assert_eq!(tx.is_signed(), false);

    assert_eq!(
        tx.get_signature(),
        Err("This is not  a signed transaction".to_string())
    );
    assert_eq!(
        tx.get_recovery_id(),
        Err("This is not  a signed transaction".to_string())
    );

    let text = "aaaaa-aa";
    let principal_id = Principal::from_text(text).unwrap();
    let res0 = block_on(create(principal_id)).unwrap();

    let raw_tx = tx.serialize().unwrap();
    let chain_id: u64 = 1;
    let res = block_on(sign(raw_tx, chain_id, principal_id)).unwrap();

    let tx_signed = transaction::get_transaction(&res.sign_tx, chain_id).unwrap();
    assert_eq!(tx_signed.is_signed(), true);

    let signature = tx_signed.get_signature().unwrap();
    assert_eq!(vec_u8_to_string(&signature), "29edd4e1d65e1b778b464112d2febc6e97bb677aba5034408fd27b49921beca94c4e5b904d58553bcd9c788360e0bd55c513922cf1f33a6386033e886cd4f77f");
    let message = tx_signed.get_message_to_sign().unwrap();
    assert_eq!(
        vec_u8_to_string(&message),
        "79965df63d7d9364f4bc8ed54ffd1c267042d4db673e129e3c459afbcb73a6f1"
    );
    let recovery_id = tx_signed.get_recovery_id().unwrap();
    assert_eq!(recovery_id, 0);
    let address = recover_address(signature, recovery_id, message);
    assert_eq!(address, "0x907dc4d0be5d691970cae886fcab34ed65a2cd66");

    assert_eq!(res0.address, address)
}
