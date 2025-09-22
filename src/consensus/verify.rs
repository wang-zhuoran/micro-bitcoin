use crate::tx::types::{Tx, OutPoint};      
use crate::state::utxo::UTXOMap;
use crate::wallet::key::pubkey_to_address;
use crate::wallet::sign::verify_signature;
use secp256k1;  

pub fn verify_transaction(tx: &Tx, utxo_map: &UTXOMap) -> bool {
    let mut total_input = 0;
    let mut total_output = 0;

    for vin in &tx.vin {
        let outpoint = OutPoint {
            txid: hex::encode(vin.prev_txid),
            index: vin.vout_index as usize,
        };

        // 1. 检查引用的 OutPoint 是否存在于 UTXO 集合中
        let prev_output = match utxo_map.get(&outpoint) {
            Some(o) => o,
            None => return false,
        };

        // println!("prev_output.value = {}", prev_output.value);
        // println!("prev_output.address = {}", hex::encode(prev_output.address));
        // println!("vin.pubkey = {}", hex::encode(vin.pubkey.clone()));
        // println!("vin.sig = {}", hex::encode(vin.sig.clone()));
        // println!("outpoint.txid = {}", outpoint.txid);
        // println!("outpoint.index = {}", outpoint.index);

        // println!("--------------------------------");

        // 2. 校验签名是否有效（你是这笔钱的拥有者）
        let pubkey = match secp256k1::PublicKey::from_slice(&vin.pubkey) {
            Ok(pk) => pk,
            Err(_) => return false,
        };
        let derived_addr = pubkey_to_address(&pubkey);
        if derived_addr != prev_output.address {
            return false;
        }

        // 对消息进行签名校验（消息内容暂时可用固定格式）
        let msg = format!("spend:{}:{}", outpoint.txid, outpoint.index);
        let sig = match secp256k1::ecdsa::Signature::from_der(&vin.sig) {
            Ok(s) => s,
            Err(_) => return false,
        };
        if !verify_signature(&sig, &pubkey, msg.as_bytes()) {
            return false;
        }

        total_input += prev_output.value;
    }

    for vout in &tx.vout {
        total_output += vout.value;
    }
    println!("total_input = {}, total_output = {}", total_input, total_output);

    // 金额守恒
    total_input >= total_output
    
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::wallet::{key::generate_keypair, sign::sign};
    use crate::tx::types::{Tx, TxIn, TxOut, OutPoint};
    use crate::state::utxo::UTXOMap;

    #[test]
    fn test_verify_valid_transaction() {
        // 生成密钥对
        let kp = generate_keypair();

        // 模拟已有的 UTXO
        let prev_outpoint = OutPoint {
            txid: "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef".to_string(),
            index: 0,
        };

        let prev_txout = TxOut {
            value: 500,
            address: kp.address,
        };

        let mut utxo_map = UTXOMap::new();
        utxo_map.insert(prev_outpoint.clone(), prev_txout);

        // 构造签名（简单消息）
        let msg = format!("spend:{}:{}", prev_outpoint.txid, prev_outpoint.index);
        let sig = sign(&kp, msg.as_bytes());

        // 构造 Tx
        let tx = Tx {
            vin: vec![TxIn {
                prev_txid: {
                    let mut txid = [0u8; 32];
                    let decoded = hex::decode(&prev_outpoint.txid).unwrap();
                    txid.copy_from_slice(&decoded);
                    txid
                },
                vout_index: prev_outpoint.index as u32,
                sig: sig.serialize_der().to_vec(),
                pubkey: kp.pk.serialize().to_vec(),
            }],
            vout: vec![TxOut {
                value: 50,
                address: kp.address,
            }],
        };

        // 校验应通过
        assert!(verify_transaction(&tx, &utxo_map));
    }
}
