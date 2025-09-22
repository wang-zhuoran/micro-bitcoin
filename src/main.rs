mod wallet;

use wallet::key;
use wallet::sign;
mod consensus;
use consensus::verify;
mod state;
use state::utxo;
mod tx;
use tx::types;

fn main() {
    println!("🪙 microbtc: 签名验证测试");

    let kp = key::generate_keypair();
    key::print_keypair(&kp);

    let msg = "我要花 txid=deadbeef 的第0个输出".as_bytes();

    sign::sign_and_verify(&kp, msg);
}
