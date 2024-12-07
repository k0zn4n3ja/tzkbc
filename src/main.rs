mod network;
mod blockchain;
mod cli;
mod utils;

use std::sync::{Arc, Mutex};
use network::{server::start_server, client::connect_to_peers};
use blockchain::blockchain::Blockchain;
use cli::cli::Cli;
use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));
    let server_blockchain = Arc::clone(&blockchain);
    let server_handle = std::thread::spawn(move || {
        start_server(server_blockchain, args.port);
    });

    {
        let mut blockchain_guard = blockchain.lock().expect("failed to lock blockchain");
        blockchain_guard.add_peers(args.peers.clone());
        connect_to_peers(Arc::clone(&blockchain), args.peers);
    }    
    

    server_handle.join().expect("Server thread panicked");
}