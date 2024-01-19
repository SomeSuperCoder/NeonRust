use base::{blockchain::{Blockchain, self}, block::{Block, self}, history::{History, self}, transaction::{Transaction, SenderPart, ValidatorPart}, mutable_storage::MutableStorage};
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use rocket::{serde::json::Json, tokio::task::spawn_blocking, futures::FutureExt};
use std::net::{Ipv4Addr, SocketAddr};
use reqwest::Client;
use std::error::Error;

static tx_pool: Lazy<Mutex<Vec<Transaction>>> = Lazy::new(|| Mutex::new(Vec::new()));
static blockchain: Lazy<Mutex<Blockchain>> = Lazy::new(|| Mutex::new(Blockchain::new()));
static other_nodes: Lazy<Mutex<Vec<SocketAddr>>> = Lazy::new(|| Mutex::new(Vec::new()));


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
    rocket::build().mount("/", routes![index, pull_blockchain, add_tx, add_to_node_list])
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
            format!("{:?}", block) // TODO: do actual serialization
        },
        None => "".to_string()
    }
}

// { "sender_part": { "program_id": "", "message_text": "", "public_key": [] }, "validator_part": { "public_key": [] }, "storage":  { "data": {} } }
#[post("/add_tx", data = "<tx>")]
fn add_tx(tx: Json<Transaction>) {
    println!("{:?}", tx);
    let mut tx_pool_access = tx_pool.lock().unwrap();
    tx_pool_access.push(tx.into_inner());
    drop(tx_pool_access);
    println!("{:?}", tx_pool);
}


#[post("/add_to_node_list", data = "<url>")]
async fn add_to_node_list(url: String) -> String {
    // let url_obj: Result<SocketAddr, _> = url.parse();
    let ping_result = ping(url).await;
    match ping_result {
        Ok(_) => "Ok".to_string(),
        Err(error) => error.to_string()
    }
}

async fn ping(url: String) -> Result<(), reqwest::Error> {
    match reqwest::get(url).await {
        Ok(_) => Ok(()),
        Err(error) => Err(error)
    }
}

/*
Logic

/pull_blockchain - provides a way to sync with the blockchain
/add_tx - used to add a tx to the pool
/select - only the leader can access this url. It is used to choose transactions to add to the new block
/add_to_node_list - add the sender ip to node list (check if port is opened)

*/
