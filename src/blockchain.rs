
use chrono::Utc;
use serde::{Serialize, Deserialize};
use log::{error, info, warn};
use sha2::{Sha256, Digest};

pub struct App{
    pub blocks: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub data: String,
    pub nonce: u64,
}

const DIFFICULTY_PREFIX: &str = "00";

fn hash_to_binary(hash: &[u8]) -> String{
    let mut res = String::default();
    for c in hash{
        res.push_str(&format!("{:b}", c));
    }
    res
}

fn calculate_hash(id:  u64, timestamp : i64, previous_hash: &str, data: &str, nonce: u64) -> Vec<u8>{
    let data = serde_json::json!({
        "id": id,
        "timestamp" : timestamp,
        "previous_data" : previous_hash,
        "data": data,
        "nonce": nonce
    });
    let mut hasher = Sha256::new();
    hasher.update(data.to_string().as_bytes());
    hasher.finalize().as_slice().to_owned()

}

fn mine_block(id: u64, timestamp : i64, previous_hash: &str, data: &str) -> (u64, String){
    info!("Mining Block...");
    let mut nonce = 0;
    loop {
        if nonce % 100000 == 0{
            info!("nonce : {}",nonce);
        }
        let hash = calculate_hash(id, timestamp, previous_hash, data, nonce);
        let binary_hash = hash_to_binary(&hash);
        if binary_hash.starts_with(DIFFICULTY_PREFIX){
            info!("mined! \nnonce : {} \nhash : {} \nbinary_hash : {}", nonce, hex::encode(&hash), binary_hash);
            return(nonce, hex::encode(hash));
        }
        nonce += 1;
    }   
}

impl App{
    pub fn new() -> Self{
        Self {blocks : vec![]}
    }
    // first block 
    pub fn genesis(&mut self){
        let genesis_block = Block{
            id: 0,
            hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
            previous_hash: String::from("genesis"),
            timestamp: Utc::now().timestamp(),
            data: String::from("genesis!"),
            nonce: 2836,
        };
        self.blocks.push(genesis_block);
    }

    pub fn try_add_block(&mut self, block: Block){
        let last_block = self.blocks.last().expect("No blocks existing");
        if self.is_block_valid(&block, last_block){
            self.blocks.push(block);
        }
        else{
            error!("Couldn't add the block!");
        }
    }

    // implement error handling

    fn is_block_valid(&self, block: &Block, previous_block: &Block) -> bool {
        if block.previous_hash != previous_block.hash{
            warn!("block with id {} has invalid previous hash", block.id);
            return false;
        }
        else if !hash_to_binary(&hex::decode(&block.hash).expect("can't decode from hex")).starts_with(DIFFICULTY_PREFIX){
            warn!("block with id {} has wrong hash starting numbers", block.id);
            return  false;
        }
        else if block.id != previous_block.id + 1{
            warn!(
                "block with id: {} is not the next block after the latest: {}",
                block.id, previous_block.id
            );
            return false;
        }
        else if hex::encode(calculate_hash(block.id, block.timestamp, &block.previous_hash, &block.data, block.nonce)) != block.hash{
            warn!("block with id: {} has invalid hash", block.id);
            return false;
        }
        true
    }

    // consensus algorithm needed between nodes if they mined the same node at the same time and want ot add it


    // fn is_chain_valid(&self, chain: &[Block]) -> bool {
    //     for i in 0..chain.len(){
    //         if i == 0{
    //             continue;
    //         }
    //         let first = chain.get(i-1).expect("The block must exist!");
    //         let second = chain.get(i).expect("The block must exist!");
    //         if !self.is_block_valid(second, first){
    //         return  false;
    //     }
    //     }
    //     true
    // }

    // fn choose_chain(&self, local: Vec<Block>, remote: Vec<Block>) -> Vec<Block>{
    //     let is_local_valid = self.is_chain_valid(&local);
    //     let is_remote_valid = self.is_chain_valid(&remote);

    //     if is_local_valid && is_remote_valid{
    //         if local.len() >= remote.len(){
    //             local
    //         }
    //         else{
    //             remote
    //         }
    //     } 
    //     else if is_remote_valid && !is_local_valid{
    //         remote
    //     }  
    //     else if !is_remote_valid && is_local_valid{
    //         local
    //     }
    //     else{
    //         panic!("both chains are invalid!");
    //     }
    // }

    //choosing chain can be upgraded
}

impl Block {
    pub fn new(id: u64, previous_hash: String, data: String) -> Self{
        let now = Utc::now();
        let (nonce, hash) = mine_block(id, now.timestamp(), &previous_hash, &data);
        Self {
            id,
            hash,
            previous_hash,
            timestamp : now.timestamp(),
            data,
            nonce,
        }   
    }   
}

