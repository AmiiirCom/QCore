# QCore Engineering & Coding Standards

This document establishes the official architectural, cryptographic, and stylistic guidelines governing the QCore codebase. All contributions and modifications must strictly comply with these rules.

---

## 1. Rust Language & Toolchain Guidelines

1. **Rust Edition:** All crates within the workspace must strictly compile under **Edition 2021** (or 2024 when finalized in stable toolchains).
2. **Zero-Warning Policy:** Production builds must compile with **zero compiler warnings**. Any unused import, dead-code attribute, or deprecated pattern must be resolved.
3. **Rustdoc Documentation (`///` and `//!`):**
   - Every public module, struct, enum, function, and method must have standard English documentation comments.
   - Private methods containing complex cryptographic or network logic must have inline explanatory comments.
   - No non-English (e.g., Persian) comments, logs, or error strings are permitted in the source tree.
4. **Error Handling Architecture:**
   - Use `thiserror` (v2.0+) for well-typed, domain-specific internal crate errors.
   - Use `anyhow` (v1.0+) in binary application entrypoints (`qcore-server`, `qcore-client`) and CLI handlers.
   - Prohibit bare `.unwrap()` or `.expect()` calls on I/O streams and user inputs. Always propagate errors using `?` or map them to descriptive errors.

---

## 2. Cryptographic Security Standards

1. **Standardized Primitives:**
   - **Post-Quantum Key Exchange:** Strictly adhere to **NIST FIPS 203 (ML-KEM-768)**.
   - **Classical Key Agreement:** Strictly adhere to **RFC 7748 (X25519)**.
   - **Authenticated Encryption:** Strictly adhere to **RFC 8439 (ChaCha20-Poly1305)** with unique, non-repeating 12-byte nonces generated per frame.
   - **Key Combiner:** Constant-time hashing utilizing **BLAKE3 HKDF** with distinct domain separation context strings.
2. **Side-Channel Mitigation:**
   - Avoid data-dependent branching on sensitive cryptographic variables (e.g., private keys, shared secrets, authentication tokens).
   - Sensitive memory buffers must be dropped and wiped promptly using `zeroize` conventions where possible.
3. **No Hardcoded Versions in Cryptographic Contexts:**
   - Cryptographic combiner strings and protocol identifiers must not contain arbitrary version tags (e.g., use `QCORE-HYBRID-KEM-MLKEM768-X25519-HKDF-BLAKE3`).

---

## 3. Network Architecture & Inbound Standards

1. **RFC Protocol Conformance:**
   - **SOCKS5:** Must comply strictly with **RFC 1928** (supporting `NO_AUTH (0x00)` and `CONNECT (0x01)` for IPv4, IPv6, and FQDN domains).
   - **HTTP Proxy:** Must implement standard **RFC 9110** `CONNECT` tunneling.
2. **Socket Resource Management & Lifecycle:**
   - **OS-Level Keep-Alive:** Every active TCP socket must have TCP Keep-Alive configured via `socket2` (60s idle threshold, 10s probe interval) to prevent socket leaks on mobile network drops.
   - **Inactivity Idle Timeouts:** Streaming loops must enforce a 180-second idle inactivity timeout.
   - **Handshake Bounds:** Handshake negotiations must abort if incomplete within 15 seconds.
3. **Stream Ownership:**
   - Never clone asynchronous TCP streams using system tricks. Stream ownership must be transferred cleanly to `TransportEngine::bridge_connections`.

---

## 4. WebAssembly & Cloudflare Worker Guidelines

1. **Standalone Deployment:**
   - The compiled WebAssembly bytecode must be inlined directly into `worker/dist/worker.js` as an embedded binary buffer.
   - No separate `.wasm` file upload is permitted; the worker artifact must be 100% self-contained.
2. **Obfuscation Integrity:**
   - The final worker output must be processed via `javascript-obfuscator` with high-entropy transformation rules (control flow flattening, string encoding with RC4/Base64, dead code injection).
   - The native `cloudflare:sockets` import (`import { connect } from 'cloudflare:sockets';`) must remain an external module reference.
3. **Compiler Flag Synchronization:**
   - WebAssembly compilation must disable `wasm-opt` in `Cargo.toml` (`[package.metadata.wasm-pack.profile.release] wasm-opt = false`) to maintain compatibility with LLVM bulk memory operations.

---

## 5. Git, CI/CD, and Release Conventions

1. **Conventional Commits:**
   - All Git commit messages must follow the format: `<type>(<scope>): <subject>`.
   - Permitted types: `feat`, `fix`, `refactor`, `perf`, `docs`, `chore`, `ci`.
2. **Release Artifact Segregation:**
   - Client and Server packages must always be distributed as independent artifacts.
   - Artifact filenames must explicitly encode the SemVer tag and target architecture (e.g., `qcore-server-v0.1.12-linux-x86_64-musl.tar.gz`).
   - Every released binary must be accompanied by a cryptographic `.sha256` checksum file.