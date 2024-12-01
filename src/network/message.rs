use serde::{Serialize, Deserialize};
use crate::blockchain::block::Block;
use crate::blockchain::transaction::Transaction;

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Transaction(Transaction),
    Block(Block),
    RequestChain,  
    Chain(Vec<Block>),
}