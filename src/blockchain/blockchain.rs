use std::io::Write;
use std::net::TcpStream;
use crate::network::message::Message;
use crate::network::utils::serialize_message;
use crate::utils::time::current_timestamp;
use std::sync::{Arc, Mutex}; // TODO: GUJAS do this
use super::block::{Block, BlockHeader};
use super::transaction::Transaction;
use structopt::StructOpt;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
    pub peers: Vec<String>,
}

impl Blockchain {
    fn get_latest_block(&self) -> Option<&Block> {
        self.chain.last()
    }

    fn validate_transaction(&self, _transaction: &Transaction) -> bool {
        // TODO: GUJAS update this to do the actual validations - merkle proof of txs, sigs etc.
        true
    }

    /// If it's the first block, an empty string will be returned
    fn get_previous_hash_or_default(&self) -> String {
        match self.get_latest_block() {
            Some(block) => block.hash.clone(),
            None => String::new(),
        }
    }

    fn mine_pending_transactions(&mut self, difficulty: usize) {
        let mut block = Block {
            header: BlockHeader {
                index: self.chain.len() as u64,
                previous_hash: self.get_previous_hash_or_default(),
                timestamp: current_timestamp(),
                nonce: 0,
                version: 1,
                merkle_root: String::new(), // TODO: Implement construction of tree and so on and so on
            },
            hash: String::new(),
            transactions: self.pending_transactions.clone(),
        };

        while &block.hash[0..difficulty] != "0".repeat(difficulty) {
            block.header.nonce += 1;
            block.hash = block.header.calculate_hash();
        }

        self.chain.push(block);
        self.pending_transactions.clear();
    }

    pub fn new() -> Self {
        // Create the genesis block
        let mut genesis_block = Block {
            header: BlockHeader {
                index: 0,
                previous_hash: String::from(""), // No previous hash for genesis block
                timestamp: current_timestamp(),
                nonce: 0,
                version: 1,
                merkle_root: String::new(), // TODO: GUJAS whatevs man
            },

            transactions: vec![],
            hash: String::new(), // calced underneath
        };

        genesis_block.hash = genesis_block.header.calculate_hash();

        Blockchain {
            chain: vec![genesis_block],
            pending_transactions: vec![],
            peers: vec![],
        }
    }

    /**
     * PUBLIC NETWORK METHODS
     */

    pub fn broadcast_transaction(&self, transaction: &Transaction) {
        let message = Message::Transaction(transaction.clone());
        let data = serialize_message(&message);
        for peer in &self.peers {
            if let Ok(mut stream) = TcpStream::connect(peer) {
                stream.write_all(&data).expect("Failed to send transaction");
            }
        }
    }

    pub fn broadcast_block(&self, block: &Block) {
        let message = Message::Block(block.clone());
        let data = serialize_message(&message);
        for peer in &self.peers {
            if let Ok(mut stream) = TcpStream::connect(peer) {
                stream.write_all(&data).expect("Failed to send block");
            }
        }
    }

    pub fn add_peer(&mut self, address: String) {
        self.peers.push(address);
    }

    pub fn add_peers(&mut self, addresses: Vec<String>) {
        for address in addresses {
            self.add_peer(address);
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction.clone());
        self.broadcast_transaction(&transaction);
    }

    pub fn add_block(&mut self, block: Block) {
        self.chain.push(block.clone());
        self.broadcast_block(&block);
    }
}
