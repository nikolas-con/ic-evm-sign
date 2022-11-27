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

pub fn get_transfer_data(address: &str, value: u64) -> String {
    let method_sig = "transfer(address,uint256)";
    let keccak256 = easy_hasher::raw_keccak256(method_sig.as_bytes().to_vec());
    let method_id = &keccak256.to_hex_string()[..8];

    let address_64 = format!("{:0>64}", &address[2..]);

    let value_hex = format!("{:02x}", value);
    let value_64 = format!("{:0>64}", value_hex);

    method_id.to_owned() + &address_64 + &value_64
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

pub fn string_to_vec_u8(str: &str) -> Vec<u8> {
    (0..str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&str[i..i + 2], 16).unwrap())
        .collect::<Vec<u8>>()
}
pub fn u64_to_vec_u8(u: &u64) -> Vec<u8> {
    u.to_be_bytes()
        .into_iter()
        .skip_while(|&x| x == 0)
        .collect()
}

pub fn vec_u8_to_string(vec: &Vec<u8>) -> String {
    vec.iter()
        .map(|r| format!("{:02x}", r))
        .collect::<Vec<String>>()
        .join("")
        .to_string()
}

pub fn vec_u8_to_u64(vec: &Vec<u8>) -> u64 {
    let mut _vec = [0; 8];
    _vec[8 - vec.len()..].copy_from_slice(&vec);
    u64::from_be_bytes(_vec).try_into().unwrap()
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

#[cfg(test)]
pub fn recover_address(signature: Vec<u8>, recovery_id: u8, message: Vec<u8>) -> String {
    let signature_bytes: [u8; 64] = signature[..].try_into().unwrap();
    let signature_bytes_64 = libsecp256k1::Signature::parse_standard(&signature_bytes).unwrap();

    let recovery_id_byte =
        libsecp256k1::RecoveryId::parse(u8::try_from(recovery_id).unwrap()).unwrap();

    let message_bytes: [u8; 32] = message[..].try_into().unwrap();
    let message_bytes_32 = libsecp256k1::Message::parse(&message_bytes);

    let public_key =
        libsecp256k1::recover(&message_bytes_32, &signature_bytes_64, &recovery_id_byte).unwrap();

    compute_address(public_key.serialize_compressed().to_vec())
}
