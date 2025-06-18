# Blockchain Implementation in Rust

A simple but functional blockchain implementation written in Rust, featuring proof-of-work consensus, transaction handling, and cryptographic security.

## Features

- **Proof-of-Work Consensus**: Configurable mining difficulty with SHA-256 hashing
- **Block Structure**: Immutable blocks with timestamps, transactions, and cryptographic links
- **Transaction System**: Support for sender/receiver transactions with amounts
- **Chain Validation**: Integrity verification through hash validation and chain linking
- **Ledger Management**: Balance tracking for wallet addresses
- **Cryptographic Security**: SHA-256 hashing for block integrity

## Project Structure

```
src/
├── main.rs          # Application entry point
├── lib.rs           # Module declarations
├── block.rs         # Block and transaction structures
├── blockchain.rs    # Blockchain implementation
└── wallet.rs        # Wallet functionality (placeholder)
```

## Core Components

### Block (`src/block.rs`)
- Represents a single block in the blockchain
- Contains transactions, timestamp, previous hash, and nonce
- Implements proof-of-work mining with configurable difficulty
- Uses SHA-256 for hash calculation

### Blockchain (`src/blockchain.rs`)
- Manages the chain of blocks
- Handles block addition and validation
- Maintains a ledger of wallet balances
- Provides chain integrity verification

### Transaction (`src/block.rs`)
- Represents transfers between wallet addresses
- Includes sender, receiver, and amount
- Supports serialization for blockchain storage

## Dependencies

- `chrono`: Timestamp handling
- `ed25519-dalek`: Cryptographic signatures (for future wallet implementation)
- `hex`: Hexadecimal encoding
- `rand`: Random number generation
- `serde`: Serialization/deserialization
- `serde_json`: JSON handling
- `sha2`: SHA-256 hashing

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo package manager

### Installation

1. Clone the repository:
```bash
git clone https://github.com/.../blockchain
cd blockchain
```

2. Build the project:
```bash
cargo build
```

3. Run the project:
```bash
cargo run
```

### Usage Example

```rust
use blockchain::blockchain::Blockchain;
use blockchain::block::Transaction;

fn main() {
    // Create a new blockchain with difficulty 4
    let mut blockchain = Blockchain::new(4);
    
    // Create a transaction
    let transaction = Transaction {
        sender: "Alice".to_string(),
        receiver: "Bob".to_string(),
        amount: 100,
    };
    
    // Add a block with the transaction
    blockchain.add_block(vec![transaction]);
    
    // Validate the blockchain
    if blockchain.is_valid_block() {
        println!("Blockchain is valid!");
    } else {
        println!("Blockchain validation failed!");
    }
}
```

## Technical Details

### Proof-of-Work Algorithm
- Uses SHA-256 hashing
- Difficulty is configurable (number of leading zeros required)
- Nonce incrementation for mining

### Block Structure
```rust
pub struct Block {
    pub index: u64,                    // Block position in chain
    pub timestamp: i64,                // Unix timestamp
    pub previous_hash: String,         // Hash of previous block
    pub hash: String,                  // Current block hash
    pub nonce: u64,                    // Proof-of-work nonce
    pub transactions: Vec<Transaction>, // Block transactions
}
```

### Hash Calculation
Blocks are hashed using the following formula:
```
hash = SHA256(index + timestamp + transactions_json + previous_hash + nonce)
```

## Future Enhancements

- [ ] Wallet implementation with Ed25519 signatures
- [ ] Network communication and peer-to-peer functionality
- [ ] Transaction pool and mempool management
- [ ] Smart contract support
- [ ] Web API for blockchain interaction
- [ ] Database persistence
- [ ] Mining rewards and coinbase transactions

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Inspired by Bitcoin's blockchain architecture
- Built with Rust for performance and safety
- Uses industry-standard cryptographic libraries 