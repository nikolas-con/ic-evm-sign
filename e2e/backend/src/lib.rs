use ic_cdk::export::candid::CandidType;
use ic_cdk_macros::*;
use ic_evm_sign;
use ic_evm_sign::state::TransactionChainData;

#[derive(Debug, CandidType)]
struct CreateAddressResponse {
    address: String,
}
#[derive(Debug, CandidType)]
struct SignTransactionResponse {
    sign_tx: Vec<u8>,
}
#[derive(Debug, CandidType)]
struct DeployEVMContractResponse {
    tx: Vec<u8>,
}
#[derive(Debug, CandidType)]
struct UserResponse {
    address: String,
    transactions: TransactionChainData,
}

#[update]
async fn create_address() -> Result<CreateAddressResponse, String> {
    let principal_id = ic_cdk::caller();

    let res = ic_evm_sign::create_address(principal_id)
        .await
        .map_err(|e| format!("Failed to call ecdsa_public_key {}", e))
        .unwrap();

    Ok(CreateAddressResponse {
        address: res.address,
    })
}

#[update]
async fn sign_evm_tx(hex_raw_tx: Vec<u8>, chain_id: u64) -> Result<SignTransactionResponse, String> {
    let principal_id = ic_cdk::caller();
    let res = ic_evm_sign::sign_transaction(hex_raw_tx, chain_id, principal_id)
        .await
        .map_err(|e| format!("Failed to call sign_with_ecdsa {}", e))
        .unwrap();

    Ok(SignTransactionResponse {
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
    let res = ic_evm_sign::deploy_contract(
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
    let res = ic_evm_sign::transfer_erc_20(
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

    let res = ic_evm_sign::clear_caller_history(principal_id, chain_id)
        .map_err(|e| format!("Failed to call clear_caller_history {}", e))
        .unwrap();

    Ok(res)
}

#[query]
fn get_caller_data(chain_id: u64) -> Option<UserResponse> {
    let principal_id = ic_cdk::caller();

    let res = ic_evm_sign::get_caller_data(principal_id, chain_id);

    if let Some(caller) = res {
        Some(UserResponse {
            address: caller.address,
            transactions: caller.transactions,
        })
    } else {
        None
    }
}

candid::export_service!();

#[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}

#[ic_cdk_macros::pre_upgrade]
fn pre_upgrade() {
    ic_evm_sign::pre_upgrade();
}

#[ic_cdk_macros::post_upgrade]
fn post_upgrade() {
    ic_evm_sign::post_upgrade();
}
