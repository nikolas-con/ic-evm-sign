use easy_hasher::easy_hasher;
use ic_cdk::export::Principal;

pub fn get_derivation_path(caller: Principal) -> Vec<u8> {
    caller.as_slice().to_vec()
}

pub fn compute_address(public_key: Vec<u8>) -> String {
    let pub_key_arr: [u8; 33] = public_key[..].try_into().unwrap();
    let pub_key = libsecp256k1::PublicKey::parse_compressed(&pub_key_arr)
        .unwrap()
        .serialize();

    let keccak256 = easy_hasher::raw_keccak256(pub_key[1..].to_vec());
    let keccak256_hex = keccak256.to_hex_string();
    let address: String = "0x".to_owned() + &keccak256_hex[24..];

    address
}

pub fn get_rec_id(
    message: &Vec<u8>,
    signature: &Vec<u8>,
    public_key: &Vec<u8>,
) -> Result<usize, String> {
    for i in 0..3 {
        let recovery_id = libsecp256k1::RecoveryId::parse_rpc(27 + i).unwrap();

        let signature_bytes: [u8; 64] = signature[..].try_into().unwrap();
        let signature_bytes_64 = libsecp256k1::Signature::parse_standard(&signature_bytes).unwrap();

        let message_bytes: [u8; 32] = message[..].try_into().unwrap();
        let message_bytes_32 = libsecp256k1::Message::parse(&message_bytes);

        let key =
            libsecp256k1::recover(&message_bytes_32, &signature_bytes_64, &recovery_id).unwrap();
        if key.serialize_compressed() == public_key[..] {
            return Ok(i as usize);
        }
    }
    return Err("Not found".to_string());
}

pub fn string_to_vev_u8(str: &str) -> Vec<u8> {
    (0..str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&str[i..i + 2], 16).unwrap())
        .collect::<Vec<u8>>()
}

#[cfg(test)]
pub fn generate_random_private_key() -> libsecp256k1::SecretKey {
    loop {
        let mut ret = [0u8; 32];
        getrandom::getrandom(&mut ret).unwrap();
        if let Ok(key) = libsecp256k1::SecretKey::parse(&ret) {
            return key;
        }
    }
}
