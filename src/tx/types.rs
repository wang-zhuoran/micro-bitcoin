/// 交易输出（可被未来交易引用）
#[derive(Clone, Debug)]
pub struct TxOut {
    pub value: u64,            // 金额（单位：最小单位，类似 satoshi）
    pub address: [u8; 20],     // HASH160(pubkey) → 接收者地址
}

/// 交易输入（引用之前的 TxOut）
#[derive(Clone, Debug)]
pub struct TxIn {
    pub prev_txid: [u8; 32],   // 引用的交易哈希
    pub vout_index: u32,       // 引用的输出索引
    pub sig: Vec<u8>,          // 签名（DER 格式）
    pub pubkey: Vec<u8>,       // 公钥（压缩格式，33字节）
}

/// 一笔完整交易
#[derive(Clone, Debug)]
pub struct Tx {
    pub vin: Vec<TxIn>,        // 多个输入, 我正在花的币，可能需要花掉多个旧币来凑够钱
    pub vout: Vec<TxOut>,      // 多个输出，我正在创建的新币，这些币终将属于别人或者给自己找零，（可以分给多个地址 + 找零
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OutPoint {
    pub txid: String,
    pub index: usize,
}

/*
TxIn指的是消耗掉的币，这些币都需要去utxo_map中寻找是否存在，
验证是否能花掉，然后TxOut指的是系统中将要创造出的币，这些币的来源是TxIn，
但是完成这一笔交易（Tx）之后，utxo_map就要删掉了，而新创建的币需要加入utxo_map

e.g.,
当前状态 utxo_map：
- (Tx1, 0) → 你，0.6 BTC
- (Tx2, 1) → 你，0.8 BTC
- (Tx3, 0) → Bob，1.0 BTC


你要转账 1 BTC 给 Bob，用法如下：


Tx {
  vin = [ (Tx1, 0), (Tx2, 1) ]  // 你花掉这两笔 UTXO，一共 1.4 BTC

  vout = [
    (Bob, 1.0 BTC),        // 转给 Bob
    (你自己, 0.4 BTC)       // 找零
  ]
}


*/