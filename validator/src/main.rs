pub mod id;
pub mod epoch;
pub mod block_votes;
pub mod tx_pool;
pub mod vote;
pub mod block_voter;
pub mod poa;
pub mod validator_config;

use base::block::Block;
use base::ecdsa::{self, TriplePublicKey};
use base::{
    blockchain::Blockchain,
    transaction::Transaction,
    ecdsa::KeyPair,
    runtime::Runtime
};
use k256::pkcs8::der::Encode;
use k256::pkcs8::EncodePublicKey;
use block_voter::BlockVoter;
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

static validator_config: Lazy<ValidatorConfig> = Lazy::new(|| {ValidatorConfig::load()});
static tx_pool: Lazy<Mutex<HashSet<Transaction>>> = Lazy::new(|| Mutex::new(HashSet::new()));
static blockchain: Lazy<Mutex<Blockchain>> = Lazy::new(|| Mutex::new(Blockchain::load()));
static other_nodes: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(vec!["127.0.0.1:8000".to_string()]));
static current_slot: Lazy<Mutex<u128>> = Lazy::new(|| Mutex::new(0));
static block_voter: Lazy<Mutex<BlockVoter>> = Lazy::new(|| Mutex::new(BlockVoter::new()));
static my_key_pair: Lazy<KeyPair> = Lazy::new(|| {KeyPair::recover(String::from("bronze major hair ranch level arrange coach engine reveal economy fragile lemon")).unwrap()});
static me: Lazy<String> = Lazy::new(|| {TriplePublicKey::from_object(my_key_pair.public_key.clone()).unwrap().address});
static runtime: Lazy<Mutex<Runtime>> = Lazy::new(|| {Mutex::new(Runtime::default())});
static runtime_locks: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| {Mutex::new(HashSet::new())});

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    println!("I am: {}", *me);
    // Add all nodes from the config to the node list
    for node in &validator_config.neighbours {
        other_nodes.lock().unwrap().push(node.clone());
    }
    // Handle genesis account
    if let Some(_) = base::cache::Cache::default().get_owned_account(&String::from(config::GENESIS_PUBKEY)) {} else {
        let account = Account {
            pubkey: String::from(config::GENESIS_PUBKEY),
            owner: String::from(config::SYSTEM_PROGRAM_ADDRESS),
            admin: true,
            atoms: 10_000 * config::NEON_PARTS as u128,
            authority: 1,
            executable: false,
            data: Vec::new()
        }; 
        base::cache::Cache::default().set_account(account);
    }
    
    load();

    // ======================
    rocket::build().mount("/", routes![index, pull_blockchain, add_tx, vote_url, get_account])

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

    if let Some(public_key_obj) = TriplePublicKey::from_address(vote.pubkey.clone()) {
        let public_key_bytes = public_key_obj.bytes;
        let public_key = public_key_obj.object;
        let sender_keypair = KeyPair{
            private_key: None,
            public_key
        };
    
        if !vote.verify_sginature(&sender_keypair) {
            println!("Invalid signature");
            return "Invalid signature error"
        }
        let slot = current_slot.lock().unwrap().clone();
        let slot_range = slot-5..slot+1;

        thread::spawn(move || {
            while runtime_locks.lock().unwrap().len() != 0 {}

            if !vote.block.valid_for(&blockchain.lock().unwrap(), &runtime.lock().unwrap().invoke_handler.read().unwrap().cache, slot_range) {
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
            let block = blockchain.lock().unwrap().create_new_block(tx_list.iter().cloned().collect(), current_slot.lock().unwrap().clone()); // Create
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

        let mut blockchain_access = blockchain.lock().unwrap();
        let mut block_voter_access = block_voter.lock().unwrap();
        let block = block_voter_access.result_for(
            blockchain_access.get_latest_block_height() + 1,
        other_nodes.lock().unwrap().len() as u128);

        block_voter_access.filter(&blockchain_access);

        match block {
            Some(block) => {
                println!("Adding block");
                blockchain_access.add_block(block.clone());

                thread::spawn(
                    move || {
                        runtime_locks.lock().unwrap().insert(block.hash.clone());
                        let runtime_access = runtime.lock().unwrap();
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

fn load() {
    if validator_config.pull_from.as_str() == "" {
        load_finish();
        return;
    }

    let mut i = blockchain.lock().unwrap().get_latest_block_height();

    loop {
        let block_text = reqwest::blocking::get(
            format!("{}/pull_blockchain/{}", validator_config.pull_from, i.clone())
        ).unwrap().text().unwrap();

        if block_text.as_str() == "" {
            println!("Blockhain sync with outher nodes done!");
            load_finish();
            return;
        }

        let block: Block = serde_json::from_str(&block_text).unwrap();
        if block.valid_for(
            &blockchain.lock().unwrap(),
        &runtime.lock().unwrap().invoke_handler.read().unwrap().cache,
        u128::MIN..u128::MAX) {
            blockchain.lock().unwrap().add_block(block.clone());
            for handle in runtime.lock().unwrap().feed_tx_list(block.data.seq) {
                handle.join().unwrap();
            }
            println!("Block №{} loaded!", block.data.height);
        } else {
            panic!("Sync node sent an invalid chain!");
        }

        i += 1;
    }
}

fn load_finish() {
    thread::spawn(main_validator);
    thread::spawn(bg_finalizer);
}
