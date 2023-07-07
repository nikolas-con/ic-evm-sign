use super::*;
use crate::transaction::Sign;
use crate::utils::{string_to_vec_u8, vec_u8_to_string};
use ic_cdk::export::Principal;
use futures::executor::block_on;

#[test]
fn create_new_user() {
    let principal_id = Principal::from_text("aaaaa-aa").unwrap();

    let res = block_on(create_address(principal_id)).unwrap();
    assert_eq!(res.address.len(), 42);
}

#[test]
fn sign_legacy_transaction() {
    let expected_get_signature_before = Err("This is not a signed transaction".to_string());
    let expected_get_signature_after ="c9e2682ec5084986365523c4268c5956c064c1ee85dc208364cb71e93edabab612ffab0eaed3e34865b225e9f349945599f8641cd806dc43029e0f92fdca23cb";
    let expected_get_recovery_id_before = Err("This is not a signed transaction".to_string());
    let expected_get_recovery_id_after = 0;
    let expected_get_message_to_sign_after = "eb86127620fbc047c6b6c2fcedea010143538e452dc7cb67a7fb1f8a00abdbd9";
    let expected_address = "0x907dc4d0be5d691970cae886fcab34ed65a2cd66";

    use primitive_types::U256;
    let tx = transaction::TransactionLegacy {
        nonce: 0,
        gas_price: U256::zero(),
        gas_limit: 0,
        to: "0x0000000000000000000000000000000000000000".to_string(),
        value: U256::zero(),
        data: "0x00".to_string(),
        chain_id: 1,
        v: "0x00".to_string(),
        r: "0x00".to_string(),
        s: "0x00".to_string(),
    };
    assert_eq!(tx.is_signed(), false);

    assert_eq!(tx.get_signature(), expected_get_signature_before);
    assert_eq!(tx.get_recovery_id(), expected_get_recovery_id_before);

    let text = "aaaaa-aa";
    let principal_id = Principal::from_text(text).unwrap();
    let res_create = block_on(create_address(principal_id)).unwrap();
    let raw_tx = tx.serialize().unwrap();
    let chain_id: u64 = 1;
    let res_sign = block_on(sign_transaction(raw_tx.clone(), chain_id, principal_id)).unwrap();

    let tx_signed = transaction::get_transaction(&res_sign.sign_tx, chain_id).unwrap();
    assert_eq!(tx_signed.is_signed(), true);

    let signature = tx_signed.get_signature().unwrap();
    assert_eq!(vec_u8_to_string(&signature), expected_get_signature_after);

    let msg = tx_signed.get_message_to_sign().unwrap();
    assert_eq!(vec_u8_to_string(&msg), expected_get_message_to_sign_after);

    let recovery_id = tx_signed.get_recovery_id().unwrap();
    assert_eq!(recovery_id, expected_get_recovery_id_after);

    let address = recover_address(signature, recovery_id, msg).unwrap();
    assert_eq!(address, expected_address);

    assert_eq!(res_create.address, address)
}
#[test]
fn sign_eip2930_transaction() {
    let expected_get_signature_before = Err("This is not a signed transaction".to_string());
    let expected_get_signature_after ="31cf08411809b04f8a82d2b07d6c33f7aa46d805e833f832464fd237c00a11d35104f49a601cf90fd5fe6297ec403959b7f649b5125ea3bcde084e9893fee5c6";
    let expected_get_recovery_id_before = Err("This is not a signed transaction".to_string());
    let expected_get_recovery_id_after = 1;
    let expected_get_message_to_sign_after = "1db9b0174e2b28a2073c88acbc792a5445407c5a8bf7bc5c65a047d45885eb89";
    let expected_address = "0x907dc4d0be5d691970cae886fcab34ed65a2cd66";

    use primitive_types::U256;
    let tx = transaction::Transaction2930 {
        chain_id: 1,
        nonce: 0,
        gas_price: U256::zero(),
        gas_limit: 0,
        to: "0x0000000000000000000000000000000000000000".to_string(),
        value: U256::zero(),
        data: "0x00".to_string(),
        access_list: vec![],
        v: "0x00".to_string(),
        r: "0x00".to_string(),
        s: "0x00".to_string(),
    };

    assert_eq!(tx.is_signed(), false);

    assert_eq!(tx.get_signature(), expected_get_signature_before);
    assert_eq!(tx.get_recovery_id(), expected_get_recovery_id_before);

    let principal_id = Principal::from_text("aaaaa-aa").unwrap();
    let res_create = block_on(create_address(principal_id)).unwrap();

    let raw_tx = tx.serialize().unwrap();
    let chain_id: u64 = 1;
    let res_sign = block_on(sign_transaction(raw_tx.clone(), chain_id, principal_id)).unwrap();

    let tx_signed = transaction::get_transaction(&res_sign.sign_tx, chain_id).unwrap();
    assert_eq!(tx_signed.is_signed(), true);
    let signature = tx_signed.get_signature().unwrap();
    assert_eq!(vec_u8_to_string(&signature), expected_get_signature_after);

    let message = tx_signed.get_message_to_sign().unwrap();
    assert_eq!(vec_u8_to_string(&message), expected_get_message_to_sign_after);
    let recovery_id = tx_signed.get_recovery_id().unwrap();
    assert_eq!(recovery_id, expected_get_recovery_id_after);

    let address = recover_address(signature, recovery_id, message).unwrap();
    assert_eq!(address, expected_address);

    assert_eq!(res_create.address, address)
}

#[test]
fn sign_eip1559_transaction() {
    let expected_get_signature_before = Err("This is not a signed transaction".to_string());
    let expected_get_signature_after ="29edd4e1d65e1b778b464112d2febc6e97bb677aba5034408fd27b49921beca94c4e5b904d58553bcd9c788360e0bd55c513922cf1f33a6386033e886cd4f77f";
    let expected_get_recovery_id_before = Err("This is not a signed transaction".to_string());
    let expected_get_recovery_id_after = 0;
    let expected_get_message_to_sign_after = "79965df63d7d9364f4bc8ed54ffd1c267042d4db673e129e3c459afbcb73a6f1";
    let expected_address = "0x907dc4d0be5d691970cae886fcab34ed65a2cd66";

    use primitive_types::U256;
    let tx = transaction::Transaction1559 {
        chain_id: 1,
        nonce: 0,
        max_priority_fee_per_gas: U256::zero(),
        gas_limit: 0,
        max_fee_per_gas: U256::zero(),
        to: "0x0000000000000000000000000000000000000000".to_string(),
        value: U256::zero(),
        data: "0x00".to_string(),
        access_list: vec![],
        v: "0x00".to_string(),
        r: "0x00".to_string(),
        s: "0x00".to_string(),
    };
    assert_eq!(tx.is_signed(), false);

    assert_eq!(tx.get_signature(), expected_get_signature_before);
    assert_eq!(tx.get_recovery_id(), expected_get_recovery_id_before);

    let text = "aaaaa-aa";
    let principal_id = Principal::from_text(text).unwrap();
    let res_create = block_on(create_address(principal_id)).unwrap();

    let raw_tx = tx.serialize().unwrap();
    let chain_id: u64 = 1;
    let res = block_on(sign_transaction(raw_tx, chain_id, principal_id)).unwrap();

    let tx_signed = transaction::get_transaction(&res.sign_tx, chain_id).unwrap();
    assert_eq!(tx_signed.is_signed(), true);

    let signature = tx_signed.get_signature().unwrap();
    assert_eq!(vec_u8_to_string(&signature), expected_get_signature_after);

    let message = tx_signed.get_message_to_sign().unwrap();
    assert_eq!(vec_u8_to_string(&message), expected_get_message_to_sign_after);

    let recovery_id = tx_signed.get_recovery_id().unwrap();
    assert_eq!(recovery_id, expected_get_recovery_id_after);

    let address = recover_address(signature, recovery_id, message).unwrap();
    assert_eq!(address, expected_address);

    assert_eq!(res_create.address, address)
}

#[test]
fn recover_address_valid() {
    let expected = "0x907dc4d0be5d691970cae886fcab34ed65a2cd66";

    let signature =string_to_vec_u8("29edd4e1d65e1b778b464112d2febc6e97bb677aba5034408fd27b49921beca94c4e5b904d58553bcd9c788360e0bd55c513922cf1f33a6386033e886cd4f77f");
    let recovery_id = 0;
    let message = string_to_vec_u8("79965df63d7d9364f4bc8ed54ffd1c267042d4db673e129e3c459afbcb73a6f1");
    let address = recover_address(signature, recovery_id, message).unwrap();

    assert_eq!(address, expected);
}

#[test]
fn recover_address_with_invalid_signature() {
    let expected = Err("Invalid signature".to_string());

    let signature = string_to_vec_u8("");
    let recovery_id = 0;
    let message = string_to_vec_u8("79965df63d7d9364f4bc8ed54ffd1c267042d4db673e129e3c459afbcb73a6f1");
    let result = recover_address(signature, recovery_id, message);

    assert_eq!(result, expected);
}

#[test]
fn recover_address_with_invalid_message() {
    let expected = Err("Invalid message".to_string());

    let signature = string_to_vec_u8("29edd4e1d65e1b778b464112d2febc6e97bb677aba5034408fd27b49921beca94c4e5b904d58553bcd9c788360e0bd55c513922cf1f33a6386033e886cd4f77f");
    let recovery_id = 0;
    let message = string_to_vec_u8("");
    let result = recover_address(signature, recovery_id, message);

    assert_eq!(result, expected);
}

fn recover_address(
    signature: Vec<u8>,
    recovery_id: u8,
    message: Vec<u8>,
) -> Result<String, String> {
    if signature.len() != 64 {
        return Err("Invalid signature".to_string());
    }
    if message.len() != 32 {
        return Err("Invalid message".to_string());
    }

    let signature_bytes: [u8; 64] = signature[..].try_into().unwrap();
    let signature_bytes_64 = libsecp256k1::Signature::parse_standard(&signature_bytes).unwrap();

    let recovery_id_byte = libsecp256k1::RecoveryId::parse(u8::try_from(recovery_id).unwrap()).unwrap();

    let message_bytes: [u8; 32] = message[..].try_into().unwrap();
    let message_bytes_32 = libsecp256k1::Message::parse(&message_bytes);

    let public_key = libsecp256k1::recover(&message_bytes_32, &signature_bytes_64, &recovery_id_byte).unwrap();

    let address = get_address_from_public_key(public_key.serialize_compressed().to_vec()).unwrap();

    Ok(address)
}
