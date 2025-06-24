use std::str::FromStr;

use base58::ToBase58;
use secp256k1::{ecdsa::Signature, rand, Message, PublicKey, Secp256k1, SecretKey};
use sha2::{Digest, Sha256};

use crate::transaction::Transaction;

#[derive(Debug)]
pub struct Wallet {
    pub private_key: SecretKey,
    pub public_key: PublicKey,
    pub address: String,
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
        }
    }

    fn public_key_to_address(pubkey: &PublicKey) -> String {
        let pubkey_bytes: [u8; 33] = pubkey.serialize();

        let sha256_hash = Sha256::digest(&pubkey_bytes);

        sha256_hash.to_base58()
    }

    pub fn sign(&self, transaction: &mut Transaction) {
        let hash: [u8; 32] = transaction.hash_without_signature();
        let msg: Message = Message::from_digest(hash);
        let signature: String = self.private_key.sign_ecdsa(msg).to_string();

        transaction.signature = Some(signature);
    }

    pub fn verify(&self, transaction: &Transaction) -> bool {
        return verify(&self.public_key, transaction);
    }
}

pub fn verify(public_key: &PublicKey, transaction: &Transaction) -> bool {
    if transaction.signature.is_none() {
        return false;
    }

    let secp = Secp256k1::new();
    let hash: [u8; 32] = transaction.hash_without_signature();
    let msg: Message = Message::from_digest(hash);
    let sig: Signature = match Signature::from_str(transaction.signature.as_ref().unwrap().as_str())
    {
        Ok(ok) => ok,
        Err(_) => panic!("Invalid signature format"),
    };

    public_key.verify(&secp, msg, &sig).is_ok()
}
