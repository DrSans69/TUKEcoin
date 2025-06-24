use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub index: u64,
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
    pub memo: Option<String>,
    pub timestamp: i64,
    pub signature: Option<String>,
}

impl Transaction {
    pub fn new(
        index: u64,
        sender: String,
        recipient: String,
        amount: u64,
        memo: Option<String>,
    ) -> Self {
        Self {
            index,
            sender,
            recipient,
            amount,
            memo,
            timestamp: Utc::now().timestamp(),
            signature: None,
        }
    }
    pub fn hash_without_signature(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{}{}{}{}{}",
            self.index, self.sender, self.recipient, self.amount, self.timestamp
        ));

        let bytes: [u8; 32] = hasher.finalize().into();

        Sha256::digest(bytes).into()
    }
}
