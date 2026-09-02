use aes_gcm::aead::OsRng;
use argon2::Argon2;
use rand::RngCore;
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
