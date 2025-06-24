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

    println!("{}", wallet1.verify(&txs[0]));
    println!("{}", wallet2.verify(&txs[1]));

    check(chain);
}

fn check(chain: Blockchain) {
    for block in &chain.chain {
        println!("{:#?}", block);
        println!("{}", block.data);
    }

    println!("Chain valid - {}\n", chain.is_valid());
}
