//! QCore WebAssembly Core for Cloudflare Workers.
//!
//! Exposes cryptographic handshake evaluation, post-quantum decapsulation (ML-KEM-768),
//! and session authentication functions compiled directly from Rust to WebAssembly.

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use ml_kem::kem::DecapsulationKey;
use ml_kem::{Encoded, EncodedSizeUser, MlKem768Params};
use qcore_crypto::hybrid_kem::HybridServerKEM;
use wasm_bindgen::prelude::*;
use x25519_dalek::StaticSecret;

/// Structured result returned from the Rust WebAssembly core to the Cloudflare Worker.
#[wasm_bindgen]
pub struct HandshakeResult {
    success: bool,
    host: String,
    port: u16,
    error: String,
}

#[wasm_bindgen]
impl HandshakeResult {
    #[wasm_bindgen(getter)]
    pub fn success(&self) -> bool {
        self.success
    }

    #[wasm_bindgen(getter)]
    pub fn host(&self) -> String {
        self.host.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn port(&self) -> u16 {
        self.port
    }

    #[wasm_bindgen(getter)]
    pub fn error(&self) -> String {
        self.error.clone()
    }
}

/// Evaluates a raw incoming QCore handshake frame using the Rust cryptographic engine.
///
/// # Arguments
/// * `frame_bytes` - Raw binary slice received from the client WebSocket.
/// * `classic_secret_b64` - Base64-encoded server X25519 static private key.
/// * `pq_decaps_key_b64` - Base64-encoded server ML-KEM-768 decapsulation key.
/// * `expected_token` - Master authentication token.
#[wasm_bindgen]
pub fn verify_and_decrypt_handshake(
    frame_bytes: &[u8],
    classic_secret_b64: &str,
    pq_decaps_key_b64: &str,
    expected_token: &str,
) -> HandshakeResult {
    // 1. Validate magic header: b"QC\x01"
    if frame_bytes.len() < 3 || &frame_bytes[0..3] != b"QC\x01" {
        return HandshakeResult {
            success: false,
            host: String::new(),
            port: 0,
            error: "Invalid protocol magic identifier".into(),
        };
    }

    let mut cursor = 3;

    // 2. Read client classical public key (32 bytes)
    if frame_bytes.len() < cursor + 32 {
        return HandshakeResult {
            success: false,
            host: String::new(),
            port: 0,
            error: "Frame truncated at client classical key".into(),
        };
    }
    let mut client_classic = [0u8; 32];
    client_classic.copy_from_slice(&frame_bytes[cursor..cursor + 32]);
    cursor += 32;

    // 3. Read post-quantum ciphertext length and payload
    if frame_bytes.len() < cursor + 2 {
        return HandshakeResult {
            success: false,
            host: String::new(),
            port: 0,
            error: "Frame truncated at PQ length header".into(),
        };
    }
    let pq_len = u16::from_be_bytes([frame_bytes[cursor], frame_bytes[cursor + 1]]) as usize;
    cursor += 2;

    if frame_bytes.len() < cursor + pq_len {
        return HandshakeResult {
            success: false,
            host: String::new(),
            port: 0,
            error: "Frame truncated at PQ ciphertext body".into(),
        };
    }
    let pq_ct = &frame_bytes[cursor..cursor + pq_len];
    cursor += pq_len;

    // 4. Restore server private keys
    let classic_secret_bytes = match STANDARD.decode(classic_secret_b64) {
        Ok(b) => b,
        Err(_) => return HandshakeResult { success: false, host: String::new(), port: 0, error: "Malformed server classic key".into() },
    };
    let classic_secret = match <[u8; 32]>::try_from(classic_secret_bytes.as_slice()) {
        Ok(arr) => StaticSecret::from(arr),
        Err(_) => return HandshakeResult { success: false, host: String::new(), port: 0, error: "Invalid classic key length".into() },
    };

    let dk_bytes = match STANDARD.decode(pq_decaps_key_b64) {
        Ok(b) => b,
        Err(_) => return HandshakeResult { success: false, host: String::new(), port: 0, error: "Malformed server PQ key".into() },
    };
    let encoded_dk = match Encoded::<DecapsulationKey<MlKem768Params>>::try_from(dk_bytes.as_slice()) {
        Ok(enc) => enc,
        Err(_) => return HandshakeResult { success: false, host: String::new(), port: 0, error: "Invalid PQ key format".into() },
    };
    let pq_decaps_key = DecapsulationKey::<MlKem768Params>::from_bytes(&encoded_dk);

    // 5. Decapsulate hybrid master secret
    let shared_secret = match HybridServerKEM::decapsulate(
        &classic_secret,
        &pq_decaps_key,
        &client_classic,
        pq_ct,
    ) {
        Ok(s) => s,
        Err(_) => return HandshakeResult { success: false, host: String::new(), port: 0, error: "ML-KEM decapsulation failed".into() },
    };

    // 6. Decrypt authenticated session metadata
    if frame_bytes.len() < cursor + 12 + 2 {
        return HandshakeResult { success: false, host: String::new(), port: 0, error: "Missing metadata header".into() };
    }
    let nonce_bytes = &frame_bytes[cursor..cursor + 12];
    cursor += 12;

    let cipher_len = u16::from_be_bytes([frame_bytes[cursor], frame_bytes[cursor + 1]]) as usize;
    cursor += 2;

    if frame_bytes.len() < cursor + cipher_len {
        return HandshakeResult { success: false, host: String::new(), port: 0, error: "Truncated metadata payload".into() };
    }
    let ciphertext = &frame_bytes[cursor..cursor + cipher_len];

    let cipher = match ChaCha20Poly1305::new_from_slice(&shared_secret) {
        Ok(c) => c,
        Err(_) => return HandshakeResult { success: false, host: String::new(), port: 0, error: "Cipher initialization error".into() },
    };

    let decrypted = match cipher.decrypt(Nonce::from_slice(nonce_bytes), ciphertext) {
        Ok(d) => d,
        Err(_) => return HandshakeResult { success: false, host: String::new(), port: 0, error: "Decryption authentication failed".into() },
    };

    let meta: serde_json::Value = match serde_json::from_slice(&decrypted) {
        Ok(v) => v,
        Err(_) => return HandshakeResult { success: false, host: String::new(), port: 0, error: "Corrupted metadata JSON".into() },
    };

    let token = meta["token"].as_str().unwrap_or("");
    if token != expected_token {
        return HandshakeResult { success: false, host: String::new(), port: 0, error: "Unauthorized user token".into() };
    }

    let host = meta["host"].as_str().unwrap_or("").to_string();
    let port = meta["port"].as_u64().unwrap_or(0) as u16;

    HandshakeResult {
        success: true,
        host,
        port,
        error: String::new(),
    }
}