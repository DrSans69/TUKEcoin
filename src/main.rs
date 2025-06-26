mod blockchain;
mod transaction;
mod wallet;

use blockchain::Blockchain;
use transaction::Transaction;
use wallet::Wallet;

use serde_json;

const DIFFICULTY: usize = 1;

fn main() {
    let mut chain = Blockchain::new(DIFFICULTY);

    let mut wallet1: Wallet = Wallet::new();
    let mut wallet2: Wallet = Wallet::new();

    let mut txs: Vec<Transaction> = vec![];

    txs.push(wallet1.create_transaction(wallet2.address.clone(), 123, None));
    txs.push(wallet1.create_transaction(wallet2.address.clone(), 123, None));
    txs.push(wallet2.create_transaction(wallet1.address.clone(), 1222, None));

    chain.add_block(serde_json::to_string_pretty(&txs).unwrap());

    check(chain);
}

fn check(chain: Blockchain) {
    for block in &chain.chain {
        println!("{:#?}", block);
        println!("{}", block.data);

        let txs: Vec<Transaction> = serde_json::from_str(&block.data).unwrap_or(vec![]);
        for tx in txs {
            println!("{:?}", tx);
            println!("{}", tx.verify());
        }
    }

    println!("Chain valid - {}\n", chain.is_valid());
}
