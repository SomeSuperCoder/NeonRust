mod app_args;

use std::collections::hash_map::Keys;
use app_args::CliArgs;
use clap::Parser;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process;
use dirs;
use base::ecdsa;
use input_py::input::input;
use k256::ecdsa::VerifyingKey;
use bs58;

fn main() {
    let args = CliArgs::parse();

    match args.command.as_str() {
        "keygen" => {
            create_account()
        },
        _ => {
            println!("Unknown command: {}", args.command)
        }
    }
}

fn create_account() {

    // Get the user's home directory
    let home_dir = dirs::home_dir().unwrap();

    // Create the path to the file
    let file_path = Path::new(&home_dir).join(".config/neon_account.json");
    let file = File::open(file_path.clone());

    match file {
        Ok(_) => {
            let agree = input("Overwrite old account? (Y/n)").unwrap();

            match agree.to_lowercase().as_str() {
                "y" | "yes" | "д" | "да" => {},
                _ => {
                    println!("Operation canceled by user");
                    process::exit(0);
                }
            }
        },
        Err(_) => {}
    }
    let keypair = ecdsa::KeyPair::random();
    let mut file = fs::File::create(&file_path).unwrap();
    let key = keypair.private_key.unwrap();
    let serialized_private_key = key.to_bytes();

    // let another_pk = SigningKey::from_bytes(
    //     &serialized_private_key
    // ).unwrap();

    // println!("{:?}", serialized_private_key);
    // println!("{:?}", another_pk.to_bytes());v

    file.write_all(serialized_private_key.as_slice()).unwrap();
    file.flush().unwrap();

    println!("Successfully created new account");
    println!("Public key: {}", bs58::encode(
        keypair.public_key.to_sec1_bytes()
    ).into_string());
}
