use crate::utils::{string_to_vec_u8, u64_to_vec_u8, vec_u8_to_string, vec_u8_to_u64};
use easy_hasher::easy_hasher;
#[derive(Debug, Clone, PartialEq)]
enum TransactionType {
    Legacy,
    EIP1559,
    EIP2930,
}

pub trait Sign {
    fn get_message_to_sign(&self) -> Result<Vec<u8>, String>;
    fn sign(&self, signature: Vec<u8>, rec_id: u64) -> Result<Vec<u8>, String>;
    fn is_signed(&self) -> bool;
    fn get_signature(&self) -> Result<Vec<u8>, String>;
    fn get_recovery_id(&self) -> Result<u8, String>;
    fn get_nonce(&self) -> Result<u64, String>;
    fn serialize(&self) -> Result<Vec<u8>, String>;
}

pub struct TransactionLegacy {
    pub chain_id: u64,
    pub nonce: u64,
    pub gas_price: u64,
    pub gas_limit: u64,
    pub to: String,
    pub value: u64,
    pub data: String,
    pub v: String,
    pub r: String,
    pub s: String,
}
impl From<Vec<u8>> for TransactionLegacy {
    fn from(data: Vec<u8>) -> Self {
        let rlp = rlp::Rlp::new(&data[..]);

        let nonce_hex = rlp.at(0).as_val::<Vec<u8>>();
        let nonce = vec_u8_to_u64(&nonce_hex);

        let gas_price_hex = rlp.at(1).as_val::<Vec<u8>>();
        let gas_price = vec_u8_to_u64(&gas_price_hex);

        let gas_limit_hex = rlp.at(2).as_val::<Vec<u8>>();
        let gas_limit = vec_u8_to_u64(&gas_limit_hex);

        let to_hex = rlp.at(3).as_val::<Vec<u8>>();
        let to = vec_u8_to_string(&to_hex);

        let value_hex = rlp.at(4).as_val::<Vec<u8>>();
        let value = vec_u8_to_u64(&value_hex);

        let data_tx_hex = rlp.at(5).as_val::<Vec<u8>>();
        let data_tx = vec_u8_to_string(&data_tx_hex);

        let v_hex = rlp.at(6).as_val::<Vec<u8>>();
        let v = vec_u8_to_string(&v_hex);

        let r_hex = rlp.at(7).as_val::<Vec<u8>>();
        let r = vec_u8_to_string(&r_hex);

        let s_hex = rlp.at(8).as_val::<Vec<u8>>();
        let s = vec_u8_to_string(&s_hex);

        let chain_id_hex = rlp.at(9).as_val::<Vec<u8>>();
        let chain_id = vec_u8_to_u64(&chain_id_hex);

        TransactionLegacy {
            chain_id,
            nonce,
            gas_price,
            gas_limit,
            to,
            value,
            data: data_tx,
            v,
            r,
            s,
        }
    }
}
impl Sign for TransactionLegacy {
    // (nonce, gasprice, startgas, to, value, data, chainid, 0, 0)
    fn get_message_to_sign(&self) -> Result<Vec<u8>, String> {
        let mut stream = rlp::RlpStream::new_list(9);

        let items = [
            u64_to_vec_u8(&self.nonce),
            u64_to_vec_u8(&self.gas_price),
            u64_to_vec_u8(&self.gas_limit),
            string_to_vec_u8(&self.to),
            u64_to_vec_u8(&self.value),
            string_to_vec_u8(&self.data),
            u64_to_vec_u8(&self.chain_id),
        ];

        for item in items {
            stream.append(&item);
        }

        stream.append_empty_data();
        stream.append_empty_data();

        let encoded_tx = stream.out();

        let keccak256 = easy_hasher::raw_keccak256(encoded_tx);

        Ok(keccak256.to_vec())
    }
    fn sign(&self, signature: Vec<u8>, rec_id: u64) -> Result<Vec<u8>, String> {
        let chain_id = u8::try_from(self.chain_id).unwrap();

        let r = &signature[..32];
        let s = &signature[32..];
        let v = vec![u8::try_from(chain_id * 2 + 35 + u8::try_from(rec_id).unwrap()).unwrap()];

        let items = [
            u64_to_vec_u8(&self.nonce),
            u64_to_vec_u8(&self.gas_price),
            u64_to_vec_u8(&self.gas_limit),
            string_to_vec_u8(&self.to),
            u64_to_vec_u8(&self.value),
            string_to_vec_u8(&self.data),
        ];

        let mut stream = rlp::RlpStream::new_list(9);

        for item in items {
            stream.append(&item);
        }

        stream.append(&v);
        stream.append(&r.to_vec());
        stream.append(&s.to_vec());

        Ok(stream.out())
    }
    fn is_signed(&self) -> bool {
        let r = string_to_vec_u8(&self.r);
        let s = string_to_vec_u8(&self.s);

        !r.is_empty() && !s.is_empty()
    }
    fn get_signature(&self) -> Result<Vec<u8>, String> {
        if !self.is_signed() {
            return Err("This is not  a signed transaction".to_string());
        }

        let r = string_to_vec_u8(&self.r);
        let s = string_to_vec_u8(&self.s);

        Ok([&r[..], &s[..]].concat())
    }
    fn get_recovery_id(&self) -> Result<u8, String> {
        if !self.is_signed() {
            return Err("This is not  a signed transaction".to_string());
        }
        let chain_id = i8::try_from(self.chain_id).unwrap();
        let v = string_to_vec_u8(&self.v);
        let recovery_id = -1 * ((chain_id * 2) + 35 - i8::try_from(v[0]).unwrap());
        Ok(u8::try_from(recovery_id).unwrap())
    }
    fn serialize(&self) -> Result<Vec<u8>, String> {
        let mut stream = rlp::RlpStream::new_list(9);

        let nonce = u64_to_vec_u8(&self.nonce);
        stream.append(&nonce);

        let gas_price = u64_to_vec_u8(&self.gas_price);
        stream.append(&gas_price);

        let gas_limit = u64_to_vec_u8(&self.gas_limit);
        stream.append(&gas_limit);

        let to = string_to_vec_u8(&&self.to[2..]);
        stream.append(&to);

        let value = u64_to_vec_u8(&self.value);
        stream.append(&value);

        let data = string_to_vec_u8(&self.data[2..]);
        stream.append(&data);

        let v = string_to_vec_u8(&self.v[2..]);
        stream.append(&v);

        let r = string_to_vec_u8(&self.r[2..]);
        stream.append(&r);

        let s = string_to_vec_u8(&self.s[2..]);
        stream.append(&s);

        Ok(stream.out().to_vec())
    }
    fn get_nonce(&self) -> Result<u64, String> {
        Ok(self.nonce)
    }
}
pub struct Transaction2930 {
    pub chain_id: u64,
    pub nonce: u64,
    pub gas_price: u64,
    pub gas_limit: u64,
    pub to: String,
    pub value: u64,
    pub data: String,
    pub access_list: Vec<u8>,
    pub v: String,
    pub r: String,
    pub s: String,
}
// 0x01 || rlp([chainId, nonce, gasPrice, gasLimit, to, value, data, accessList, signatureYParity, signatureR, signatureS]).
impl From<Vec<u8>> for Transaction2930 {
    fn from(data: Vec<u8>) -> Self {
        let rlp = rlp::Rlp::new(&data[1..]);

        let chain_id_hex = rlp.at(0).as_val::<Vec<u8>>();
        let chain_id = vec_u8_to_u64(&chain_id_hex);

        let nonce_hex = rlp.at(1).as_val::<Vec<u8>>();
        let nonce = vec_u8_to_u64(&nonce_hex);

        let gas_price_hex = rlp.at(2).as_val::<Vec<u8>>();
        let gas_price = vec_u8_to_u64(&gas_price_hex);

        let gas_limit_hex = rlp.at(3).as_val::<Vec<u8>>();
        let gas_limit = vec_u8_to_u64(&gas_limit_hex);

        let to_hex = rlp.at(4).as_val::<Vec<u8>>();
        let to = vec_u8_to_string(&to_hex);

        let value_hex = rlp.at(5).as_val::<Vec<u8>>();
        let value = vec_u8_to_u64(&value_hex);

        let data_tx_hex = rlp.at(6).as_val::<Vec<u8>>();
        let data_tx = vec_u8_to_string(&data_tx_hex);

        let access_list = rlp.at(7).as_raw().to_vec();

        let v_hex = rlp.at(8).as_val::<Vec<u8>>();
        let v = vec_u8_to_string(&v_hex);

        let r_hex = rlp.at(9).as_val::<Vec<u8>>();
        let r = vec_u8_to_string(&r_hex);

        let s_hex = rlp.at(10).as_val::<Vec<u8>>();
        let s = vec_u8_to_string(&s_hex);
        Transaction2930 {
            chain_id,
            nonce,
            gas_price,
            gas_limit,
            to,
            data: data_tx,
            value,
            access_list,
            v,
            r,
            s,
        }
    }
}
impl Sign for Transaction2930 {
    fn get_message_to_sign(&self) -> Result<Vec<u8>, String> {
        let mut stream = rlp::RlpStream::new_list(8);
        let items = [
            u64_to_vec_u8(&self.chain_id),
            u64_to_vec_u8(&self.nonce),
            u64_to_vec_u8(&self.gas_price),
            u64_to_vec_u8(&self.gas_limit),
            string_to_vec_u8(&self.to),
            u64_to_vec_u8(&self.value),
            string_to_vec_u8(&self.data),
        ];

        for item in items {
            stream.append(&item);
        }

        let item_count: usize = 1;
        stream.append_raw(&self.access_list, item_count);

        let decode_tx = stream.out();

        let msg = [&[0x01], &decode_tx[..]].concat();
        let keccak256 = easy_hasher::raw_keccak256(msg);
        Ok(keccak256.to_vec())
    }
    fn sign(&self, signature: Vec<u8>, rec_id: u64) -> Result<Vec<u8>, String> {
        let r = &signature[..32];
        let s = &signature[32..];
        let mut stream = rlp::RlpStream::new_list(11);

        let items = [
            u64_to_vec_u8(&self.chain_id),
            u64_to_vec_u8(&self.nonce),
            u64_to_vec_u8(&self.gas_price),
            u64_to_vec_u8(&self.gas_limit),
            string_to_vec_u8(&self.to),
            u64_to_vec_u8(&self.value),
            string_to_vec_u8(&self.data),
        ];

        for item in items {
            stream.append(&item);
        }

        stream.append_raw(&self.access_list, 1);

        if rec_id == 0 {
            stream.append_empty_data();
        } else {
            let v = vec![0x01];
            stream.append(&v);
        }

        stream.append(&r);
        stream.append(&s);

        let result = stream.out();

        Ok([&[0x01], &result[..]].concat())
    }

    fn is_signed(&self) -> bool {
        let r = string_to_vec_u8(&self.r);
        let s = string_to_vec_u8(&self.s);

        !r.is_empty() && !s.is_empty()
    }
    fn get_signature(&self) -> Result<Vec<u8>, String> {
        if !self.is_signed() {
            return Err("This is not  a signed transaction".to_string());
        }

        let r = string_to_vec_u8(&self.r);
        let s = string_to_vec_u8(&self.s);

        Ok([&r[..], &s[..]].concat())
    }
    fn get_recovery_id(&self) -> Result<u8, String> {
        if !self.is_signed() {
            return Err("This is not  a signed transaction".to_string());
        }

        let v = string_to_vec_u8(&self.v);

        if v.is_empty() {
            Ok(0 as u8)
        } else {
            Ok(1 as u8)
        }
    }
    fn serialize(&self) -> Result<Vec<u8>, String> {
        let mut stream = rlp::RlpStream::new_list(11);

        let chain_id = u64_to_vec_u8(&self.chain_id);
        stream.append(&chain_id);

        let nonce = u64_to_vec_u8(&self.nonce);
        stream.append(&nonce);

        let gas_price = u64_to_vec_u8(&self.gas_price);
        stream.append(&gas_price);

        let gas_limit = u64_to_vec_u8(&self.gas_limit);
        stream.append(&gas_limit);

        let to = string_to_vec_u8(&self.to[2..]);
        stream.append(&to);

        let value = u64_to_vec_u8(&self.value);
        stream.append(&value);

        let data = string_to_vec_u8(&self.data[2..]);
        stream.append(&data);

        let access_list = rlp::encode_list(&self.access_list[..]);

        stream.append_raw(&access_list, 1);

        let v = string_to_vec_u8(&self.v[2..]);
        stream.append(&v);

        let r = string_to_vec_u8(&self.r[2..]);
        stream.append(&r);

        let s = string_to_vec_u8(&self.s[2..]);
        stream.append(&s);

        let result = stream.out().to_vec();

        Ok([&[0x01], &result[..]].concat())
    }
    fn get_nonce(&self) -> Result<u64, String> {
        Ok(self.nonce)
    }
}
#[derive(Debug, Clone)]
pub struct Transaction1559 {
    pub chain_id: u64,
    pub nonce: u64,
    pub max_priority_fee_per_gas: u64,
    pub gas_limit: u64,
    pub max_fee_per_gas: u64,
    pub to: String,
    pub value: u64,
    pub data: String,
    pub access_list: Vec<u8>,
    pub v: String,
    pub r: String,
    pub s: String,
}
impl From<Vec<u8>> for Transaction1559 {
    fn from(data: Vec<u8>) -> Self {
        let rlp = rlp::Rlp::new(&data[1..]);

        let chain_id_hex = rlp.at(0).as_val::<Vec<u8>>();
        let chain_id = vec_u8_to_u64(&chain_id_hex);

        let nonce_hex = rlp.at(1).as_val::<Vec<u8>>();
        let nonce = vec_u8_to_u64(&nonce_hex);

        let max_priority_fee_per_gas_hex = rlp.at(2).as_val::<Vec<u8>>();
        let max_priority_fee_per_gas = vec_u8_to_u64(&max_priority_fee_per_gas_hex);

        let max_fee_per_gas_hex = rlp.at(3).as_val::<Vec<u8>>();

        let max_fee_per_gas = vec_u8_to_u64(&max_fee_per_gas_hex);

        let gas_limit_hex = rlp.at(4).as_val::<Vec<u8>>();
        let gas_limit = vec_u8_to_u64(&gas_limit_hex);

        let to_hex = rlp.at(5).as_val::<Vec<u8>>();
        let to = vec_u8_to_string(&to_hex);

        let value_hex = rlp.at(6).as_val::<Vec<u8>>();
        let value = vec_u8_to_u64(&value_hex);

        let data_tx_hex = rlp.at(7).as_val::<Vec<u8>>();
        let data_tx = vec_u8_to_string(&data_tx_hex);

        let access_list = rlp.at(8).as_raw().to_vec();

        let v_hex = rlp.at(9).as_val::<Vec<u8>>();
        let v = vec_u8_to_string(&v_hex);

        let r_hex = rlp.at(10).as_val::<Vec<u8>>();
        let r = vec_u8_to_string(&r_hex);

        let s_hex = rlp.at(11).as_val::<Vec<u8>>();
        let s = vec_u8_to_string(&s_hex);

        Transaction1559 {
            chain_id,
            nonce,
            max_priority_fee_per_gas,
            max_fee_per_gas,
            gas_limit,
            to,
            value,
            data: data_tx,
            access_list,
            v,
            r,
            s,
        }
    }
}

impl Sign for Transaction1559 {
    fn get_message_to_sign(&self) -> Result<Vec<u8>, String> {
        let mut stream = rlp::RlpStream::new_list(9);
        let items = [
            u64_to_vec_u8(&self.chain_id),
            u64_to_vec_u8(&self.nonce),
            u64_to_vec_u8(&self.max_priority_fee_per_gas),
            u64_to_vec_u8(&self.max_fee_per_gas),
            u64_to_vec_u8(&self.gas_limit),
            string_to_vec_u8(&self.to),
            u64_to_vec_u8(&self.value),
            string_to_vec_u8(&self.data),
        ];

        for i in 0..=7 {
            let item = &items[i];
            stream.append(item);
        }

        stream.append_raw(&self.access_list, 1);

        let decode_tx = stream.out();

        let msg = [&[0x02], &decode_tx[..]].concat();

        let keccak256 = easy_hasher::raw_keccak256(msg);

        Ok(keccak256.to_vec())
    }

    fn sign(&self, signature: Vec<u8>, rec_id: u64) -> Result<Vec<u8>, String> {
        let r = &signature[..32];
        let s = &signature[32..];
        let mut stream = rlp::RlpStream::new_list(12);

        let selfs = [
            u64_to_vec_u8(&self.chain_id),
            u64_to_vec_u8(&self.nonce),
            u64_to_vec_u8(&self.max_priority_fee_per_gas),
            u64_to_vec_u8(&self.max_fee_per_gas),
            u64_to_vec_u8(&self.gas_limit),
            string_to_vec_u8(&self.to),
            u64_to_vec_u8(&self.value),
            string_to_vec_u8(&self.data),
        ];

        for i in 0..=7 {
            let item = &selfs[i];
            stream.append(item);
        }

        stream.append_raw(&self.access_list, 1);

        if rec_id == 0 {
            stream.append_empty_data();
        } else {
            let v = vec![0x01];
            stream.append(&v);
        }
        stream.append(&r);

        stream.append(&s);

        let result = stream.out();

        Ok([&[0x02], &result[..]].concat())
    }
    fn is_signed(&self) -> bool {
        !self.r.is_empty() || !self.r.is_empty()
    }
    fn get_signature(&self) -> Result<Vec<u8>, String> {
        if !self.is_signed() {
            return Err("This is not  a signed transaction".to_string());
        }

        let r = string_to_vec_u8(&self.r);
        let s = string_to_vec_u8(&self.s);

        Ok([&r[..], &s[..]].concat())
    }
    fn get_recovery_id(&self) -> Result<u8, String> {
        if !self.is_signed() {
            return Err("This is not  a signed transaction".to_string());
        }
        let v = &self.v;

        if v.is_empty() {
            Ok(0 as u8)
        } else {
            Ok(1 as u8)
        }
    }
    fn serialize(&self) -> Result<Vec<u8>, String> {
        let mut stream = rlp::RlpStream::new_list(12);

        let chain_id = u64_to_vec_u8(&self.chain_id);
        stream.append(&chain_id);

        let nonce = u64_to_vec_u8(&self.nonce);
        stream.append(&nonce);

        let max_priority_fee_per_gas = u64_to_vec_u8(&self.max_priority_fee_per_gas);
        stream.append(&max_priority_fee_per_gas);

        let max_fee_per_gas = u64_to_vec_u8(&self.max_fee_per_gas);
        stream.append(&max_fee_per_gas);

        let gas_limit = u64_to_vec_u8(&self.gas_limit);
        stream.append(&gas_limit);

        let to = string_to_vec_u8(&self.to[2..]);
        stream.append(&to);

        let value = u64_to_vec_u8(&self.value);
        stream.append(&value);

        let data = string_to_vec_u8(&self.data[2..]);
        stream.append(&data);

        let access_list = rlp::encode_list(&self.access_list[..]);

        stream.append_raw(&access_list, 1);

        let v = string_to_vec_u8(&self.v[2..]);
        stream.append(&v);

        let r = string_to_vec_u8(&self.r[2..]);
        stream.append(&r);

        let s = string_to_vec_u8(&self.s[2..]);
        stream.append(&s);

        let result = stream.out().to_vec();

        Ok([&[0x02], &result[..]].concat())
    }
    fn get_nonce(&self) -> Result<u64, String> {
        Ok(self.nonce)
    }
}

pub fn get_transaction(hex_raw_tx: &Vec<u8>, chain_id: u64) -> Result<Box<dyn Sign>, String> {
    let tx_type = get_transaction_type(hex_raw_tx).unwrap();

    if tx_type == TransactionType::Legacy {
        let rlp = rlp::Rlp::new(&hex_raw_tx);
        let mut stream = rlp::RlpStream::new_list(10);
        for i in 0..=8 {
            let item = rlp.at(i).as_val::<Vec<u8>>();
            stream.append(&item);
        }
        stream.append(&u64_to_vec_u8(&chain_id));

        let result = stream.out().to_vec();
        Ok(Box::new(TransactionLegacy::from(result)))
    } else if tx_type == TransactionType::EIP1559 {
        Ok(Box::new(Transaction1559::from(hex_raw_tx.clone())))
    } else if tx_type == TransactionType::EIP2930 {
        Ok(Box::new(Transaction2930::from(hex_raw_tx.clone())))
    } else {
        Err(String::from("Invalid type"))
    }
}

fn get_transaction_type(hex_raw_tx: &Vec<u8>) -> Result<TransactionType, String> {
    if hex_raw_tx[0] >= 0xc0 {
        Ok(TransactionType::Legacy)
    } else if hex_raw_tx[0] == 0x01 {
        Ok(TransactionType::EIP2930)
    } else if hex_raw_tx[0] == 0x02 {
        Ok(TransactionType::EIP1559)
    } else {
        Err(String::from("Invalid type"))
    }
}
