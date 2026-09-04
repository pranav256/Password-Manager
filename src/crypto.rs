use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use argon2::Argon2;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::fmt;
use zeroize::{Zeroize, ZeroizeOnDrop};

pub const SALT_LEN: usize = 16;
pub const NONCE_LEN: usize = 12;
pub const KEY_LEN: usize = 32;

#[derive(Debug)]
pub enum CryptoError {
    KeyDerivationFailed(String),
    EncryptionFailed(String),
    DecryptionFailed,
    InvalidPayload,
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptoError::KeyDerivationFailed(e) => write!(f, "Key derivation error: {}", e),
            CryptoError::EncryptionFailed(e) => write!(f, "Encryption error: {}", e),
            CryptoError::DecryptionFailed => write!(
                f,
                "Decryption failed (incorrect password or corrupted data)"
            ),
            CryptoError::InvalidPayload => write!(f, "Invalid encrypted payload format"),
        }
    }
}

impl std::error::Error for CryptoError {}

/// Wrapper for derived master key that scrubs memory when dropped
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct DerivedKey([u8; KEY_LEN]);

impl DerivedKey {
    pub fn as_bytes(&self) -> &[u8; KEY_LEN] {
        &self.0
    }
}

/// Encrypted container format written to disk
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EncryptedEnvelope {
    pub version: u32,
    pub salt: Vec<u8>,
    pub nonce: Vec<u8>,
    pub ciphertext: Vec<u8>,
}

/// Derive a 256-bit key from master password and salt using Argon2id
pub fn derive_key(password: &str, salt: &[u8]) -> Result<DerivedKey, CryptoError> {
    let mut key = [0u8; KEY_LEN];
    let argon2 = Argon2::default();

    argon2
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|e| CryptoError::KeyDerivationFailed(e.to_string()))?;

    Ok(DerivedKey(key))
}

/// Generate cryptographically secure random bytes
pub fn generate_salt() -> [u8; SALT_LEN] {
    let mut salt = [0u8; SALT_LEN];
    OsRng.fill_bytes(&mut salt);
    salt
}

/// Generate cryptographically secure nonce for AES-GCM
pub fn generate_nonce() -> [u8; NONCE_LEN] {
    let mut nonce = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce);
    nonce
}

/// Encrypt plaintext with derived key using AES-256-GCM
pub fn encrypt(key: &DerivedKey, plaintext: &[u8]) -> Result<EncryptedEnvelope, CryptoError> {
    let cipher = Aes256Gcm::new_from_slice(key.as_bytes())
        .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;

    let nonce_bytes = generate_nonce();
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;

    Ok(EncryptedEnvelope {
        version: 1,
        salt: Vec::new(),
        nonce: nonce_bytes.to_vec(),
        ciphertext,
    })
}

/// Decrypt ciphertext with derived key using AES-256-GCM
pub fn decrypt(key: &DerivedKey, envelope: &EncryptedEnvelope) -> Result<Vec<u8>, CryptoError> {
    if envelope.nonce.len() != NONCE_LEN {
        return Err(CryptoError::InvalidPayload);
    }

    let cipher =
        Aes256Gcm::new_from_slice(key.as_bytes()).map_err(|_| CryptoError::DecryptionFailed)?;

    let nonce = Nonce::from_slice(&envelope.nonce);

    cipher
        .decrypt(nonce, envelope.ciphertext.as_ref())
        .map_err(|_| CryptoError::DecryptionFailed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let password = "super_secret_master_password_123!";
        let salt = generate_salt();
        let key = derive_key(password, &salt).expect("Key derivation should succeed");

        let secret_data = b"{\"user\": \"alice\", \"pass\": \"CorrectHorseBatteryStaple\"}";
        let envelope = encrypt(&key, secret_data).expect("Encryption should succeed");

        let decrypted = decrypt(&key, &envelope).expect("Decryption should succeed");
        assert_eq!(decrypted, secret_data);
    }

    #[test]
    fn test_wrong_password_fails_decryption() {
        let password = "correct_password";
        let wrong_password = "wrong_password";
        let salt = generate_salt();

        let key1 = derive_key(password, &salt).unwrap();
        let key2 = derive_key(wrong_password, &salt).unwrap();

        let secret_data = b"sensitive credentials";
        let envelope = encrypt(&key1, secret_data).unwrap();

        let result = decrypt(&key2, &envelope);
        assert!(result.is_err());
    }
}
