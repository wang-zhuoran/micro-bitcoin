use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha2::{Sha256, Digest};
use ripemd::Ripemd160;

pub struct Keypair {
    pub sk: SecretKey,
    pub pk: PublicKey,
    pub address: [u8; 20],
}

pub fn generate_keypair() -> Keypair {
    let secp = Secp256k1::new();
    let sk = SecretKey::new(&mut rand::rng());
    let pk = PublicKey::from_secret_key(&secp, &sk);

    // 生成地址（HASH160）
    let pk_bytes = pk.serialize();
    let sha256 = Sha256::digest(&pk_bytes);
    let ripemd = Ripemd160::digest(sha256);
    let mut address = [0u8; 20];
    address.copy_from_slice(&ripemd);

    Keypair { sk, pk, address }
}

pub fn pubkey_to_address(pubkey: &PublicKey) -> [u8; 20] {
    let pubkey_bytes = pubkey.serialize(); // compressed
    let sha256 = Sha256::digest(&pubkey_bytes);
    let ripemd = Ripemd160::digest(&sha256);
    let mut address = [0u8; 20];
    address.copy_from_slice(&ripemd);
    address
}


pub fn print_keypair(kp: &Keypair) {
    println!("🔐 Private Key: {}", hex::encode(kp.sk.secret_bytes()));
    println!("📡 Public Key:  {}", hex::encode(kp.pk.serialize()));
    println!("🏠 Address:      {}", hex::encode(kp.address));
}
