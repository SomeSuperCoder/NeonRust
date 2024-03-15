mod app_args;

use app_args::CliArgs;
use base::account::AccountSkeleton;
use base::instruction::InstrcuctionSekelton;
use base::system_program::system_instruction::SystemInstrusction;
use base::transaction::{Message, Transaction};
use clap::Parser;
use k256::ecdsa::signature::Keypair;
use k256::elliptic_curve::generic_array::GenericArray;
use k256::sha2::digest::Key;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use dirs;
use base::ecdsa::{self, address_to_public_key, public_key_to_address};
use input_py::input::input;
use bs58;
use base::ecdsa::KeyPair;
use reqwest;
use serde_json;
use config;
use borsh;
use rand::{self, Rng};

const CONFIG_PATH: &'static str = "/home/allen/.config/neon_account.json";

fn main() {
    let args = CliArgs::parse();

    match args.command.as_str() {
        "keygen" => {
            create_account()
        },
        "recover" => {
            recover_account()
        },
        "balance" => {
            get_balance()
        },
        "send" => {
            transfer()
        }
        _ => {
            println!("Unknown command: {}", args.command)
        }
    }
}

fn create_account() {
    let seed_phrase = base::ecdsa::generate_seed_phrase();
    let keypair = ecdsa::KeyPair::recover(seed_phrase.clone()).unwrap();
    set_keypair(keypair.clone());
    println!("Secrect recovery phrase: {}", seed_phrase.clone());
    println!("Public key: {}", bs58::encode(
        keypair.public_key.to_sec1_bytes()
    ).into_string());
}

fn recover_account() {
    let seed_phrase = input("Enter you revocery phrase").unwrap();
    if let Some(keypair) = ecdsa::KeyPair::recover(seed_phrase.clone()) {
        set_keypair(keypair.clone());
        println!("Secrect recovery phrase: {}", seed_phrase.clone());
        println!("Public key: {}", public_key_to_address(&*keypair.public_key.to_sec1_bytes()));
    } else {
        println!("Invalid recovery phrase!")
    }
}

fn get_balance() {
    let keypair = get_keypair();
    let pubkey = public_key_to_address(&*keypair.public_key.to_sec1_bytes());
    let resonse = reqwest::blocking::get(format!("http://127.0.0.1:8000/account/{}", pubkey)).expect("Failed to fetch data from rpc");
    let account: Option<base::account::Account> = serde_json::from_str(resonse.text().unwrap().as_str()).unwrap();
    let account = account.unwrap_or_default();
    println!("Balance: {} NEON", account.atoms as f64 / config::NEON_PARTS as f64);
}

fn transfer() {
    let keypair = get_keypair();
    let me = public_key_to_address(&*keypair.public_key.to_sec1_bytes());
    let to = input("To").unwrap();
    if let Err(_) = address_to_public_key(to.clone()) {
        panic!("Invalid receiver address");
    }
    let amount: f64 = input("Amount").unwrap().parse().expect("That is not a number");
    let mut rng = rand::thread_rng();
    let message = Message {
        nonce: rng.gen_range(u128::MIN..u128::MAX).to_string(),
        instruction: InstrcuctionSekelton {
            program_id: "System".to_string(),
            accounts: vec![
                // AccountSkeleton {
                //     pubkey: to,
                //     is_signer: false,
                //     is_writable: true
                // },
                // AccountSkeleton {
                //     pubkey: me,
                //     is_signer: true,
                //     is_writable: false
                // }
            ],
            // data: borsh::to_vec(&SystemInstrusction::Send { amount: amount as u128 * config::NEON_PARTS as u128 }).unwrap()
            data: borsh::to_vec(&SystemInstrusction::HelloWorld).unwrap()
        }
    };
    let sig = keypair.sign(&serde_json::to_string(&message).unwrap()).unwrap().to_bytes().to_vec();
    let tx = Transaction {
        signatures: vec![sig],
        message: message
    };
    let tx = serde_json::to_string(&tx).unwrap();

    let client = reqwest::blocking::Client::new();
    
    let _response = client
        .post("http://127.0.0.1:8000/add_tx")
        .header("Content-Type", "application/json")
        .body(tx)
        .send();
}

fn make_pk_path() -> PathBuf {
    let home_dir = dirs::home_dir().unwrap();

    // Create the path to the file
    let file_path = Path::new(&home_dir).join(CONFIG_PATH);

    file_path
}

fn set_keypair(keypair: KeyPair) {
    let key = keypair.private_key.unwrap();
    let serialized_private_key = key.to_bytes().to_vec();
    let serialized_private_key = serialized_private_key.as_slice();
    fs::write(CONFIG_PATH, serde_json::to_string(&serialized_private_key).unwrap()).unwrap();
}

fn get_keypair() -> KeyPair {
    let data: Vec<u8> = serde_json::from_str(&fs::read_to_string(CONFIG_PATH).unwrap()).unwrap();
    let sk = k256::ecdsa::SigningKey::from_bytes(&GenericArray::clone_from_slice(data.as_slice())).unwrap();

    KeyPair {
        private_key: Some(sk.clone()),
        public_key: *sk.verifying_key()
    }
}
