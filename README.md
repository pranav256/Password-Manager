# 🔐 RustyVault — Modern Encrypted Desktop Password Manager

<p align="center">
  <img src="https://img.shields.io/badge/Language-Rust%202021-DEA584?logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/Crypto-AES--256--GCM-blue" alt="AES-256-GCM">
  <img src="https://img.shields.io/badge/KDF-Argon2id-blueviolet" alt="Argon2id">
  <img src="https://img.shields.io/badge/GUI-egui%20%2F%20eframe-brightgreen" alt="egui">
  <img src="https://img.shields.io/badge/License-MIT-green" alt="MIT License">
</p>

A lightweight desktop password manager written in Rust with Argon2id, AES-256-GCM, and egui.

---

## ⚡ Key Highlights

* **Cryptographic Security**:
  * **Argon2id Key Derivation**: High memory-cost key derivation resistant to GPU/ASIC brute-force attacks.
  * **AES-256-GCM Authenticated Encryption**: Complete confidentiality and tamper-proof authentication tags for every vault write.
  * **Zeroize Memory Protection**: Derived master keys and sensitive memory buffers are zeroized (overwritten with zeroes) on drop.
* **Instant Native Desktop GUI**:
  * Powered by `egui` and `eframe` — immediate-mode, hardware-accelerated, high FPS.
  * No Python runtime, no Qt libraries, zero bloated webview/Electron wrappers.
* **Live Fuzzy Search**:
  * Filter through accounts instantly by service name, email, username, or notes.
* **Built-in Cryptographic Password Generator**:
  * Real-time Shannon entropy calculation (bits).
  * Strength classification (Weak, Fair, Strong, Very Strong).
  * Configurable character rules and ambiguous character filtering (`0/O`, `1/l`).
* **Backward Compatible Migration**:
  * Built-in one-click importer for legacy Python `emails.txt` files (`ord(c) - 5`), smoothly upgrading them into the new encrypted AES-GCM envelope.

---

## 🏗️ Architecture

```text
  ┌────────────────────────────────────────────────────────┐
  │                   RustyVault GUI                       │
  │     (Master Password Input / Vault Explorer / Dialogs) │
  └───────────────────────────┬────────────────────────────┘
                              │
               Master Password + Salt (16-byte CSPRNG)
                              │
                              ▼
  ┌────────────────────────────────────────────────────────┐
  │                 Argon2id Key Derivation                │
  │           Derives 256-bit Key with Zeroize             │
  └───────────────────────────┬────────────────────────────┘
                              │
                              ▼
  ┌────────────────────────────────────────────────────────┐
  │              AES-256-GCM Authenticated AEAD            │
  │     (12-byte Nonce + 16-byte Poly1305 / GCM Tag)       │
  └───────────────────────────┬────────────────────────────┘
                              │
                              ▼
  ┌────────────────────────────────────────────────────────┐
  │             Encrypted Envelope (vault.enc)             │
  │       Atomic File Write (Safe from power cuts)         │
  └────────────────────────────────────────────────────────┘
```

---

## 🚀 Building & Running

### Prerequisites
* Rust toolchain (`cargo` and `rustc`).

### Run in Development
```bash
cargo run --release
```

### Run Tests
```bash
cargo test
```

---

## 📄 License
MIT License.
