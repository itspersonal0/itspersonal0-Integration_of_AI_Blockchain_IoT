use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: usize,
    pub timestamp: u64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Block #{} [Hash: {}...] - Data: {}", 
               self.index, 
               &self.hash[..10], 
               if self.data.len() > 50 { 
                   format!("{}...", &self.data[..50]) 
               } else { 
                   self.data.clone() 
               })
    }
}

impl Block {
    pub fn new(index: usize, timestamp: u64, data: String, previous_hash: String) -> Self {
        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        
        block.mine(2); // Difficulty level 2 (two leading zeros)
        block
    }
    
    pub fn calculate_hash(&self) -> String {
        let data = format!("{}{}{}{}{}", 
                          self.index, 
                          self.timestamp, 
                          self.data, 
                          self.previous_hash,
                          self.nonce);
        
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }
    
    fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        
        loop {
            self.hash = self.calculate_hash();
            if self.hash.starts_with(&target) {
                break;
            }
            self.nonce += 1;
        }
        
        println!("Block mined: {}", self.hash);
    }
}

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
        };
        
        // Create genesis block
        let genesis_block = Block::new(
            0,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            String::from("Genesis Block"),
            String::from("0"),
        );
        
        blockchain.chain.push(genesis_block);
        blockchain
    }
    
    pub fn add_block(&mut self, data: String) -> &Block {
        let previous_block = self.chain.last().unwrap();
        let new_block = Block::new(
            previous_block.index + 1,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            data,
            previous_block.hash.clone(),
        );
        
        self.chain.push(new_block);
        self.chain.last().unwrap()
    }
    
    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];
            
            // Verify hash
            if current_block.hash != current_block.calculate_hash() {
                println!("Invalid hash for block {}", current_block.index);
                return false;
            }
            
            // Verify chain link
            if current_block.previous_hash != previous_block.hash {
                println!("Invalid chain link at block {}", current_block.index);
                return false;
            }
        }
        
        true
    }
    
    pub fn get_all_blocks(&self) -> Vec<&Block> {
        self.chain.iter().collect()
    }
    
    pub fn get_blockchain_data(&self) -> Vec<String> {
        self.chain.iter().map(|block| block.data.clone()).collect()
    }
    
    pub fn get_chain_length(&self) -> usize {
        self.chain.len()
    }
}