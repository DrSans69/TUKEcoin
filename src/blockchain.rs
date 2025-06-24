use hex;
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct Block {
    pub height: u64,
    pub data: String,
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    pub fn new(height: u64, data: String, previous_hash: String, difficulty: usize) -> Self {
        let mut block: Block = Block {
            height,
            data,
            previous_hash,
            nonce: 0,
            hash: "".to_string(),
        };
        block.nonce = mine(&block, difficulty);
        block.hash = hex::encode(block.calculate_hash(block.nonce));
        block
    }

    pub fn calculate_hash(&self, nonce: u64) -> [u8; 32] {
        Sha256::digest(format!(
            "{}{}{}{}",
            self.height, self.data, self.previous_hash, nonce
        ))
        .into()
    }
}

pub fn mine(block: &Block, difficulty: usize) -> u64 {
    let mut nonce: u64 = 0;

    let mut hash: [u8; 32];

    loop {
        hash = block.calculate_hash(nonce);

        if hash.iter().take(difficulty).all(|&b| b == 0) {
            break;
        }
        nonce += 1;
    }

    nonce
}

pub struct Blockchain {
    pub block_height: u64,
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let genesis = Block::new(0, "Genesis Block".to_string(), "0".to_string(), difficulty);
        Self {
            block_height: 1,
            chain: vec![genesis],
            difficulty,
        }
    }

    pub fn add_block(&mut self, data: String) {
        let previous_hash = self.chain.last().unwrap().hash.clone();
        let block = Block::new(self.block_height, data, previous_hash, self.difficulty);
        self.block_height += 1;
        self.chain.push(block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.previous_hash != previous.hash {
                return false;
            }

            let recalculated = hex::encode(current.calculate_hash(current.nonce));

            if current.hash != recalculated {
                return false;
            }
        }
        true
    }
}
