use crate::transaction::{EVMTransactionEIP1559, EVMTransactionEIP2930, EVMTransactionLegacy};
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

pub fn string_to_vec_u8(str: &str) -> Vec<u8> {
    (0..str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&str[i..i + 2], 16).unwrap())
        .collect::<Vec<u8>>()
}
pub fn usize_to_vec_u8(u: usize) -> Vec<u8> {
    u.to_be_bytes()
        .into_iter()
        .skip_while(|&x| x == 0)
        .collect()
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
pub fn create_raw_legacy_tx(arg: EVMTransactionLegacy) -> Vec<u8> {
    let mut stream = rlp::RlpStream::new_list(9);

    let nonce = usize_to_vec_u8(arg.nonce);
    stream.append(&nonce);

    let gas_price = usize_to_vec_u8(arg.gas_price);
    stream.append(&gas_price);

    let gas_limit = usize_to_vec_u8(arg.gas_limit);
    stream.append(&gas_limit);

    let to = string_to_vec_u8(&arg.to[2..]);
    stream.append(&to);

    let value = usize_to_vec_u8(arg.value);
    stream.append(&value);

    let data = string_to_vec_u8(&arg.data[2..]);
    stream.append(&data);

    for _i in 0..3 {
        stream.append_empty_data();
    }

    stream.out().to_vec()
}

pub fn create_raw_eip_2930_tx(arg: EVMTransactionEIP2930) -> Vec<u8> {
    let mut stream = rlp::RlpStream::new_list(11);

    let chain_id = usize_to_vec_u8(arg.chain_id);
    stream.append(&chain_id);

    let nonce = usize_to_vec_u8(arg.nonce);
    stream.append(&nonce);

    let gas_price = usize_to_vec_u8(arg.gas_price);
    stream.append(&gas_price);

    let gas_limit = usize_to_vec_u8(arg.gas_limit);
    stream.append(&gas_limit);

    let to = string_to_vec_u8(&arg.to[2..]);
    stream.append(&to);

    let value = usize_to_vec_u8(arg.value);
    stream.append(&value);

    let data = string_to_vec_u8(&arg.data[2..]);
    stream.append(&data);

    stream.append_raw(&[0xc0], 1);

    for _i in 0..3 {
        stream.append_empty_data();
    }

    let result = stream.out().to_vec();

    [&[0x01], &result[..]].concat()
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

pub fn create_raw_tx_1559(arg: EVMTransactionEIP1559) -> Vec<u8> {
    let mut stream = rlp::RlpStream::new_list(12);

    let chain_id = usize_to_vec_u8(arg.chain_id);
    stream.append(&chain_id);

    let nonce = usize_to_vec_u8(arg.nonce);
    stream.append(&nonce);

    let max_priority_fee_per_gas = usize_to_vec_u8(arg.max_priority_fee_per_gas);
    stream.append(&max_priority_fee_per_gas);

    let max_fee_per_gas = usize_to_vec_u8(arg.max_fee_per_gas);
    stream.append(&max_fee_per_gas);

    let gas_limit = usize_to_vec_u8(arg.gas_limit);
    stream.append(&gas_limit);

    let to = string_to_vec_u8(&arg.to[2..]);
    stream.append(&to);

    let value = usize_to_vec_u8(arg.value);
    stream.append(&value);

    let data = string_to_vec_u8(&arg.data[2..]);
    stream.append(&data);

    stream.append_raw(&[0xc0], 1);

    for _i in 0..3 {
        stream.append_empty_data();
    }

    let result = stream.out().to_vec();

    [&[0x02], &result[..]].concat()
}
