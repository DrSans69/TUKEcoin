use base58::ToBase58;
use secp256k1::{rand, PublicKey, Secp256k1, SecretKey};
use sha2::{Digest, Sha256};

use crate::transaction::Transaction;

#[derive(Debug)]
pub struct Wallet {
    pub private_key: SecretKey,
    pub public_key: PublicKey,
    pub address: String,
    pub tx_height: u64,
}

impl Wallet {
    pub fn new() -> Self {
        let secp = Secp256k1::new();

        let (private_key, public_key) = secp.generate_keypair(&mut rand::rng());

        let address = Self::public_key_to_address(&public_key);

        Wallet {
            private_key,
            public_key,
            address,
            tx_height: 0,
        }
    }

    fn public_key_to_address(pubkey: &PublicKey) -> String {
        let pubkey_bytes: [u8; 33] = pubkey.serialize();

        let sha256_hash = Sha256::digest(&pubkey_bytes);

        sha256_hash.to_base58()
    }

    pub fn create_transaction(
        &mut self,
        recipient: String,
        amount: u64,
        memo: Option<String>,
    ) -> Transaction {
        let mut tx = Transaction::new(
            self.tx_height,
            self.address.clone(),
            recipient,
            amount,
            memo,
            self.public_key,
        );
        tx.sign(&self.private_key);
        self.tx_height += 1;
        tx
    }
}
