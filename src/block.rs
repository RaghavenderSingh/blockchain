use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignedTransaction {
    pub transaction: Transaction,
    pub signature: String,
    pub public_key: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(
        index: u64,
        transactions: Vec<Transaction>,
        previous_hash: String,
        difficulty: usize,
    ) -> Self {
        let timestamp = Utc::now().timestamp();
        let mut nonce = 0;
        loop {
            let transaction_json = serde_json::to_string(&transactions).unwrap();
            let hash = Block::calculate_hash(
                index,
                timestamp as u64,
                &transaction_json,
                &previous_hash,
                nonce,
            );
            if hash.starts_with(&"0".repeat(difficulty)) {
                return Block {
                    index,
                    timestamp,
                    previous_hash,
                    hash,
                    transactions,
                    nonce,
                };
            } else {
                nonce += 1;
            }
        }
    }
    pub fn calculate_hash(
        index: u64,
        timestamp: u64,
        tx_json: &str,
        previous_hash: &str,
        nonce: u64,
    ) -> String {
        let input = format!(
            "{}{}{}{}{}",
            index, timestamp, tx_json, previous_hash, nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}
