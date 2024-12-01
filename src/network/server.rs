// src/network/server.rs
use super::utils::{deserialize_message, serialize_message};
use crate::blockchain::blockchain::Blockchain;
use crate::network::message::Message;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn start_server(blockchain: Arc<Mutex<Blockchain>>, port: u16) {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).expect("Failed to bind port");
    println!("Server listening on port {}", port);

    for stream in listener.incoming() {
        let stream = stream.expect("Failed to accept connection");
        let blockchain = Arc::clone(&blockchain);

        thread::spawn(move || {
            handle_connection(stream, blockchain);
        });
    }
}

fn handle_connection(mut stream: TcpStream, blockchain: Arc<Mutex<Blockchain>>) {
    let mut buffer = [0u8; 1024];
    while match stream.read(&mut buffer) {
        Ok(size) => {
            if size == 0 {
                // Connection closed
                false
            } else {
                let message = deserialize_message(&buffer[..size]);
                handle_message(message, &mut stream, &blockchain);
                true
            }
        }
        Err(_) => {
            println!("An error occurred while reading from the connection.");
            false
        }
    } {}
}

fn handle_message(message: Message, stream: &mut TcpStream, blockchain: &Arc<Mutex<Blockchain>>) {
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
        Message::RequestChain => {
            let chain = blockchain.lock().unwrap();
            let chain_message = Message::Chain(chain.chain.clone());
            let data = serialize_message(&chain_message);
            stream.write_all(&data).expect("Failed to send chain");
        }
        Message::Chain(chain) => {
            println!("Received blockchain with {} blocks", chain.len());
            let mut local_chain = blockchain.lock().unwrap();
            if chain.len() > local_chain.chain.len() {
                local_chain.chain = chain;
            }
        }
    }
}
