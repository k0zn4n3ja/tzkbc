use std::net::TcpStream;
use std::io::Write;
use tzkbc::network::message::Message;
use tzkbc::network::utils::serialize_message;
use tzkbc::blockchain::transaction::Transaction;

fn main() {
    // Define the server address
    let server_address = "127.0.0.1:8000";

    // Create a dummy transaction
    let dummy_transaction = Transaction {
        inputs: todo!(),
        outputs: todo!(),
        proof: todo!(),
        public_inputs: todo!(),
    };

    // Connect to the server
    match TcpStream::connect(server_address) {
        Ok(mut stream) => {
            println!("Connected to {}", server_address);

            // Send 200 transactions
            for _ in 0..200 {
                // Construct the transaction message
                let message = Message::Transaction(dummy_transaction.clone());
                let data = serialize_message(&message);

                // Send the serialized message
                if let Err(e) = stream.write_all(&data) {
                    println!("Failed to send transaction: {}", e);
                }
            }

            println!("Sent 200 transactions.");
        }
        Err(e) => {
            println!("Failed to connect to {}: {}", server_address, e);
        }
    }
}