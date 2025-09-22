use std::collections::HashMap;
use crate::tx::types::OutPoint;
use crate::tx::types::TxOut;

pub type UTXOMap = HashMap<OutPoint, TxOut>;
