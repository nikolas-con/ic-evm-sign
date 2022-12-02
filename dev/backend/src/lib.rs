use ic_cdk::export::candid::CandidType;
use ic_cdk_macros::*;
use no_key_wallet;
use no_key_wallet::state::ChainData;

#[derive(Debug, CandidType)]
struct CreateResponse {
    address: String,
}
#[derive(Debug, CandidType)]
struct SignatureInfo {
    sign_tx: Vec<u8>,
}

#[derive(Debug, CandidType)]
struct DeployEVMContractResponse {
    tx: Vec<u8>,
}
#[derive(Debug, CandidType)]
struct CallerResponse {
    address: String,
    transactions: ChainData,
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
async fn sign_evm_tx(hex_raw_tx: Vec<u8>, chain_id: u64) -> Result<SignatureInfo, String> {
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
async fn deploy_evm_contract(
    bytecode: Vec<u8>,
    chain_id: u64,
    max_priority_fee_per_gas: u64,
    gas_limit: u64,
    max_fee_per_gas: u64,
) -> Result<DeployEVMContractResponse, String> {
    let principal_id = ic_cdk::caller();
    let res = no_key_wallet::deploy_contract(
        principal_id,
        bytecode,
        chain_id,
        max_priority_fee_per_gas,
        gas_limit,
        max_fee_per_gas,
    )
    .await
    .map_err(|e| format!("Failed to call sign_with_ecdsa {}", e))
    .unwrap();

    Ok(DeployEVMContractResponse { tx: res.tx })
}

#[update]
async fn transfer_erc_20(
    chain_id: u64,
    max_priority_fee_per_gas: u64,
    gas_limit: u64,
    max_fee_per_gas: u64,
    address: String,
    value: u64,
    contract_address: String,
) -> Result<DeployEVMContractResponse, String> {
    let principal_id = ic_cdk::caller();
    let res = no_key_wallet::transfer_erc_20(
        principal_id,
        chain_id,
        max_priority_fee_per_gas,
        gas_limit,
        max_fee_per_gas,
        address,
        value,
        contract_address,
    )
    .await
    .map_err(|e| format!("Failed to call sign_with_ecdsa {}", e))
    .unwrap();

    Ok(DeployEVMContractResponse { tx: res.tx })
}

#[update]
fn clear_caller_history(chain_id: u64) -> Result<(), String> {
    let principal_id = ic_cdk::caller();

    let res = no_key_wallet::clear_caller_history(principal_id, chain_id)
        .map_err(|e| format!("Failed to call clear_caller_history {}", e))
        .unwrap();

    Ok(res)
}

#[query]
fn get_caller_data(chain_id: u64) -> Result<CallerResponse, String> {
    let principal_id = ic_cdk::caller();

    let res = no_key_wallet::get_caller_data(principal_id, chain_id)
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

#[ic_cdk_macros::pre_upgrade]
fn pre_upgrade() {
    no_key_wallet::pre_upgrade();
}

#[ic_cdk_macros::post_upgrade]
fn post_upgrade() {
    no_key_wallet::post_upgrade();
}
