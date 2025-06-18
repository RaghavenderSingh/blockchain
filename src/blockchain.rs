use crate::block::Block;
use crate::block::Transaction;
use std::collections::HashMap;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub ledger: HashMap<String, u64>,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let genesis = Block::new(0, vec![], "0".to_string(), difficulty);
        Self {
            ledger: HashMap::new(),
            chain: vec![genesis],
            difficulty,
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        for tx in &transactions {
            let sender_balance = self.ledger.get(&tx.sender).copied().unwrap_or(0);
        }
        let last_block = self.chain.last().unwrap();
        let new_block = Block::new(
            last_block.index + 1,
            transactions,
            last_block.hash.clone(),
            self.difficulty,
        );
        self.chain.push(new_block);
    }

    pub fn is_valid_block(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous = &self.chain[i - 1];

            let recalculated_hash = Block::calculate_hash(
                current_block.index,
                current_block.timestamp as u64,
                &serde_json::to_string(&current_block.transactions).unwrap(),
                &current_block.previous_hash,
                current_block.nonce,
            );
            if recalculated_hash != current_block.hash {
                println!("Invalid hash at block {}", i);
                return false;
            }
            if current_block.previous_hash != previous.hash {
                println!("Broken chain at block {}", i);
                return false;
            }
            if !current_block.hash.starts_with(&"0".repeat(self.difficulty)) {
                println!("Block {} doesn't meet difficulty", i);
                return false;
            }
        }
        true
    }
}
