# rCLI

### PROJECT OVERVIEW
rCLI is a high-performance, zero-knowledge command-line password manager engineered in Rust. Unlike centralized cloud-based password solutions, this tool functions entirely locally, compiling down to a single native binary. It encrypts your credentials at rest using enterprise-grade cryptographic primitives, ensuring your plain-text master password and individual secrets never touch a remote server, log file, or volatile memory block.

***

### CRYPTOGRAPHIC ARCHITECTURE

- **Zero-Knowledge Paradigm**
  The master password is never saved to a database, cached, or verified via direct string comparison. Instead, the application relies entirely on keys derived dynamically at runtime.

- **Argon2id Key Stretching**
  Utilizes the profile memory-hard Argon2id algorithm (winner of the Password Hashing Competition) to stretch your master password and a unique 16-byte local salt into a strong 256-bit encryption key, heavily mitigating GPU-based brute-force attacks.

- **AES-256-GCM Authenticated Encryption**
  All passwords are encrypted utilizing Advanced Encryption Standard in Galois/Counter Mode (AES-GCM). This provides Authenticated Encryption with Associated Data (AEAD); if an attacker modifies even a single bit of your stored data payload, decryption will explicitly fail.

- **Zeroize Memory Hygiene**
  Integrates the `zeroize` crate traits to securely overwrite volatile memory buffers (RAM) containing the plain-text master password or decrypted secrets immediately after they exit execution scope.

***

### SYSTEM FILE LAYOUT

- **`src/cli.rs`**
  Handles native command-line argument parsing and flags using `clap` structure derive macros.

- **`src/crypto.rs`**
  The central cryptographic layout executing Argon2id key derivation, initialization vector (nonce) tracking, and AES-GCM primitives.

- **`src/storage.rs`**
  Manages platform-agnostic file I/O operations, writing the encrypted state directly to a centralized dotfile (`.secure_vault.json`) in the user profile directory.

- **`src/main.rs`**
  The execution engine orchestrating UI command buffers, state hydration, and runtime safety boundaries.

***

### GETTING STARTED

- **Step 1: Clone the Repository**
  - `git clone https://github.com/your-username/secure-pass-cli.git`
  - `cd secure-pass-cli`

- **Step 2: Compile the Production Binary**
  - To generate a highly optimized native executable with full compiler optimizations enabled, run:
  - `cargo build --release`

- **Step 3: Move Executable to System PATH (Windows Example)**
  - Move the compiled `.exe` from `.\target\release\secure-pass-cli.exe` to a global folder like `C:\bin\`.
  - Append `C:\bin\` to your Windows System Environment variables to enable global terminal execution.

***

### INTERACTION EXAMPLES

- **A. Store a New Credential Securely**
  ```bash
  $ secure-pass-cli add --service github
  Create/Enter Master Password: ************
  Enter password to save for this service: ************
  Processing cryptographic layout...
  [Success] Entry securely locked for service: 'github'