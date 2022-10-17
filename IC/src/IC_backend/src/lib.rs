use ic_cdk::export::candid::CandidType;
use ic_cdk_macros::*;
use no_key_wallet;

#[derive(Debug, CandidType)]
struct Public_key_info {
    public_key: Vec<u8>,
}
#[derive(Debug, CandidType)]
struct Signature_info {
    signature: Vec<u8>,
}
#[update]
async fn get_public_key() -> Result<Public_key_info, String> {
    let caller = ic_cdk::caller().as_slice().to_vec();

    let res = no_key_wallet::public_key(caller)
        .await
        .map_err(|e| format!("Failed to call ecdsa_public_key {}", e.1))
        .unwrap();
    Ok(Public_key_info {
        public_key: res.public_key,
    })
}

#[update]
async fn sign_evm_tx(message: Vec<u8>) -> Result<Signature_info, String> {
    let res = no_key_wallet::sign(message)
        .await
        .map_err(|e| format!("Failed to call sign_with_ecdsa {}", e.1))
        .unwrap();

    Ok(Signature_info {
        signature: res.signature,
    })
}

candid::export_service!();

#[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}
