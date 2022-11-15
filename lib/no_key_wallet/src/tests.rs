use super::*;
use crate::utils::{
    create_raw_eip_2930_tx, create_raw_legacy_tx, create_raw_tx_1559, recover_address,
};
use ic_cdk::export::Principal;

use futures::executor::block_on;
pub struct EVMTransactionLegacy {
    pub nonce: usize,
    pub gas_price: usize,
    pub gas_limit: usize,
    pub to: String,
    pub value: usize,
    pub data: String,
}
pub struct EVMTransactionEIP2930 {
    pub chain_id: usize,
    pub nonce: usize,
    pub gas_price: usize,
    pub gas_limit: usize,
    pub to: String,
    pub value: usize,
    pub data: String,
    pub access_list: Vec<u8>,
}
pub struct EVMTransactionEIP1559 {
    pub chain_id: usize,
    pub nonce: usize,
    pub max_priority_fee_per_gas: usize,
    pub gas_limit: usize,
    pub max_fee_per_gas: usize,
    pub to: String,
    pub value: usize,
    pub data: String,
    pub access_list: Vec<u8>,
}

#[test]
fn create_new_user() {
    let text = "aaaaa-aa";
    let principal_id = Principal::from_text(text).unwrap();

    let res = block_on(create(principal_id)).unwrap();
    assert_eq!(42, res.address.len());
}
#[test]
fn sign_legacy_transaction() {
    let text = "aaaaa-aa";
    let principal_id = Principal::from_text(text).unwrap();
    let tx_parm = EVMTransactionLegacy {
        nonce: 0,
        gas_price: 36935555629,
        gas_limit: 31272,
        to: "0x70997970C51812dc3A010C7d01b50e0d17dc79C8".to_string(),
        value: 1000000000000000000,
        data: "0x00".to_string(),
    };
    let raw_tx = create_raw_legacy_tx(tx_parm);

    let chain_id: usize = 1;

    let res0 = block_on(create(principal_id)).unwrap();

    let res = block_on(sign(raw_tx.clone(), chain_id, principal_id)).unwrap();

    let tx = transaction::get_transaction(&res.sign_tx, chain_id).unwrap();

    let signature = tx.get_signature().unwrap();

    let msg = tx.get_message_to_sign().unwrap();

    let recovery_id = tx.get_recovery_id().unwrap();
    let address = recover_address(signature, recovery_id, msg);

    assert_eq!(res0.address, address)
}
#[test]
fn sign_eip2930_transaction() {
    let text = "aaaaa-aa";
    let principal_id = Principal::from_text(text).unwrap();
    let tx_parm = EVMTransactionEIP2930 {
        chain_id: 1,
        nonce: 0,
        gas_price: 36935555629,
        gas_limit: 31272,
        to: "0x70997970C51812dc3A010C7d01b50e0d17dc79C8".to_string(),
        value: 1000000000000000000,
        data: "0x00".to_string(),
        access_list: vec![],
    };
    let raw_tx = create_raw_eip_2930_tx(tx_parm);

    let chain_id: usize = 1;

    let res0 = block_on(create(principal_id)).unwrap();

    let res = block_on(sign(raw_tx.clone(), chain_id, principal_id)).unwrap();

    let tx = transaction::get_transaction(&res.sign_tx, chain_id).unwrap();

    let signature = tx.get_signature().unwrap();

    let message = tx.get_message_to_sign().unwrap();

    let recovery_id = tx.get_recovery_id().unwrap();
    let address = recover_address(signature, recovery_id, message);

    assert_eq!(res0.address, address)
}

#[test]
fn sign_eip1559_transaction() {
    let text = "aaaaa-aa";
    let principal_id = Principal::from_text(text).unwrap();

    let tx_parm = EVMTransactionEIP1559 {
        chain_id: 1,
        nonce: 0,
        max_priority_fee_per_gas: 1500000000,
        gas_limit: 31272,
        max_fee_per_gas: 36935555629,
        to: "0x70997970C51812dc3A010C7d01b50e0d17dc79C8".to_string(),
        value: 1000000000000000000,
        data: "0x00".to_string(),
        access_list: vec![],
    };
    let raw_tx = create_raw_tx_1559(tx_parm);

    let chain_id: usize = 1;

    let res0 = block_on(create(principal_id)).unwrap();

    let res = block_on(sign(raw_tx.clone(), chain_id, principal_id)).unwrap();

    let tx = transaction::get_transaction(&res.sign_tx, chain_id).unwrap();

    let signature = tx.get_signature().unwrap();

    let message = tx.get_message_to_sign().unwrap();

    let recovery_id = tx.get_recovery_id().unwrap();
    let address = recover_address(signature, recovery_id, message);

    assert_eq!(res0.address, address)
}
