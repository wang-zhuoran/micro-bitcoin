use secp256k1::{Message, PublicKey, Secp256k1};
use secp256k1::ecdsa::Signature;
use sha2::{Sha256, Digest};
use crate::wallet::key::Keypair;

pub fn sign_and_verify(kp: &Keypair, message: &[u8]) {
    let secp = Secp256k1::new();

    // 1. 哈希消息
    let msg_hash = Sha256::digest(message);
    let msg = Message::from_digest(msg_hash.into());

    // 2. 签名
    let sig: Signature = secp.sign_ecdsa(msg, &kp.sk);

    // 3. 验证
    let result = secp.verify_ecdsa(msg, &sig, &kp.pk);

    // 4. 输出
    println!("📨 Message: {}", String::from_utf8_lossy(message));
    println!("✍️ Signature: {}", hex::encode(sig.serialize_der()));
    match result {
        Ok(_) => println!("✅ 验证成功（该私钥能花该地址的钱）"),
        Err(e) => println!("❌ 验证失败: {:?}", e),
    }
}

pub fn sign(kp: &Keypair, message: &[u8]) -> Signature {
    let secp = Secp256k1::new();
    let msg_hash = Sha256::digest(message);
    let msg = Message::from_digest(msg_hash.into());
    secp.sign_ecdsa(msg, &kp.sk)
}

pub fn verify_signature(sig: &Signature, pubkey: &PublicKey, message: &[u8]) -> bool {
    let secp = Secp256k1::new();
    let msg_hash = Sha256::digest(message);
    let msg = Message::from_digest(msg_hash.into());
    secp.verify_ecdsa(msg, sig, pubkey).is_ok()
}
