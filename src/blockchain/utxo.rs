use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UTXO {
    pub value: u64,
    pub owner: String, 
}

