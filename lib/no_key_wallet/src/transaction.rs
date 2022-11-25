use easy_hasher::easy_hasher;
#[derive(Debug, Clone, PartialEq)]
enum TransactionType {
    Legacy,
    EIP1559,
    EPI2930,
}
#[derive(Debug)]
pub struct TransactionData {
    hex: Vec<u8>,
    chain_id: usize,
}
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

pub trait Sign {
    fn get_message_to_sign(&self) -> Result<Vec<u8>, String>;
    fn signed(&self, signature: Vec<u8>, rec_id: usize) -> Result<Vec<u8>, String>;
    fn is_signed(&self) -> bool;
    fn get_signature(&self) -> Result<Vec<u8>, String>;
    fn get_recovery_id(&self) -> Result<u8, String>;
}

pub struct TransactionLegacy {
    pub data: TransactionData,
}
impl Sign for TransactionLegacy {
    fn get_message_to_sign(&self) -> Result<Vec<u8>, String> {
        let rlp = rlp::Rlp::new(&self.data.hex[..]);

        let mut stream = rlp::RlpStream::new_list(9);
        for i in 0..=5 {
            let bytes: Vec<u8>;
            bytes = rlp.at(i).as_val::<Vec<u8>>();
            stream.append(&bytes);
        }

        let chain_id = vec![u8::try_from(self.data.chain_id.clone()).unwrap()];
        stream.append(&chain_id);

        stream.append_empty_data();
        stream.append_empty_data();

        let encoded_tx = stream.out();

        let keccak256 = easy_hasher::raw_keccak256(encoded_tx);

        Ok(keccak256.to_vec())
    }
    fn signed(&self, signature: Vec<u8>, rec_id: usize) -> Result<Vec<u8>, String> {
        let chain_id = u8::try_from(self.data.chain_id.clone()).unwrap();
        let r = &signature[..32];
        let s = &signature[32..];
        let v = vec![u8::try_from(chain_id * 2 + 35 + u8::try_from(rec_id).unwrap()).unwrap()];

        let rlp = rlp::Rlp::new(&self.data.hex[..]);

        let mut stream = rlp::RlpStream::new_list(9);

        for i in 0..=5 {
            let bytes = rlp.at(i).as_val::<Vec<u8>>();
            stream.append(&bytes);
        }

        stream.append(&v);
        stream.append(&r.to_vec());
        stream.append(&s.to_vec());

        Ok(stream.out())
    }
    fn is_signed(&self) -> bool {
        let rlp = rlp::Rlp::new(&self.data.hex);
        let r_is_empty = rlp.at(7).is_empty();
        let s_is_empty = rlp.at(8).is_empty();

        !r_is_empty && !s_is_empty
    }
    fn get_signature(&self) -> Result<Vec<u8>, String> {
        if !self.is_signed() {
            return Err("This is not  a signed transaction".to_string());
        }
        let rlp = rlp::Rlp::new(&self.data.hex);
        let r = rlp.at(7).as_val::<Vec<u8>>();
        let s = rlp.at(8).as_val::<Vec<u8>>();

        Ok([&r[..], &s[..]].concat())
    }
    fn get_recovery_id(&self) -> Result<u8, String> {
        if !self.is_signed() {
            return Err("This is not  a signed transaction".to_string());
        }
        let rlp = rlp::Rlp::new(&self.data.hex[..]);
        let chain_id = i8::try_from(self.data.chain_id).unwrap();
        let v = rlp.at(6).as_val::<Vec<u8>>();
        let recovery_id = -1 * ((chain_id * 2) + 35 - i8::try_from(v[0]).unwrap());
        Ok(u8::try_from(recovery_id).unwrap())
    }
}

pub struct Transaction1559 {
    pub data: TransactionData,
}
impl Sign for Transaction1559 {
    fn get_message_to_sign(&self) -> Result<Vec<u8>, String> {
        let rlp = rlp::Rlp::new(&self.data.hex[1..]);

        let mut stream = rlp::RlpStream::new_list(9);
        for i in 0..=7 {
            let item = rlp.at(i).as_val::<Vec<u8>>();
            stream.append(&item);
        }

        let item = rlp.at(8);
        let raw = item.as_raw();
        let item_count: usize = 1;
        stream.append_raw(raw, item_count);

        let decode_tx = stream.out();

        let msg = [&self.data.hex[..1], &decode_tx[..]].concat();
        let keccak256 = easy_hasher::raw_keccak256(msg);
        Ok(keccak256.to_vec())
    }
    fn signed(&self, signature: Vec<u8>, rec_id: usize) -> Result<Vec<u8>, String> {
        let r = &signature[..32];
        let s = &signature[32..];
        let rlp = rlp::Rlp::new(&self.data.hex[1..]);
        let mut stream = rlp::RlpStream::new_list(12);

        for i in 0..=7 {
            let bytes = rlp.at(i).as_val::<Vec<u8>>();

            stream.append(&bytes);
        }

        let access_list = rlp.at(8).as_raw();

        stream.append_raw(&access_list, 1);

        if rec_id == 0 {
            stream.append_empty_data();
        } else {
            let v = vec![0x01];
            stream.append(&v);
        }

        stream.append(&r);
        stream.append(&s);

        Ok([&self.data.hex[..1], &stream.out()].concat())
    }
    fn is_signed(&self) -> bool {
        let rlp = rlp::Rlp::new(&self.data.hex[1..]);
        let r_is_empty = rlp.at(10).is_empty();
        let s_is_empty = rlp.at(11).is_empty();

        !r_is_empty && !s_is_empty
    }
    fn get_signature(&self) -> Result<Vec<u8>, String> {
        if !self.is_signed() {
            return Err("This is not  a signed transaction".to_string());
        }
        let rlp = rlp::Rlp::new(&self.data.hex[1..]);
        let r = rlp.at(10).as_val::<Vec<u8>>();
        let s = rlp.at(11).as_val::<Vec<u8>>();

        Ok([&r[..], &s[..]].concat())
    }
    fn get_recovery_id(&self) -> Result<u8, String> {
        if !self.is_signed() {
            return Err("This is not  a signed transaction".to_string());
        }
        let rlp = rlp::Rlp::new(&self.data.hex[1..]);
        let v = rlp.at(9);

        if v.is_empty() {
            Ok(0 as u8)
        } else {
            Ok(1 as u8)
        }
    }
}

pub struct Transaction2930 {
    pub data: TransactionData,
}
impl Sign for Transaction2930 {
    fn get_message_to_sign(&self) -> Result<Vec<u8>, String> {
        let rlp = rlp::Rlp::new(&self.data.hex[1..]);
        let mut stream = rlp::RlpStream::new_list(8);

        for i in 0..=6 {
            let item = rlp.at(i).as_val::<Vec<u8>>();
            stream.append(&item);
        }

        let item = rlp.at(7);
        let raw = item.as_raw();
        let item_count: usize = 1;
        stream.append_raw(raw, item_count);
        let decode_tx = stream.out();

        let msg = [&self.data.hex[..1], &decode_tx[..]].concat();
        let keccak256 = easy_hasher::raw_keccak256(msg);
        Ok(keccak256.to_vec())
    }
    fn signed(&self, signature: Vec<u8>, rec_id: usize) -> Result<Vec<u8>, String> {
        let r = &signature[..32];
        let s = &signature[32..];
        let rlp = rlp::Rlp::new(&self.data.hex[1..]);
        let mut stream = rlp::RlpStream::new_list(11);

        for i in 0..=6 {
            let bytes = rlp.at(i).as_val::<Vec<u8>>();

            stream.append(&bytes);
        }

        let access_list = rlp.at(7).as_raw();

        stream.append_raw(&access_list, 1);

        if rec_id == 0 {
            stream.append_empty_data();
        } else {
            let v = vec![0x01];
            stream.append(&v);
        }

        stream.append(&r);
        stream.append(&s);

        Ok([&self.data.hex[..1], &stream.out()].concat())
    }

    fn is_signed(&self) -> bool {
        let rlp = rlp::Rlp::new(&self.data.hex[1..]);
        let r_is_empty = rlp.at(9).is_empty();
        let s_is_empty = rlp.at(10).is_empty();

        !r_is_empty && !s_is_empty
    }
    fn get_signature(&self) -> Result<Vec<u8>, String> {
        if !self.is_signed() {
            return Err("This is not  a signed transaction".to_string());
        }
        let rlp = rlp::Rlp::new(&self.data.hex[1..]);
        let r = rlp.at(9).as_val::<Vec<u8>>();
        let s = rlp.at(10).as_val::<Vec<u8>>();

        Ok([&r[..], &s[..]].concat())
    }
    fn get_recovery_id(&self) -> Result<u8, String> {
        if !self.is_signed() {
            return Err("This is not  a signed transaction".to_string());
        }
        let rlp = rlp::Rlp::new(&self.data.hex[1..]);
        let v = rlp.at(8);

        if v.is_empty() {
            Ok(0 as u8)
        } else {
            Ok(1 as u8)
        }
    }
}

pub fn get_transaction(hex_raw_tx: &Vec<u8>, chain_id: usize) -> Result<Box<dyn Sign>, String> {
    let data = TransactionData {
        hex: hex_raw_tx.clone(),
        chain_id,
    };
    let tx_type = get_transaction_type(hex_raw_tx).unwrap();

    if tx_type == TransactionType::Legacy {
        Ok(Box::new(TransactionLegacy { data }))
    } else if tx_type == TransactionType::EPI2930 {
        Ok(Box::new(Transaction2930 { data }))
    } else if tx_type == TransactionType::EIP1559 {
        Ok(Box::new(Transaction1559 { data }))
    } else {
        Err(String::from("Invalid type"))
    }
}

fn get_transaction_type(hex_raw_tx: &Vec<u8>) -> Result<TransactionType, String> {
    if hex_raw_tx[0] >= 0xc0 {
        Ok(TransactionType::Legacy)
    } else if hex_raw_tx[0] == 0x01 {
        Ok(TransactionType::EPI2930)
    } else if hex_raw_tx[0] == 0x02 {
        Ok(TransactionType::EIP1559)
    } else {
        Err(String::from("Invalid type"))
    }
}
