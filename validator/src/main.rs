use base::{blockchain::{Blockchain, self}, block::{Block, self}, history::{History, self}, transaction::{Transaction, SenderPart, ValidatorPart}, mutable_storage::MutableStorage};
use rocket::State;

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    // let mut history = History::new();
    // let tx = Transaction::default();
    // let new_part = history.create_new_part(Some(tx.clone()));
    // history.add_part(new_part);
    // let new_part = history.create_new_part(Some(tx.clone()));
    // history.add_part(new_part);

    let mut blockchain = Blockchain::new();
    blockchain.add_block(Block::default());

    let mut tx_pool: Vec<Transaction> = Vec::new();

    // println!("{:?}", history);
    rocket::build().mount("/", routes![index, pull_blockchain]).manage(blockchain)
}

#[get("/")]
fn index() -> &'static str {
    "Neon Validator"
}

#[get("/pull_blockchain/<index>")]
fn pull_blockchain(index: usize, blockchain: &State<Blockchain>) -> String {
    println!("{:?}", blockchain);
    let block = blockchain.get_block(index);
    match block {
        Some(block) => {
            format!("{:?}", block)
        },
        None => "".to_string()
    }
}

/*
Logic

/pull_blockchain - provaides a way to sync with the blockchain
/add_tx - used to add a tx to the pool
/select - only the leader can access this url. It is used to choose transactions to add to the new block
/add_to_node_list - add the sender ip to node list (check if port is opened)

*/
