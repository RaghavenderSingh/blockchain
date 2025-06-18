use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedTransaction {
    pub transaction: Transaction,
    pub signature: String,
    pub public_key: String,
}

pub struct Wallet {
    signing_key: SigningKey,
    public_key: VerifyingKey,
}

impl Wallet {
    pub fn new() -> Self {
        let seed: [u8; 32] = [
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
        ];

        let signing_key = SigningKey::from_bytes(&seed);
        let public_key = signing_key.verifying_key();

        Self {
            signing_key,
            public_key,
        }
    }

    pub fn get_public_key(&self) -> &VerifyingKey {
        &self.public_key
    }

    pub fn get_public_key_hex(&self) -> String {
        hex::encode(self.public_key.as_bytes())
    }

    pub fn get_address(&self) -> String {
        self.get_public_key_hex()
    }

    pub fn create_transaction(&self, receiver: String, amount: u64) -> Transaction {
        Transaction {
            sender: self.get_address(),
            receiver,
            amount,
            timestamp: chrono::Utc::now().timestamp() as u64,
        }
    }

    pub fn sign_transaction(&self, transaction: &Transaction) -> SignedTransaction {
        let transaction_data = serde_json::to_string(transaction).unwrap();
        let signature = self.signing_key.sign(transaction_data.as_bytes());

        SignedTransaction {
            transaction: transaction.clone(),
            signature: hex::encode(signature.to_bytes()),
            public_key: self.get_public_key_hex(),
        }
    }

    pub fn verify_transaction(signed_transaction: &SignedTransaction) -> bool {
        let transaction_data = match serde_json::to_string(&signed_transaction.transaction) {
            Ok(data) => data,
            Err(_) => return false,
        };

        let public_key_bytes = match hex::decode(&signed_transaction.public_key) {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };

        let public_key_array: [u8; 32] = match public_key_bytes.try_into() {
            Ok(array) => array,
            Err(_) => return false,
        };

        let public_key = match VerifyingKey::from_bytes(&public_key_array) {
            Ok(key) => key,
            Err(_) => return false,
        };
        let signature_bytes = match hex::decode(&signed_transaction.signature) {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };
        let signature = match Signature::from_slice(&signature_bytes) {
            Ok(sig) => sig,
            Err(_) => return false,
        };
        public_key
            .verify(transaction_data.as_bytes(), &signature)
            .is_ok()
    }

    pub fn get_balance(&self, ledger: &std::collections::HashMap<String, u64>) -> u64 {
        let address = self.get_address();
        ledger.get(&address).copied().unwrap_or(0)
    }
}

impl Default for Wallet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_creation() {
        let wallet = Wallet::new();
        assert!(!wallet.get_public_key_hex().is_empty());
        assert!(!wallet.get_address().is_empty());
    }

    #[test]
    fn test_transaction_signing_and_verification() {
        let wallet = Wallet::new();
        let transaction = wallet.create_transaction("receiver_address".to_string(), 100);
        let signed_transaction = wallet.sign_transaction(&transaction);
        assert!(Wallet::verify_transaction(&signed_transaction));
    }

    #[test]
    fn test_invalid_signature_verification() {
        let wallet = Wallet::new();
        let transaction = wallet.create_transaction("receiver_address".to_string(), 100);
        let mut signed_transaction = wallet.sign_transaction(&transaction);
        signed_transaction.signature = "invalid_signature".to_string();

        assert!(!Wallet::verify_transaction(&signed_transaction));
    }
}
