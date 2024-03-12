pub mod id;
pub mod epoch;
pub mod block_votes;
pub mod tx_pool;
pub mod vote;
pub mod block_voter;

use base::ecdsa;
use base::instruction::InstrcuctionSekelton;
use base::system_program::system_instruction::SystemInstrusction;
use base::transaction::Message;
use base::{
    blockchain::Blockchain,
    block::Block,
    transaction::Transaction,
    ecdsa::KeyPair,
    runtime::Runtime
};
use block_voter::BlockVoter;
use block_votes::BlockVotes;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use rocket::serde::json::Json;
use std::thread;
use std::collections::HashMap;
use crate::vote::Vote;
use std::sync::Arc;
use borsh;

static tx_pool: Lazy<Mutex<Vec<Transaction>>> = Lazy::new(|| Mutex::new(Vec::new()));
static blockchain: Lazy<Mutex<Blockchain>> = Lazy::new(|| Mutex::new(Blockchain::load()));
static other_nodes: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(vec!["127.0.0.1:8000".to_string()]));
static current_slot: Lazy<Mutex<u128>> = Lazy::new(|| Mutex::new(0));
static block_voter: Lazy<Mutex<BlockVoter>> = Lazy::new(|| Mutex::new(BlockVoter::new()));
static my_key_pair: Lazy<KeyPair> = Lazy::new(|| {KeyPair::random()});
static me: Lazy<String> = Lazy::new(|| {ecdsa::public_key_to_address(&*my_key_pair.public_key.to_sec1_bytes())});
static current_leader: Lazy<String> = Lazy::new(|| {me.clone()});
static runtime: Lazy<Mutex<Runtime>> = Lazy::new(|| {Mutex::new(Runtime::default())});

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    Blockchain::load();
    let main_validator_handle = thread::spawn(main_validator);
    let bg_finalizer_handle = thread::spawn(bg_finalizer);
    rocket::build().mount("/", routes![index, pull_blockchain, add_tx, vote_url])

}

#[get("/")]
fn index() -> String {
    format!("Neon Validator: {}", *me)
}

#[get("/pull_blockchain/<index>")]
fn pull_blockchain(index: usize) -> String {
    let blockchaion_access = blockchain.lock().unwrap();
    let block = blockchaion_access.get_block(index as u128);

    match block {
        Some(block) => {
            serde_json::to_string(&block).unwrap()
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

#[post("/vote", data = "<vote>")]
fn vote_url(vote: Json<Vote>) -> &'static str {
    println!("Accepted some vote!");
    println!("Loading vote data...");
    
    let vote: Vote = vote.into_inner();

    if let Ok(public_key_bytes) = ecdsa::address_to_public_key(vote.pubkey.clone()) {
        if let Ok(public_key) = k256::ecdsa::VerifyingKey::from_sec1_bytes(
            public_key_bytes.as_slice()
        ) {
            let sender_keypair = KeyPair{
                private_key: None,
                public_key
            };
        
            if !vote.verify_sginature(&sender_keypair) {
                println!("Invalid signature");
                return "Invalid signature error"
            }
        
            if vote.block.data.height <= blockchain.lock().unwrap().get_latest_block_height() {
                return "";
            }
            // TODO: verify vote
        
            let mut block_voter_access = block_voter.lock().unwrap();
            let did_actually_vote = block_voter_access.vote(vote.clone());
            drop(block_voter_access);
            
            let my_vote = vote.agree(&my_key_pair);
        
            if did_actually_vote {
                thread::spawn(move || {
                    bc_to_url_post("vote", serde_json::to_string(&my_vote).expect("You just created an unserializable vote! Wierd..."))
                });
            }


            ""
        } else {
            println!("Error loading public key");
            "Error loading public key"
        }
    } else {
        println!("Error loading public key");
        "Error loading public key"
    }
}

fn main_validator() {
    upadte_slot(); // Wait for a full slot #1
    upadte_slot(); // Wait for a full slot #2

    loop {
        if *current_leader == *me {
            println!("🎉 You are chosen 🎉");
            
            // Wait for runtime executions to finish

            // Gather valid transactions

            // Create and broadcast a block
            let block = blockchain.lock().unwrap().create_new_block(Vec::new()); // Create
            let block_hash = block.hash.clone();
            let block_height = block.data.height.clone();

            let my_vote = Vote::new(block, &my_key_pair);

            bc_to_url_post("vote", serde_json::to_string(&my_vote).expect("You just created an unserializable vote! Wierd...")); // Broadcast

            println!("Successfully created and broadcasted block! (height: {}, hash: {})", block_height, block_hash);
        }

        // update the slot
        upadte_slot();
    }
}

fn bg_finalizer() {
    loop {
        let mut blockchain_access = blockchain.lock().unwrap();
        let mut block_voter_access = block_voter.lock().unwrap();
        let block = block_voter_access.result_for(
            blockchain_access.get_latest_block_height() + 1,
        1);
        
        block_voter_access.filter(&blockchain_access);

        match block {
            Some(block) => {
                blockchain_access.add_block(block.clone());

                // Create a thread to execute all transactions (inside Add an execution lock to the runtime)
                thread::spawn(
                    move || {
                        // let test_tx = Transaction {
                        //     signatures: Vec::new(),
                        //     message: Message {
                        //         nonce: 0,
                        //         instruction: InstrcuctionSekelton {
                        //             data: borsh::to_vec(&SystemInstrusction::HelloWorld).unwrap(),
                        //             program_id: "System".to_string(),
                        //             ..Default::default()
                        //         }
                        //     }
                        // };
                        let runtime_access = runtime.lock().unwrap();
                        // let lock = runtime_access.lock();
                        let handles = Runtime::feed_tx_list(&runtime_access, block.data.seq.clone());
                        for handle in handles {
                            handle.join().unwrap();
                        }
                        // runtime_access.release(lock);
                    }
                );
            },
            None => {}
        }
    }
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
    let time = (std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).expect("You time is crazy").as_secs()) as u128;

    ((time as f64) / config::SLOT_LENGTH).floor() as u128
}

fn bc_to_url_post(path: &str, data: String) {
    let mut path = String::from(path);
    let other_nodes_access = other_nodes.lock().unwrap();
    let other_node_list = other_nodes_access.clone();
    drop(other_nodes_access);

    for node_address in other_node_list {
        let mut result_url = node_address.clone();

        if !(result_url.starts_with("http://") || result_url.starts_with("https://")) {
            result_url = "http://".to_string() + result_url.as_str();
        }

        if !result_url.ends_with("/") {
            result_url.push_str("/")
        }

        if path.starts_with("/") {
            path.remove(0);
        }

        result_url.push_str(path.as_str());

        let client = reqwest::blocking::Client::new();

        println!("Broadcast to: {}", result_url.clone());
        
        let _response = client
            .post(result_url)
            .header("Content-Type", "application/json")
            .body(data.clone())
            .send();
    }
}
