use ic_cdk::export::candid::CandidType;
use ic_cdk_macros::*;
use no_key_wallet;

#[derive(Debug, CandidType)]
struct CreateUserResponse {
    public_key: Vec<u8>,
    address: String,
}
#[derive(Debug, CandidType)]
struct SignatureInfo {
    sign_tx: Vec<u8>,
}
#[update]
async fn create_user() -> Result<CreateUserResponse, String> {
    let res = no_key_wallet::create()
        .await
        .map_err(|e| format!("Failed to call ecdsa_public_key {}", e))
        .unwrap();

    Ok(CreateUserResponse {
        public_key: res.public_key,
        address: res.address,
    })
}

#[update]
async fn sign_evm_tx(hex_raw_tx: Vec<u8>, public_key: Vec<u8>) -> Result<SignatureInfo, String> {
    let res = no_key_wallet::sign(hex_raw_tx, public_key)
        .await
        .map_err(|e| format!("Failed to call sign_with_ecdsa {}", e))
        .unwrap();

    Ok(SignatureInfo {
        sign_tx: res.sign_tx,
    })
}

candid::export_service!();

#[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}
