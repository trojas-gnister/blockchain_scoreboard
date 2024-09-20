use super::block::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    // initializes blockchain with a "genesis block" (first block in the blockchain)
    pub fn new() -> Self {
        let mut blockchain = Blockchain { blocks: vec![] };
        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self) {
        let genesis_block = Block::new(
            0,
            User {
                player_name: "Genesis Block".to_string(),
                score: 9999,
            },
            "0".to_string(),
        );
        self.blocks.push(genesis_block);
    }

    // function responsible for creating a block on the blockchain
    pub fn add_block(&mut self, data: User) {
        let last_block = self.blocks.last().unwrap();
        let new_block = Block::new(last_block.index + 1, data, last_block.hash.clone());
        self.blocks.push(new_block);
    }
}
