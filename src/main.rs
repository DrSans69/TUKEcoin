use sha2::{Digest, Sha256};
use std::io;

const DIFFICULTY: usize = 1;

#[derive(Debug)]
pub struct Block {
    pub data: String,
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    pub fn new(data: String, previous_hash: String, difficulty: usize) -> Self {
        let mut nonce: u64 = 0;
        let mut hash: String = Self::calculate_hash(&data, &previous_hash, nonce);

        while &hash[..difficulty] != "0".repeat(difficulty) {
            nonce += 1;
            hash = Self::calculate_hash(&data, &previous_hash, nonce);
        }

        Self {
            data,
            previous_hash,
            nonce,
            hash,
        }
    }

    fn calculate_hash(data: &str, previous_hash: &str, nonce: u64) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{data}{previous_hash}{nonce}"));
        format!("{:x}", hasher.finalize())
    }
}

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let genesis = Block::new("Genesis Block".to_string(), "0".to_string(), difficulty);
        Self {
            chain: vec![genesis],
            difficulty,
        }
    }

    pub fn add_block(&mut self, data: String) {
        let previous_hash = self.chain.last().unwrap().hash.clone();
        let block = Block::new(data, previous_hash, self.difficulty);
        self.chain.push(block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.previous_hash != previous.hash {
                return false;
            }

            let recalculated =
                Block::calculate_hash(&current.data, &current.previous_hash, current.nonce);

            if current.hash != recalculated {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut chain = Blockchain::new(DIFFICULTY);

    loop {
        println!("Enter data for new block (or `exit`):");
        let mut data = String::new();
        io::stdin().read_line(&mut data).unwrap();
        let data = data.trim();

        if data == "exit" {
            break;
        }

        chain.add_block(data.to_string());
    }

    for block in &chain.chain {
        println!("{:#?}", block);
    }
    println!("Chain valid? {}\n", chain.is_valid());
}
