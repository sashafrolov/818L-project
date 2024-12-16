use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct PublicValuesStruct {
    pub reg: String,
    pub input_hash: [u8; 32], // Hash of the input text
    pub is_match: bool,
}