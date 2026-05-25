// fn main() {
//     let s1 = String::from("hello");
//     let len = calculate_length(&s1);
//     println!("The length of '{}' is {}.", s1, len);

//     let mut x: i32 = 5;
//     let r: &mut i32 = &mut x;
//     *r += 1;
//     // println!("Value of x: {}", x);
//     println!("Value of r: {}", r);

   
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }
    

mod cli;
mod crypto;
mod storage;

use clap::Parser;
use cli::{Cli, Commands};
use crypto::CryptoEngine;
use rand::{RngCore, SeedableRng};
use storage::{PasswordRecord, StorageManager};

fn get_master_password(prompt: &str) -> Result<String, String> {
    rpassword::prompt_password(prompt)
        .map_err(|e| format!("Failed to safely read input buffer: {}", e))
}

fn main() {
    let args = Cli::parse();

    // 1. Load or initialize our local storage vault
    let mut vault = StorageManager::load_vault().unwrap_or_else(|err| {
        eprintln!("[Fatal Error] {}", err);
        std::process::exit(1);
    });

    // Generate a fresh salt if this is a newly created vault file
    if !StorageManager::vault_exists() {
        let mut new_salt = [0u8; 16];
        rand::rngs::StdRng::from_entropy().fill_bytes(&mut new_salt);
        vault.salt = new_salt;
    }

    match args.command {
        Commands::Add { service } => {
            let mut master_pass = get_master_password("Create/Enter Master Password: ").unwrap();
            let secret_pass = get_master_password("Enter password to save for this service: ").unwrap();

            println!("Processing cryptographic layout...");
            let key = CryptoEngine::derive_key(&mut master_pass, &vault.salt).unwrap();
            let (encrypted, nonce) = CryptoEngine::encrypt(secret_pass.as_bytes(), &key).unwrap();

            vault.records.insert(service.clone(), PasswordRecord { encrypted_secret: encrypted, nonce });
            StorageManager::save_vault(&vault).unwrap();

            println!("[Success] Entry securely locked for service: '{}'", service);
        }
        Commands::Get { service } => {
            if !vault.records.contains_key(&service) {
                println!("[Error] No record found mapping to the service: '{}'", service);
                return;
            }

            let mut master_pass = get_master_password("Enter Master Password: ").unwrap();
            let key = CryptoEngine::derive_key(&mut master_pass, &vault.salt).unwrap();

            let record = vault.records.get(&service).unwrap();
            match CryptoEngine::decrypt(&record.encrypted_secret, &key, &record.nonce) {
                Ok(plaintext_bytes) => {
                    let decrypted_password = String::from_utf8(plaintext_bytes).unwrap();
                    println!("--------------------------------------------------");
                    println!(" Decrypted Password for {}: {}", service, decrypted_password);
                    println!("--------------------------------------------------");
                }
                Err(e) => {
                    eprintln!("[Access Denied] {}", e);
                }
            }
        }
    }
}