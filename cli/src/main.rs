mod app_args;

use app_args::CliArgs;
use clap::Parser;
use k256::ecdsa::signature::Keypair;
use k256::elliptic_curve::generic_array::GenericArray;
use k256::sha2::digest::Key;

use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use dirs;
use base::ecdsa::{self, public_key_to_address};
use input_py::input::input;
use bs58;
use base::ecdsa::KeyPair;
use reqwest;
use serde_json;
use config;

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
