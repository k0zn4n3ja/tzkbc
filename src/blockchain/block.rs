use serde::{Serialize, Deserialize};
use crate::transaction::Transaction;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub previous_hash: String,
    pub timestamp: u128,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
    pub hash: String,
}