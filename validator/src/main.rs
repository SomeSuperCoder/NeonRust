pub mod id;
pub mod epoch;
pub mod block_votes;
pub mod tx_pool;

use base::{
    blockchain::Blockchain,
    block::{Block, BlockData},
    transaction::Transaction
};
use block_votes::BlockVotes;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use rocket::serde::json::Json;
use std::thread;
use std::collections::HashMap;


static tx_pool: Lazy<Mutex<Vec<Transaction>>> = Lazy::new(|| Mutex::new(Vec::new()));
static blockchain: Lazy<Mutex<Blockchain>> = Lazy::new(|| Mutex::new(Blockchain::new()));
static other_nodes: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));
static current_slot: Lazy<Mutex<u128>> = Lazy::new(|| Mutex::new(0));
static current_leader: String = String::new();
static me: String = String::new();
static votes: Lazy<Mutex<HashMap<u128, BlockVotes>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    let main_validator_handle = thread::spawn(main_validator);
    let bg_finalizer_handle = thread::spawn(bg_finalizer);
    thread::sleep(std::time::Duration::from_secs(10));
    rocket::build().mount("/", routes![index, pull_blockchain, add_tx, add_to_node_list, add_block, vote])
}

#[get("/")]
fn index() -> String {
    format!("Neon Validator: {}", me)
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

#[post("/add_tx", data = "<tx>")]
fn add_tx(tx: Json<Transaction>) {
    println!("{:?}", tx);
    let mut tx_pool_access = tx_pool.lock().unwrap();
    tx_pool_access.push(tx.into_inner());
    drop(tx_pool_access);
    println!("{:?}", tx_pool);
}

#[post("/add_block", data = "<block>")]
fn add_block(block: Json<Block>) {
    todo!("Send post request to /vote");
}

#[post("/vote", data = "<block>")]
fn vote(block: Json<Block>) {

}

#[post("/add_to_node_list", data = "<url>")]
async fn add_to_node_list(url: String) -> String {
    // let url_obj: Result<SocketAddr, _> = url.parse();
    let ping_result = ping(url.clone()).await;
    match ping_result {
        Ok(_) => {
            let mut other_nodes_access = other_nodes.lock().unwrap();
            other_nodes_access.push(url);
            drop(other_nodes_access);
            "Ok".to_string()
        },
        Err(error) => error.to_string()
    }
}

async fn ping(url: String) -> Result<(), reqwest::Error> {
    match reqwest::get(url).await {
        Ok(_) => Ok(()),
        Err(error) => Err(error) 
    }
}

fn main_validator() {
    loop {
        if current_leader == me {
            println!("🎉 You are chosen 🎉");
            
            // Create and broadcast a block
            let block = blockchain.lock().unwrap().create_new_block(Vec::new()); // Create
            let block_hash = block.hash.clone();
            let block_height = block.data.height.clone();

            broadcast_block(block).expect("Broadcast error. Check your internet connection"); // Broadcast

            println!("Successfully created and broadcasted block! (height: {}, hash: {})", block_height, block_hash);
        }

        // WARNING: DO NOT WAIT FOR BLOCKS! Rocket will handle this!
        
        // update the slot
        upadte_slot();
    }
}

fn bg_finalizer() {
    loop {
        
    }
}

fn broadcast_block(block: Block) -> Result<(), ()> {
    // todo!("Implement block broadcast");
    Ok(())
}

fn upadte_slot() {
    let mut latest_slot = get_current_slot();
    let mut current_slot_access = current_slot.lock().unwrap();
 
    while latest_slot <= current_slot_access.clone() {
        latest_slot = get_current_slot();
    }

    println!("Current slot: {}", latest_slot.clone());
    *current_slot_access = latest_slot;
}

fn get_current_slot() -> u128 {
    (std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).expect("You time is crazy").as_secs()) as u128
}

fn bc_to_url_post(path: &str, data: String) {
    let mut path = String::from(path);
    for node_address in other_nodes.lock().unwrap().iter() {
        let mut result_url = node_address.clone();

        if !result_url.ends_with("/") {
            result_url.push_str("/")
        }

        if path.starts_with("/") {
            path.remove(0);
        }

        result_url.push_str(path.as_str());

        let client = reqwest::blocking::Client::new();

        let _response = client
            .post(result_url)
            .header("Content-Type", "application/json")
            .body(data.clone())
            .send();
    }
}
