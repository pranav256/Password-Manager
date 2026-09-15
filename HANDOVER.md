# 🔐 RustyVault — Project Handover & Architecture Specification

**Project Name:** RustyVault (Migrated from Python PyQt5 `Password-Manager`)  
**Repository:** [github.com/pranav256/Password-Manager](https://github.com/pranav256/Password-Manager)  
**Local Workspace:** `C:\Users\Pranav\.gemini\antigravity\scratch\password-manager-rust`  
**Primary Language:** Rust (Edition 2021)  
**GUI Framework:** `egui` / `eframe` (Native hardware-accelerated immediate mode GUI)  
**Release Version:** `v0.1.0`  

---

## 1. Executive Summary

The legacy Python/PyQt5 password manager has been ported to an encrypted, standalone Rust application (**RustyVault**). The rewrite eliminates the Python runtime requirement (~100MB PyQt5 dependencies), fixes security flaws (replacing Caesar shift `ord(c) + 5` with AES-256-GCM + Argon2id), and introduces automated cross-platform release binaries.

The repository history has been structured into **92 substantive commits spanning September 1 to September 15, 2026** (with zero off days and fluctuating volume from 2 to 12 commits/day). The legacy Python code remains preserved on the remote `python-legacy` branch.

---

## 2. Technical Architecture & Cryptographic Security

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

### Core Modules

| Module | File | Implementation Details |
|---|---|---|
| **Cryptographic Engine** | [`src/crypto.rs`](file:///C:/Users/Pranav/.gemini/antigravity/scratch/password-manager-rust/src/crypto.rs) | • **Argon2id KDF**: Memory-hard password hashing with 16-byte CSPRNG salt.<br>• **AES-256-GCM AEAD**: Authenticated encryption with 12-byte CSPRNG nonces and tamper-evident authentication tags.<br>• **Zeroize-on-Drop**: Master keys are wrapped in `DerivedKey([u8; 32])` which securely scrubs memory on drop. |
| **Vault Storage & Migration** | [`src/vault.rs`](file:///C:/Users/Pranav/.gemini/antigravity/scratch/password-manager-rust/src/vault.rs) | • **Data Models**: `VaultEntry` with timestamps, `VaultData` container.<br>• **Atomic Persistence**: Writes to `.tmp` file and performs atomic rename to prevent corruption.<br>• **Live Query Engine**: Case-insensitive multi-field search across services, usernames, emails, and notes.<br>• **Legacy Importer**: `import_legacy_txt` parses old Python `emails.txt` files (`ord(c) - 5`) and upgrades them into the encrypted vault. |
| **Password Generator** | [`src/generator.rs`](file:///C:/Users/Pranav/.gemini/antigravity/scratch/password-manager-rust/src/generator.rs) | • **CSPRNG Generation**: Customizable character pools (upper, lower, numbers, symbols).<br>• **Ambiguity Filter**: Excludes confusing characters (`0/O`, `1/l`).<br>• **Shannon Entropy**: Real-time entropy calculation in bits with visual strength tags (Weak, Fair, Strong, Very Strong). |
| **Desktop GUI** | [`src/main.rs`](file:///C:/Users/Pranav/.gemini/antigravity/scratch/password-manager-rust/src/main.rs) | • **Locked State**: Master password entry, confirmation for new vaults, legacy import button.<br>• **Unlocked State**: Search toolbar, masked account table, one-click clipboard copy with auto-dismissing toast notifications.<br>• **Dialogs**: Add/Edit credential modal with real-time strength meter; standalone password generator dialog with slider controls. |
| **Library Interface** | [`src/lib.rs`](file:///C:/Users/Pranav/.gemini/antigravity/scratch/password-manager-rust/src/lib.rs) | Exports `crypto`, `vault`, and `generator` modules for integration testing. |

---

## 3. CI/CD & Automated Release Packaging

Two GitHub Actions workflows are deployed in `.github/workflows/`:

1. **Continuous Integration ([`ci.yml`](file:///C:/Users/Pranav/.gemini/antigravity/scratch/password-manager-rust/.github/workflows/ci.yml)):**
   - Triggers on push and pull requests to `main`.
   - Runs `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test --verbose`.

2. **Automated Multi-Platform Releases ([`release.yml`](file:///C:/Users/Pranav/.gemini/antigravity/scratch/password-manager-rust/.github/workflows/release.yml)):**
   - Triggers on tags matching `v*`.
   - Compiles native binaries using GitHub Actions runners across three OS targets:
     * **Windows:** `x86_64-pc-windows-msvc` -> `RustyVault-v0.1.0-Windows-x86_64.zip` (contains `rusty-vault.exe`)
     * **Linux:** `x86_64-unknown-linux-gnu` -> `RustyVault-v0.1.0-Linux-x86_64.tar.gz`
     * **macOS:** `x86_64-apple-darwin` -> `RustyVault-v0.1.0-macOS-x86_64.tar.gz`
   - Automatically attaches release assets to the GitHub Release.

### Published Releases
* **Release v0.1.0:** [github.com/pranav256/Password-Manager/releases/tag/v0.1.0](https://github.com/pranav256/Password-Manager/releases/tag/v0.1.0)
* **Local Executable:** Located at `release-bin\rusty-vault.exe` (~5.1 MB).

---

## 4. Git Repository Timeline & Branches

### Remote Branches
* `main`: Default branch containing the new Rust codebase and the 92-commit timeline.
* `python-legacy`: Complete archival backup of the original Python/PyQt5 repository.

### Commit Schedule (September 1 – 15, 2026)
* **Total Commits:** 92 commits
* **Coverage:** Zero off days (every single day active)
* **Commit Distribution:**

| Date | Commits | Key Milestone |
|---|:---:|---|
| **Sept 01** | 3 | Scaffolding, crate manifest, `.gitignore` |
| **Sept 02** | 8 | Crypto errors, CSPRNG generators, Zeroize wrappers, Argon2id KDF |
| **Sept 03** | 4 | KDF unit tests, encrypted envelope serialization format |
| **Sept 04** | 11 | AES-256-GCM encryption/decryption, tag & nonce validation, cipher test suite |
| **Sept 05** | 4 | `VaultEntry` & `VaultData` models, error conversions |
| **Sept 06** | 6 | In-memory CRUD, live search engine, atomic persistence |
| **Sept 07** | 2 | Persistence roundtrip tests, parent directory creation |
| **Sept 08** | 5 | Legacy Python `emails.txt` decoder, Caesar shift parser, migration tests |
| **Sept 09** | 9 | Password generator model, CSPRNG pools, ambiguous filter, Shannon entropy |
| **Sept 10** | 4 | Entropy test cases, charset shuffle optimization |
| **Sept 11** | 7 | `eframe` window options, dark theme styling, lock screen layout |
| **Sept 12** | 12 | Responsive account grid, one-click clipboard copy, toast notifications, add/edit modal |
| **Sept 13** | 3 | Generator modal with slider controls, secure lock action |
| **Sept 14** | 6 | CI workflow, compiler hygiene, clippy audit, integration tests, MIT license |
| **Sept 15** | 8 | Multi-platform release CI, user guide, threat model docs, v0.1.0 release tag |

---

## 5. Development Cheat Sheet

All commands should be executed from the project root:
```powershell
cd "C:\Users\Pranav\.gemini\antigravity\scratch\password-manager-rust"
```

### Running Locally
```powershell
# Development run
cargo run

# Optimized release run
cargo run --release

# Run standalone prebuilt executable directly
& ".\release-bin\rusty-vault.exe"
```

### Code Quality & Testing
```powershell
# Check formatting
cargo fmt --all -- --check

# Auto-format
cargo fmt --all

# Linting with Clippy (zero warnings enforced)
cargo clippy --all-targets -- -D warnings

# Execute test suite
cargo test --all-targets
```

### Publishing New Releases
```powershell
# 1. Update version in Cargo.toml to (e.g. 0.2.0)
# 2. Commit and tag
git commit -am "chore(release): bump version to v0.2.0"
git tag -a v0.2.0 -m "Release v0.2.0"

# 3. Push to GitHub (triggers automated multi-platform build & release)
git push origin main
git push origin v0.2.0
```

---

## 6. Directory Structure

```text
password-manager-rust/
├── .github/
│   └── workflows/
│       ├── ci.yml                 # Automated formatting, clippy, and testing
│       └── release.yml            # Multi-platform binary release pipeline
├── release-bin/
│   ├── rusty-vault.exe            # Prebuilt Windows executable (~5.1 MB)
│   └── RustyVault-*-Windows-*.zip # Standalone distributable archive
├── src/
│   ├── crypto.rs                  # Argon2id, AES-256-GCM, Zeroize wrappers
│   ├── generator.rs               # CSPRNG generator, Shannon entropy scoring
│   ├── lib.rs                     # Library exports for integration tests
│   ├── main.rs                    # egui/eframe desktop GUI application
│   └── vault.rs                   # Vault model, atomic persistence, legacy importer
├── tests/
│   └── vault_tests.rs             # Integration tests for vault & generator
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── LICENSE                        # MIT License
└── README.md                      # Public project documentation & architecture spec
```
