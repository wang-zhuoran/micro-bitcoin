# micro-bitcoin (Rust)
一个用 **Rust** 实现的极简比特币学习项目  
A minimal **Bitcoin** implementation in Rust for learning purposes

目标是用尽量少的代码，循序实现：交易 / UTXO / 脚本（子集）/ 区块 / PoW / 节点与网络，同步巩固 Rust 基础与系统设计理解。

> ⚠️ **免责声明**：本项目仅用于学习与研究。**请勿**用于主网真实资产场景。

## ✨ 目前进度 / Features

- [x] 基本数据结构：`Tx / TxIn / TxOut / OutPoint`
- [x] UTXO 集合（内存版 `UTXOMap`）
- [x] 交易校验（`verify_transaction`）
  - 引用的 `OutPoint` 存在性检查
  - 输入签名与脚本（先以占位/简化策略）验证
  - 输入/输出金额守恒
  - 基础规则：金额非负、无溢出、无重复花费（同交易内）
- [x] 单元测试（例如 `src/consensus/verify.rs`）
- [ ] （进行中）交易应用（`apply_transaction`）与状态更新
- [ ] 区块结构 / Merkle Root / 区块校验
- [ ] PoW（简化难度调整）


## Quick Start
```bash
# 构建
cargo build

# 运行全部测试
cargo test

# 仅运行共识校验相关测试（示例）
cargo test verify_transaction
```

## Reference
- S. Nakamoto. Bitcoin: A Peer-to-Peer Electronic Cash System