use super::transaction::Transaction;
use hex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockHeader {
    pub index: u64,
    pub version: u32,
    pub previous_hash: String,
    pub merkle_root: String, // TODO: GUJAS need to validate transactions in block validation
    pub timestamp: u128,
    pub nonce: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub hash: String, // TODO: GUJAS for now. will see if actually needs to be here
    pub transactions: Vec<Transaction>,
}

impl BlockHeader {
    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let mut header_bytes = Vec::new();
        header_bytes.extend(&self.index.to_le_bytes());
        header_bytes.extend(&self.version.to_le_bytes());
        header_bytes.extend(hex::decode(&self.previous_hash).expect("Invalid previous hash"));
        header_bytes.extend(hex::decode(&self.merkle_root).expect("Invalid merkle root"));
        header_bytes.extend(&self.timestamp.to_le_bytes());
        header_bytes.extend(&self.nonce.to_le_bytes());

        hasher.update(&header_bytes);
        let first_hash = hasher.finalize_reset();
        hasher.update(first_hash);
        let final_hash = hasher.finalize();
        hex::encode(final_hash)
    }
}
