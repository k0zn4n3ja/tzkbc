use serde::{Serialize, Deserialize};
use super::utxo::UTXO;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub inputs: Vec<UTXO>,
    pub outputs: Vec<UTXO>,
    pub proof: Vec<u8>,
    pub public_inputs: Vec<u8>,
    // TODO: GUJAS digital sig
}