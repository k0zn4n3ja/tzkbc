use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub proof: Vec<u8>,
    pub public_inputs: Vec<u8>, 
}

