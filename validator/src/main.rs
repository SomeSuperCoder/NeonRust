pub mod id;
pub mod epoch;
pub mod block_votes;
pub mod tx_pool;
pub mod vote;
pub mod block_voter;
pub mod poa;
pub mod validator_config;

use base::block::Block;
use base::cache::Cache;
use base::ecdsa;
use base::{
    blockchain::Blockchain,
    transaction::Transaction,
    ecdsa::KeyPair,
    runtime::Runtime
};
use block_voter::BlockVoter;
use std::sync::RwLock;
use validator_config::ValidatorConfig;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use rocket::serde::json::Json;
use std::thread;
use crate::vote::Vote;
use config;
use base::account::Account;
use std::collections::HashSet;
use poa::PoA;

static tx_pool: Lazy<Mutex<HashSet<Transaction>>> = Lazy::new(|| Mutex::new(HashSet::new()));
static blockchain: Lazy<RwLock<Blockchain>> = Lazy::new(|| RwLock::new(Blockchain::load()));
static other_nodes: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(vec!["127.0.0.1:8000".to_string()]));
static current_slot: Lazy<Mutex<u128>> = Lazy::new(|| Mutex::new(0));
static block_voter: Lazy<Mutex<BlockVoter>> = Lazy::new(|| Mutex::new(BlockVoter::new()));
static my_key_pair: Lazy<KeyPair> = Lazy::new(|| {KeyPair::recover(String::from("bronze major hair ranch level arrange coach engine reveal economy fragile lemon")).unwrap()});
static me: Lazy<String> = Lazy::new(|| {ecdsa::public_key_to_address(&*my_key_pair.public_key.to_sec1_bytes())});
static runtime: Lazy<RwLock<Runtime>> = Lazy::new(|| {RwLock::new(Runtime::default())});
static runtime_locks: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| {Mutex::new(HashSet::new())});
static validator_config_var: Lazy<ValidatorConfig> = Lazy::new(|| {ValidatorConfig::load()});
static download_process: Lazy<Mutex<bool>> = Lazy::new(|| { Mutex::new( false ) });

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    println!("I am: {}", *me);
    // Handle genesis account
    if let Some(_) = base::cache::Cache::default().get_owned_account(&String::from(config::GENESIS_PUBKEY)) {} else {
        let account = Account {
            pubkey: String::from(config::GENESIS_PUBKEY),
            owner: String::from(config::SYSTEM_PROGRAM_ADDRESS),
            admin: true,
            atoms: 10_000 * config::NEON_PARTS as u128,
            authority: 0,
            executable: false,
            data: Vec::new()
        }; 
        base::cache::Cache::default().set_account(account);
    }
    // ======================
    Blockchain::load();
    let main_validator_handle = thread::spawn(main_validator);
    let bg_finalizer_handle = thread::spawn(bg_finalizer);
    
    if validator_config_var.pull_from.as_str() != "" {
        thread::spawn(redownload);
    }
    
    rocket::build().mount("/", routes![index, pull_blockchain, add_tx, vote_url, get_account])

}

#[get("/")]
fn index() -> String {
    format!("Neon Validator: {}", *me)
}

#[get("/pull_blockchain/<index>")]
fn pull_blockchain(index: usize) -> String {
    let blockchaion_access = blockchain.read().unwrap();
    let block = blockchaion_access.get_block(index as u128);

    match block {
        Some(block) => {
            serde_json::to_string(&block).unwrap()
        },
        None => "".to_string()
    }
}

#[post("/add_tx", data = "<tx>")]
fn add_tx(tx: Json<Transaction>) -> &'static str {
    let tx = tx.into_inner();
    if tx_pool.lock().unwrap().insert(tx.clone()) {
        thread::spawn(move || {
            bc_to_url_post("/add_tx", serde_json::to_string(&tx).unwrap())
        });
    }

    ""
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
            let slot = current_slot.lock().unwrap().clone();
            let slot_range: Vec<u128> = (slot-5..slot+1).collect();

            thread::spawn(move || {
                while runtime_locks.lock().unwrap().len() != 0 {}

                if !vote.block.valid_for(&blockchain.read().unwrap(), &runtime.read().unwrap().invoke_handler.read().unwrap().cache, slot_range) {
                    println!("Invalid block");
                    return "Invalid block"
                } else {
                    let mut block_voter_access = block_voter.lock().unwrap();
                    let did_actually_vote = block_voter_access.vote(vote.clone());
                    drop(block_voter_access);
                    
                    let my_vote = vote.agree(&my_key_pair);
                
                    if did_actually_vote {
                        thread::spawn(move || {
                            bc_to_url_post("vote", serde_json::to_string(&my_vote).expect("You just created an unserializable vote! Wierd..."))
                        });
                    };
                    return ""
                };
            });

            return ""
        } else {
            println!("Error loading public key");
            "Error loading public key"
        }
    } else {
        println!("Error loading public key");
        "Error loading public key"
    }
}

#[get("/account/<pubkey>")]
fn get_account(pubkey: String) -> String {
    serde_json::to_string(&base::cache::Cache::default().get_owned_account(&pubkey)).unwrap()
}

fn main_validator() {
    upadte_slot(); // Wait for a full slot #1
    upadte_slot(); // Wait for a full slot #2

    loop {
        if PoA::next() == *me {
            println!("🎉 You are chosen 🎉");
            
            // Create and broadcast a block
            let mut tx_poll_access = tx_pool.lock().unwrap();
            let tx_list = tx_poll_access.clone();
            tx_poll_access.clear();
            drop(tx_poll_access);
            let block = blockchain.read().unwrap().create_new_block(tx_list.iter().cloned().collect(), current_slot.lock().unwrap().clone()); // Create
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
        let sleep_time = std::time::Duration::from_millis(10);
        thread::sleep(sleep_time);

        let blockchain_access = blockchain.read().unwrap();
        let mut block_voter_access = block_voter.lock().unwrap();
        let block = block_voter_access.result_for(
            blockchain_access.get_latest_block_height() + 1,
        1);

        block_voter_access.filter(&blockchain_access);

        match block {
            Some(block) => {
                *download_process.lock().unwrap() = false;

                println!("Adding block");

                let runtime_access = runtime.read().unwrap();
                if block.valid_for(&blockchain_access, &runtime_access.invoke_handler.read().unwrap().cache, (u128::MIN..u128::MAX).collect()) {
                    let mut blockchain_access = blockchain.write().unwrap();

                    blockchain_access.add_block(block.clone());
                }

                thread::spawn(
                    move || {
                        let runtime_access = runtime.write().unwrap();
                        runtime_locks.lock().unwrap().insert(block.hash.clone());
                        let handles = Runtime::feed_tx_list(&runtime_access, block.data.seq.clone());
                        for handle in handles {
                            handle.join().unwrap();
                        }
                        runtime_locks.lock().unwrap().remove(&block.hash.clone());
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

pub fn redownload() {
    *download_process.lock().unwrap() = true;

    let mut block_index: u128 = blockchain.read().unwrap().get_latest_block_height() + 1;
    
    loop {
        if !*download_process.lock().unwrap() {
            return;
        }
        // Pull a block
        let mut block: String;

        loop {
            if !*download_process.lock().unwrap() {
                return;
            }
            block = reqwest::blocking::get(
                format!("{}/pull_blockchain/{}", validator_config_var.pull_from, block_index.clone())
            ).unwrap().text().unwrap();

            if block != "" {
                break;
            }

            std::thread::sleep(std::time::Duration::from_millis((config::SLOT_LENGTH * (1000 as f64)) as u64));
        }

        let block: Block = serde_json::from_str(block.as_str()).unwrap();

        if block.valid_for(&blockchain.read().unwrap(), &Cache::default(), (u128::MIN..u128::MAX).collect()) {
            blockchain.write().unwrap().add_block(block.clone());
            for handle in runtime.write().unwrap().feed_tx_list(block.data.seq) {
                handle.join().unwrap();
            }
        } else {
            panic!("The node you provided in the config gave an invalid block");
        }

        block_index += 1;
    }
}
