use chrono::Utc;
use secp256k1::{ecdsa::Signature, Message, PublicKey, Secp256k1, SecretKey};
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
    pub signature: Option<Signature>,
    pub public_key: PublicKey,
}

impl Transaction {
    pub fn new(
        index: u64,
        sender: String,
        recipient: String,
        amount: u64,
        memo: Option<String>,
        public_key: PublicKey,
    ) -> Self {
        Self {
            index,
            sender,
            recipient,
            amount,
            memo,
            timestamp: Utc::now().timestamp(),
            signature: None,
            public_key,
        }
    }
    pub fn hash_without_signature(&self) -> Option<[u8; 32]> {
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{}{}{}{}{}{}",
            self.index, self.sender, self.recipient, self.amount, self.timestamp, self.public_key
        ));

        let bytes: [u8; 32] = hasher.finalize().into();

        Some(Sha256::digest(bytes).into())
    }

    pub fn sign(&mut self, private_key: &SecretKey) {
        let hash: Option<[u8; 32]> = self.hash_without_signature();
        if hash.is_none() {
            ()
        }

        let msg: Message = Message::from_digest(hash.unwrap());
        let signature: Signature = private_key.sign_ecdsa(msg);

        self.signature = Some(signature);
    }

    pub fn verify(&self) -> bool {
        if self.signature.is_none() {
            return false;
        }

        let hash: Option<[u8; 32]> = self.hash_without_signature();
        if hash.is_none() {
            return false;
        }

        let msg: Message = Message::from_digest(hash.unwrap());
        let secp = Secp256k1::new();

        self.public_key
            .verify(&secp, msg, &self.signature.unwrap())
            .is_ok()
    }
}
