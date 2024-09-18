use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    //Position of the block in the blockchain
    pub index: u64,
    // Time when the block was created
    pub timestamp: DateTime<Utc>,
    // Data stored in the block NOTE:  score and maybe IP? or user accounts
    pub data: String,
    // hash of previous block NOTE: not sure if we need this?
    pub previous_hash: String,
    //NOTE: hash of current block
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, data: String, previous_hash: String) -> Self {
        let timestamp = Utc::now();
        let hash = Block::calculate_hash(index, &timestamp, &data, &previous_hash);
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }

    pub fn calculate_hash(
        index: u64,
        timestamp: &DateTime<Utc>,
        data: &str,
        previous_hash: &str,
    ) -> String {
        let input = format!("{}{}{}{}", index, timestamp, data, previous_hash);
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }
}
