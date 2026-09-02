use std::fmt;

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
