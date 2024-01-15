use base::{blockchain::{Blockchain, self}, block::{Block, self}, history::{History, self}, transaction::{Transaction, SenderPart, ValidatorPart}, mutable_storage::MutableStorage};
use rocket::State;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use rocket::serde::json::Json;

static tx_pool: Lazy<Mutex<Vec<Transaction>>> = Lazy::new(|| Mutex::new(Vec::new()));
static blockchain: Lazy<Mutex<Blockchain>> = Lazy::new(|| Mutex::new(Blockchain::new()));

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    // let mut history = History::new();
    // let tx = Transaction::default();
    // let new_part = history.create_new_part(Some(tx.clone()));
    // history.add_part(new_part);
    // let new_part = history.create_new_part(Some(tx.clone()));
    // history.add_part(new_part);
    println!("{:?}", Transaction::default());
    let mut blockchain_access = blockchain.lock().unwrap();
    blockchain_access.add_block(Block::default());
    drop(blockchain_access);
    rocket::build().mount("/", routes![index, pull_blockchain, add_tx])
}

#[get("/")]
fn index() -> &'static str {
    "Neon Validator"
}

#[get("/pull_blockchain/<index>")]
fn pull_blockchain(index: usize) -> String {
    let blockchaion_access = blockchain.lock().unwrap();
    let block = blockchaion_access.get_block(index);
    match block {
        Some(block) => {
            format!("{:?}", block)
        },
        None => "".to_string()
    }
}
// { "sender_part": { "program_id": "", "message_text": "", "public_key": [] }, "validator_part": { "public_key": [] }, "storage":  { "data": {} } }
#[post("/add_tx", data = "<tx>")]
fn add_tx(mut tx:Json<Transaction>) {
    println!("{:?}", tx);
    tx_pool.lock().unwrap().push(tx.into_inner());
    println!("{:?}", tx_pool);
}

/*
Logic

/pull_blockchain - provaides a way to sync with the blockchain
/add_tx - used to add a tx to the pool
/select - only the leader can access this url. It is used to choose transactions to add to the new block
/add_to_node_list - add the sender ip to node list (check if port is opened)

*/
