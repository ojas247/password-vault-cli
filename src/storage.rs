use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Clone)]
pub struct PasswordRecord {
    pub encrypted_secret: Vec<u8>,
    pub nonce: [u8; 12],
}

#[derive(Serialize, Deserialize, Default)]
pub struct VaultFile {
    pub salt: [u8; 16],
    pub records: HashMap<String, PasswordRecord>,
}

pub struct StorageManager;

use std::env;
use std::path::PathBuf;

impl StorageManager {
    // 1. Helper function to calculate a permanent, global file path
    fn get_vault_path() -> PathBuf {
        // Looks for C:\Users\YourName on Windows, or /home/yourname on Mac/Linux
        let base_dir = env::var("USERPROFILE")
            .or_else(|_| env::var("HOME"))
            .unwrap_or_else(|_| ".".to_string());
        
        Path::new(&base_dir).join(".secure_vault.json")
    }

    // 2. Update vault_exists to use the global path
    pub fn vault_exists() -> bool {
        Self::get_vault_path().exists()
    }

    // 3. Update load_vault to read from the global path
    pub fn load_vault() -> Result<VaultFile, String> {
        let path = Self::get_vault_path();
        if !path.exists() {
            return Ok(VaultFile::default());
        }

        let mut file = File::open(path).map_err(|e| e.to_string())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|e| e.to_string())?;

        let vault: VaultFile = serde_json::from_str(&contents)
            .map_err(|e| format!("Failed to parse vault file structure: {}", e))?;
        Ok(vault)
    }

    // 4. Update save_vault to write to the global path
    pub fn save_vault(vault: &VaultFile) -> Result<(), String> {
        let path = Self::get_vault_path();
        let serialized = serde_json::to_string_pretty(vault)
            .map_err(|e| format!("Failed to serialize vault: {}", e))?;

        let mut file = File::create(path).map_err(|e| e.to_string())?;
        file.write_all(serialized.as_bytes()).map_err(|e| e.to_string())?;
        Ok(())
    }
}