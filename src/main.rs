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

    let wallet1: Wallet = Wallet::new();
    let wallet2: Wallet = Wallet::new();

    let mut txs: Vec<Transaction> = vec![];

    let mut trans1: Transaction = Transaction::new(
        wallet1.address.clone(),
        wallet2.address.clone(),
        1231,
        Some("mm".into()),
    );
    wallet1.sign(&mut trans1);
    txs.push(trans1);

    let mut trans2: Transaction = Transaction::new(
        wallet2.address.clone(),
        wallet1.address.clone(),
        321,
        Some("Some memo".into()),
    );
    wallet2.sign(&mut trans2);
    txs.push(trans2);

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
