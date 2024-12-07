// src/network/client.rs
use std::net::TcpStream;
use std::io::{Read, Write};
use crate::network::utils::{serialize_message, deserialize_message};
use crate::network::message::Message;
use crate::blockchain::blockchain::Blockchain;
use std::sync::{Arc, Mutex};

pub fn connect_to_peer(blockchain: Arc<Mutex<Blockchain>>, address: &str) {
    match TcpStream::connect(address) {
        Ok(mut stream) => {
            println!("Connected to {}", address);
            request_chain(&mut stream);

            let blockchain = Arc::clone(&blockchain);
            std::thread::spawn(move || {
                handle_server_messages(stream, blockchain);
            });
        }
        Err(e) => {
            println!("Failed to connect to {}: {}", address, e);
        }
    }
}

pub fn connect_to_peers(blockchain: Arc<Mutex<Blockchain>>, addresses: Vec<String>) {
    for address in addresses {
        connect_to_peer(Arc::clone(&blockchain), &address);
    }
}

fn request_chain(stream: &mut TcpStream) {
    let message = Message::RequestChain;
    let data = serialize_message(&message);
    stream.write_all(&data).expect("Failed to send RequestChain message");
}

fn handle_server_messages(mut stream: TcpStream, blockchain: Arc<Mutex<Blockchain>>) {
    let mut buffer = [0u8; 1024];
    while match stream.read(&mut buffer) {
        Ok(size) => {
            if size == 0 {
                // Connection closed
                false
            } else {
                let message = deserialize_message(&buffer[..size]);
                handle_message(message, &blockchain);
                true
            }
        }
        Err(_) => {
            println!("An error occurred while reading from the server.");
            false
        }
    } {}
}

fn handle_message(message: Message, blockchain: &Arc<Mutex<Blockchain>>) {
    match message {
        Message::Transaction(tx) => {
            println!("Received transaction: {:?}", tx);
            let mut chain = blockchain.lock().unwrap();
            chain.add_transaction(tx);
        }
        Message::Block(block) => {
            println!("Received block: {:?}", block);
            let mut chain = blockchain.lock().unwrap();
            chain.add_block(block);
        }
        Message::Chain(chain_data) => {
            println!("Received blockchain with {} blocks", chain_data.len());
            let mut local_chain = blockchain.lock().unwrap();
            if chain_data.len() > local_chain.chain.len() {
                local_chain.chain = chain_data;
            }
        }
        _ => {}
    }
}
