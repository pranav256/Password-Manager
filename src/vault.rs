use crate::crypto::{self, CryptoError, SALT_LEN};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VaultEntry {
    pub id: String,
    pub service: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub url: String,
    pub notes: String,
    pub category: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl VaultEntry {
    pub fn new(
        service: impl Into<String>,
        username: impl Into<String>,
        email: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        let now = Utc::now();
        let id = format!("{:x}", rand::random::<u128>());
        Self {
            id,
            service: service.into(),
            username: username.into(),
            email: email.into(),
            password: password.into(),
            url: String::new(),
            notes: String::new(),
            category: "General".to_string(),
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VaultData {
    pub version: u32,
    pub entries: Vec<VaultEntry>,
}

pub struct Vault {
    pub salt: [u8; SALT_LEN],
    pub entries: Vec<VaultEntry>,
}

#[derive(Debug)]
pub enum VaultError {
    Crypto(CryptoError),
    Io(io::Error),
    Serialization(serde_json::Error),
    InvalidSalt,
}

impl From<CryptoError> for VaultError {
    fn from(e: CryptoError) -> Self {
        VaultError::Crypto(e)
    }
}

impl From<io::Error> for VaultError {
    fn from(e: io::Error) -> Self {
        VaultError::Io(e)
    }
}

impl From<serde_json::Error> for VaultError {
    fn from(e: serde_json::Error) -> Self {
        VaultError::Serialization(e)
    }
}

impl std::fmt::Display for VaultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VaultError::Crypto(e) => write!(f, "Crypto error: {}", e),
            VaultError::Io(e) => write!(f, "I/O error: {}", e),
            VaultError::Serialization(e) => write!(f, "Serialization error: {}", e),
            VaultError::InvalidSalt => write!(f, "Corrupted salt in vault file"),
        }
    }
}

impl std::error::Error for VaultError {}

impl Vault {
    /// Create a new empty vault with a fresh random salt
    pub fn new() -> Self {
        Self {
            salt: crypto::generate_salt(),
            entries: Vec::new(),
        }
    }

    /// Add a new entry to the vault
    pub fn add_entry(&mut self, entry: VaultEntry) {
        self.entries.push(entry);
    }

    /// Remove an entry by ID
    pub fn remove_entry(&mut self, id: &str) -> bool {
        let initial_len = self.entries.len();
        self.entries.retain(|e| e.id != id);
        self.entries.len() < initial_len
    }

    /// Update an existing entry
    pub fn update_entry(&mut self, updated: VaultEntry) -> bool {
        if let Some(existing) = self.entries.iter_mut().find(|e| e.id == updated.id) {
            *existing = updated;
            existing.updated_at = Utc::now();
            true
        } else {
            false
        }
    }
}

impl Default for Vault {
    fn default() -> Self {
        Self::new()
    }
}
