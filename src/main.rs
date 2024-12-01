mod network;
mod blockchain;
mod cli;
mod utils;

use std::sync::{Arc, Mutex};
use network::{server::start_server, client::connect_to_peer};
use blockchain::blockchain::Blockchain;
use cli::cli::Cli;
use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();

    let blockchain = Arc::new(Mutex::new(Blockchain::new()));
    let server_blockchain = Arc::clone(&blockchain);
    std::thread::spawn(move || {
        start_server(server_blockchain, args.port);
    });

    // Connect to peers
    // let peer_blockchain = Arc::clone(&blockchain);

    {
        let mut blockchain = blockchain.lock().unwrap(); // TODO: GUJAS get rid of this
        blockchain.add_peers(args.peers.clone()); // get this of this cloning BS
    }    
}