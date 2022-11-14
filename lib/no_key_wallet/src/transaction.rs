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

pub trait Sign {
    fn get_message_to_sign(&self) -> Result<Vec<u8>, String>;
    fn signed(&self, signature: Vec<u8>, rec_id: usize) -> Result<Vec<u8>, String>;
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
        let v = u8::try_from(chain_id * 2 + 35 + u8::try_from(rec_id).unwrap()).unwrap();

        let rlp = rlp::Rlp::new(&self.data.hex[..]);

        let mut stream = rlp::RlpStream::new_list(9);
        for i in 0..=8 {
            let bytes: Vec<u8>;
            if i == 6 {
                bytes = vec![v];
            } else if i == 7 {
                bytes = r.to_vec();
            } else if i == 8 {
                bytes = s.to_vec();
            } else {
                bytes = rlp.at(i).as_val::<Vec<u8>>();
            }
            stream.append(&bytes);
        }

        Ok(stream.out())
    }
}

pub struct Transaction1559 {
    pub data: TransactionData,
}
impl Sign for Transaction1559 {
    fn get_message_to_sign(&self) -> Result<Vec<u8>, String> {
        let rlp = rlp::Rlp::new(&self.data.hex[1..]);

        let mut stream = rlp::RlpStream::new_list(9);
        for i in 0..=8 {
            if i == 8 {
                let item = rlp.at(i);
                let raw = item.as_raw();
                let item_count: usize = 1;
                stream.append_raw(raw, item_count);
            } else {
                let item = rlp.at(i).as_val::<Vec<u8>>();
                stream.append(&item);
            }
        }

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

        for i in 0..12 {
            if i == 8 {
                let val = rlp.at(i).as_raw();

                stream.append_raw(&val, 1);
            } else if i == 9 {
                if rec_id == 0 {
                    stream.append_empty_data();
                } else {
                    let v = vec![0x01];
                    stream.append(&v);
                }
            } else if i == 10 {
                stream.append(&r);
            } else if i == 11 {
                stream.append(&s);
            } else {
                let bytes = rlp.at(i).as_val::<Vec<u8>>();

                stream.append(&bytes);
            }
        }
        Ok([&self.data.hex[..1], &stream.out()].concat())
    }
}

pub struct Transaction2930 {
    pub data: TransactionData,
}
impl Sign for Transaction2930 {
    fn get_message_to_sign(&self) -> Result<Vec<u8>, String> {
        let rlp = rlp::Rlp::new(&self.data.hex[1..]);

        let mut stream = rlp::RlpStream::new_list(8);

        for i in 0..=7 {
            if i == 7 {
                let item = rlp.at(i);
                let raw = item.as_raw();
                let item_count: usize = 1;
                stream.append_raw(raw, item_count);
            } else {
                let item = rlp.at(i).as_val::<Vec<u8>>();
                stream.append(&item);
            }
        }
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

        for i in 0..11 {
            if i == 7 {
                let val = rlp.at(i).as_raw();

                stream.append_raw(&val, 1);
            } else if i == 8 {
                if rec_id == 0 {
                    stream.append_empty_data();
                } else {
                    let v = vec![0x01];
                    stream.append(&v);
                }
            } else if i == 9 {
                stream.append(&r);
            } else if i == 10 {
                stream.append(&s);
            } else {
                let bytes = rlp.at(i).as_val::<Vec<u8>>();

                stream.append(&bytes);
            }
        }
        Ok([&self.data.hex[..1], &stream.out()].concat())
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
    if hex_raw_tx[0] == 0xf8 {
        return Ok(TransactionType::Legacy);
    } else if hex_raw_tx[0] == 0x01 {
        return Ok(TransactionType::EPI2930);
    } else if hex_raw_tx[0] == 0x02 {
        return Ok(TransactionType::EIP1559);
    } else {
        return Err(String::from("Invalid type"));
    }
}
