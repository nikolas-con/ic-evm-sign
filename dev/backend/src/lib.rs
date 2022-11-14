use ic_cdk::export::candid::CandidType;
use ic_cdk_macros::*;
use no_key_wallet;
use no_key_wallet::types::state::Transaction;

#[derive(Debug, CandidType)]
struct CreateResponse {
    address: String,
}
#[derive(Debug, CandidType)]
struct SignatureInfo {
    sign_tx: Vec<u8>,
}
#[derive(Debug, CandidType)]
struct CallerTransactionsResponse {
    transactions: Vec<Transaction>,
}
#[derive(Debug, CandidType)]
struct CallerResponse {
    address: String,
    transactions: Vec<Transaction>,
}

#[update]
async fn create() -> Result<CreateResponse, String> {
    let principal_id = ic_cdk::caller();

    let res = no_key_wallet::create(principal_id)
        .await
        .map_err(|e| format!("Failed to call ecdsa_public_key {}", e))
        .unwrap();

    Ok(CreateResponse {
        address: res.address,
    })
}

#[update]
async fn sign_evm_tx(hex_raw_tx: Vec<u8>, chain_id: usize) -> Result<SignatureInfo, String> {
    let principal_id = ic_cdk::caller();
    let res = no_key_wallet::sign(hex_raw_tx, chain_id, principal_id)
        .await
        .map_err(|e| format!("Failed to call sign_with_ecdsa {}", e))
        .unwrap();

    Ok(SignatureInfo {
        sign_tx: res.sign_tx,
    })
}

#[update]
fn clear_caller_history() -> Result<(), String> {
    let principal_id = ic_cdk::caller();

    let res = no_key_wallet::clear_caller_history(principal_id)
        .map_err(|e| format!("Failed to call clear_caller_history {}", e))
        .unwrap();

    Ok(res)
}

#[query]
fn get_caller_data() -> Result<CallerResponse, String> {
    let principal_id = ic_cdk::caller();

    let res = no_key_wallet::get_caller_data(principal_id)
        .map_err(|e| format!("Failed to call get_caller_data {}", e))
        .unwrap();

    Ok(CallerResponse {
        address: res.address,
        transactions: res.transactions,
    })
}

candid::export_service!();

#[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}
