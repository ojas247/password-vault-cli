use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use rand::{RngCore, SeedableRng};
use zeroize::Zeroize;

pub struct CryptoEngine;

impl CryptoEngine {
    
    /// Stretches a master password into a cryptographically secure 32-byte key using Argon2id
    pub fn derive_key(master_password: &mut String, salt: &[u8]) -> Result<[u8; 32], String> {
        let mut key = [0u8; 32];
        
        let salt_str = SaltString::encode_b64(salt)
            .map_err(|e| format!("Failed to encode salt: {}", e))?;

        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(master_password.as_bytes(), &salt_str)
            .map_err(|e| format!("Argon2 hashing failed: {}", e))?;

        // Fix for E0599 compilation error: extract key bytes directly via .hash field
        if let Some(output) = hash.hash {
            let bytes = output.as_bytes();
            if bytes.len() >= 32 {
                key.copy_from_slice(&bytes[..32]);
            } else {
                return Err("Derived hash output is too short for a 256-bit key".to_string());
            }
        } else {
            return Err("Failed to extract derived key bytes".to_string());
        }

        // Wipe the master password string from memory immediately after derivation
        master_password.zeroize();

        Ok(key)
    }



    /// Encrypts plaintext data using AES-256-GCM
    pub fn encrypt(plaintext: &[u8], key: &[u8; 32]) -> Result<(Vec<u8>, [u8; 12]), String> {
        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|e| format!("Cipher initialization failed: {}", e))?;

        // Generate a cryptographically secure random 12-byte nonce (initialization vector)
        let mut nonce_bytes = [0u8; 12];
        rand::rngs::StdRng::from_entropy().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| format!("Encryption failed: {}", e))?;

        Ok((ciphertext, nonce_bytes))
    }

    /// Decrypts ciphertext data using AES-256-GCM. Fails if tampering is detected.
    pub fn decrypt(ciphertext: &[u8], key: &[u8; 32], nonce_bytes: &[u8; 12]) -> Result<Vec<u8>, String> {
        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|e| format!("Cipher initialization failed: {}", e))?;
        
        let nonce = Nonce::from_slice(nonce_bytes);
        
        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| "Decryption failed. Invalid master password or corrupted file.".to_string())?;

        Ok(plaintext)
    }
}