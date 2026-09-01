# 🔐 RustyVault — Modern Encrypted Desktop Password Manager

<p align="center">
  <img src="https://img.shields.io/badge/Language-Rust%202021-DEA584?logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/Crypto-AES--256--GCM-blue" alt="AES-256-GCM">
  <img src="https://img.shields.io/badge/KDF-Argon2id-blueviolet" alt="Argon2id">
  <img src="https://img.shields.io/badge/License-MIT-green" alt="MIT License">
</p>

A lightweight, zero-dependency desktop password manager written from the ground up in Rust. Designed as a secure, production-grade successor to Python PyQt5 password managers.

---

## ⚡ Key Highlights

* **Military-Grade Cryptography**:
  * **Argon2id Key Derivation**: High memory-cost key derivation resistant to GPU/ASIC brute-force attacks.
  * **AES-256-GCM Authenticated Encryption**: Complete confidentiality and tamper-proof authentication tags for every vault write.
  * **Zeroize Memory Protection**: Derived master keys and sensitive memory buffers are zeroized (overwritten with zeroes) on drop.
* **Instant Native Desktop GUI**:
  * Powered by `egui` and `eframe` — immediate-mode, hardware-accelerated, high FPS.
  * No Python runtime, no Qt libraries, zero bloated webview/Electron wrappers.

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
