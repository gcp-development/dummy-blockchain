use chrono::prelude::*;
use sha2::{Digest, Sha256};
use base64::{engine::general_purpose, Engine as _};

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Transaction {
    pub amount: f64,
    pub recipient: String,
    pub sender: String,
}

impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(amount:{}, recipient: {}, sender: {})", self.amount, self.recipient, self.recipient)
    }
}

pub struct Block {
    pub nonce: u64,
    pub timestamp: i64,
    pub previous_hash: String,
    pub transactions: Vec<Transaction>,
}

impl Block {

    pub fn new(nonce: u64,previous_hash: String,transactions: Vec<Transaction>) -> Self {
        let now = Utc::now();
        Self {
            nonce,
            timestamp: now.timestamp(),
            previous_hash,
            transactions,
        }
    }

    pub fn calculate_hash(block: &Block) -> String {
        let mut string_to_be_hashed: String = block.nonce.to_string().to_owned();
        let timestamp: String = block.timestamp.to_string();
        string_to_be_hashed.push_str(&timestamp);
        let previous_hash: String = block.previous_hash.to_string();
        string_to_be_hashed.push_str(&previous_hash);

        let mut hasher = Sha256::new();
        hasher.update(string_to_be_hashed);
        let result = hasher.finalize();
        general_purpose::STANDARD.encode(&result)
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(nonce:{}, timestamp(Epoch): {}, previous_hash: {})", self.nonce, self.timestamp, self.previous_hash)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_block() {
        const NONCE: u64 = 0;
        const VECTOR_SIZE: usize = 3;

        let tx1 = Transaction {
            amount: 1.0,
            recipient: String::from("1111t1WpEZ73CNmQviecrnyiWrnqRhWNLy"),
            sender: String::from("2222t1WpEZ73CNmQviecrnyiWrnqRhWNLy"),
        };

        let tx2 = Transaction {
            amount: 2.0,
            recipient: String::from("3333t1WpEZ73CNmQviecrnyiWrnqRhWNLy"),
            sender: String::from("4444t1WpEZ73CNmQviecrnyiWrnqRhWNLy"),
        };

        let tx3 = Transaction {
            amount: 3.0,
            recipient: String::from("5555t1WpEZ73CNmQviecrnyiWrnqRhWNLy"),
            sender: String::from("6666t1WpEZ73CNmQviecrnyiWrnqRhWNLy"),
        };

        let vec: Vec<Transaction> = Vec::from([tx1, tx2, tx3]);
        let genesis_block = Block::new(0, "empty".to_string(), vec);
        assert_eq!(genesis_block.nonce, NONCE);
        assert_eq!(genesis_block.previous_hash.to_string(), "empty".to_string());
        assert_eq!(genesis_block.transactions.len(), VECTOR_SIZE);
    }

    #[test]
    fn test_calculate_hash() {
        let tx1 = Transaction {
            amount: 1.0,
            recipient: String::from("1111t1WpEZ73CNmQviecrnyiWrnqRhWNLy"),
            sender: String::from("2222t1WpEZ73CNmQviecrnyiWrnqRhWNLy"),
        };

        let tx2 = Transaction {
            amount: 2.0,
            recipient: String::from("3333t1WpEZ73CNmQviecrnyiWrnqRhWNLy"),
            sender: String::from("4444t1WpEZ73CNmQviecrnyiWrnqRhWNLy"),
        };

        let tx3 = Transaction {
            amount: 3.0,
            recipient: String::from("5555t1WpEZ73CNmQviecrnyiWrnqRhWNLy"),
            sender: String::from("6666t1WpEZ73CNmQviecrnyiWrnqRhWNLy"),
        };

        let vec: Vec<Transaction> = Vec::from([tx1, tx2, tx3]);
        let genesis_block = Block::new(0, "empty".to_string(), vec);

        assert_eq!(Block::calculate_hash(&genesis_block).is_empty(), false);
    }
}