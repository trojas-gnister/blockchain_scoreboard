use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use hex;

// frontend should take a players name. TODO: frontend needs to track score and playuer name
#[derive(Serialize, Deserialize,Debug, Clone)]
pub struct User {
    pub player_name: String,
    pub score: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    //Position of the block in the blockchain
    pub index: u64,
    // Time when the block was created
    pub timestamp: DateTime<Utc>,
    // Data stored in the block
    pub data: User,
    // hash of previous block
    pub previous_hash: String,
    //NOTE: hash of current block
    pub hash: String,
}
impl Block {
    pub fn new(index: u64, data: User, previous_hash: String) -> Self {
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
        data: &User,
        previous_hash: &str,
    ) -> String {
        // user block
        let input = format!("{}{}{:?}{}", index, timestamp, data, previous_hash);
        // will be used to hash inputs. will hash using Sha256
        let mut hasher = Sha256::new();
        // use hasher to hash the users block (input)
        hasher.update(input.as_bytes());
        // hash result
        let result = hasher.finalize();
        // converts hash to hexadecimal string
        hex::encode(result)
    }
}
