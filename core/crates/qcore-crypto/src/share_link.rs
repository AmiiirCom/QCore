//! Secure Share Link Serialization & SNI Rotation Pooling.

use anyhow::{anyhow, Context, Result};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use rand::{thread_rng, Rng, RngCore};
use serde::{Deserialize, Serialize};

/// Serializable server connection descriptor and cryptographic parameters.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerPayload {
    /// Remote server endpoint in `<host>:<port>` format.
    pub endpoint: String,
    /// SNI camouflage domain or comma-separated pool of domains.
    pub sni: String,
    /// User UUID identifier.
    pub user_uuid: String,
    /// Authentication token.
    pub token: String,
    /// Server classical X25519 public key.
    pub classic_pub: [u8; 32],
    /// Server post-quantum ML-KEM-768 public key bytes.
    pub pq_pub: Vec<u8>,
    /// Underlying transport identifier.
    pub transport: String,
}

impl ServerPayload {
    /// Selects an active camouflage domain from the SNI rotation pool.
    pub fn get_active_sni(&self) -> String {
        let domains: Vec<&str> = self.sni.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
        if domains.is_empty() {
            return "swdist.apple.com".to_string();
        }
        let selected_idx = thread_rng().gen_range(0..domains.len());
        domains[selected_idx].to_string()
    }
}

/// Engine for encoding and decoding encrypted subscription share links.
pub struct SecureLinkEngine;

impl SecureLinkEngine {
    /// Encrypts server payload using password-derived ChaCha20-Poly1305 key.
    pub fn export_symmetric(payload: &ServerPayload, password: &str) -> Result<String> {
        let serialized = serde_json::to_vec(payload)?;
        let mut salt = [0u8; 16];
        let mut nonce_bytes = [0u8; 12];
        let mut rng = thread_rng();
        rng.fill_bytes(&mut salt);
        rng.fill_bytes(&mut nonce_bytes);

        let mut key_deriver = blake3::Hasher::new_derive_key("qcore-symmetric-link-kdf");
        key_deriver.update(&salt);
        key_deriver.update(password.as_bytes());
        let key = key_deriver.finalize();

        let cipher = ChaCha20Poly1305::new_from_slice(key.as_bytes())
            .map_err(|e| anyhow!("Cipher initialization error: {e}"))?;
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = cipher
            .encrypt(nonce, serialized.as_ref())
            .map_err(|e| anyhow!("Payload encryption error: {e}"))?;

        let mut blob = Vec::with_capacity(16 + 12 + ciphertext.len());
        blob.extend_from_slice(&salt);
        blob.extend_from_slice(&nonce_bytes);
        blob.extend_from_slice(&ciphertext);

        Ok(format!("qcore://sym.{}", URL_SAFE_NO_PAD.encode(blob)))
    }

    /// Decrypts a symmetric `qcore://sym.` link using the provided password.
    pub fn import_symmetric(link: &str, password: &str) -> Result<ServerPayload> {
        let stripped = link.trim().strip_prefix("qcore://sym.").context("Invalid link scheme")?;
        let blob = URL_SAFE_NO_PAD.decode(stripped)?;
        if blob.len() < 28 {
            return Err(anyhow!("Payload format error: insufficient length"));
        }

        let salt = &blob[0..16];
        let nonce_bytes = &blob[16..28];
        let ciphertext = &blob[28..];

        let mut key_deriver = blake3::Hasher::new_derive_key("qcore-symmetric-link-kdf");
        key_deriver.update(salt);
        key_deriver.update(password.as_bytes());
        let key = key_deriver.finalize();

        let cipher = ChaCha20Poly1305::new_from_slice(key.as_bytes())
            .map_err(|e| anyhow!("Cipher initialization error: {e}"))?;
        let nonce = Nonce::from_slice(nonce_bytes);
        let decrypted = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| anyhow!("Decryption failed: invalid credentials or corrupt payload"))?;

        let payload = serde_json::from_slice(&decrypted)?;
        Ok(payload)
    }

    /// Encrypts server payload using recipient's static X25519 public key.
    pub fn export_asymmetric(payload: &ServerPayload, recipient_x25519: &[u8; 32]) -> Result<String> {
        let serialized = serde_json::to_vec(payload)?;
        let mut rng = thread_rng();

        let mut ephemeral_bytes = [0u8; 32];
        rng.fill_bytes(&mut ephemeral_bytes);
        let ephemeral = x25519_dalek::StaticSecret::from(ephemeral_bytes);
        let ephemeral_pub = x25519_dalek::PublicKey::from(&ephemeral);
        let target_pub = x25519_dalek::PublicKey::from(*recipient_x25519);
        let ss = ephemeral.diffie_hellman(&target_pub);

        let key = blake3::derive_key("qcore-asymmetric-link-kdf", ss.as_bytes());
        let mut nonce_bytes = [0u8; 12];
        rng.fill_bytes(&mut nonce_bytes);

        let cipher = ChaCha20Poly1305::new_from_slice(&key)
            .map_err(|e| anyhow!("Cipher initialization error: {e}"))?;
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = cipher
            .encrypt(nonce, serialized.as_ref())
            .map_err(|e| anyhow!("Payload encryption error: {e}"))?;

        let mut blob = Vec::with_capacity(32 + 12 + ciphertext.len());
        blob.extend_from_slice(ephemeral_pub.as_bytes());
        blob.extend_from_slice(&nonce_bytes);
        blob.extend_from_slice(&ciphertext);

        Ok(format!("qcore://asym.{}", URL_SAFE_NO_PAD.encode(blob)))
    }

    /// Decrypts an asymmetric `qcore://asym.` link using client's private key.
    pub fn import_asymmetric(link: &str, my_secret: &x25519_dalek::StaticSecret) -> Result<ServerPayload> {
        let stripped = link.trim().strip_prefix("qcore://asym.").context("Invalid link scheme")?;
        let blob = URL_SAFE_NO_PAD.decode(stripped)?;
        if blob.len() < 44 {
            return Err(anyhow!("Payload format error: insufficient length"));
        }

        let mut peer_pub_bytes = [0u8; 32];
        peer_pub_bytes.copy_from_slice(&blob[0..32]);
        let nonce_bytes = &blob[32..44];
        let ciphertext = &blob[44..];

        let peer_pub = x25519_dalek::PublicKey::from(peer_pub_bytes);
        let ss = my_secret.diffie_hellman(&peer_pub);
        let key = blake3::derive_key("qcore-asymmetric-link-kdf", ss.as_bytes());

        let cipher = ChaCha20Poly1305::new_from_slice(&key)
            .map_err(|e| anyhow!("Cipher initialization error: {e}"))?;
        let nonce = Nonce::from_slice(nonce_bytes);
        let decrypted = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| anyhow!("Decryption failed: key mismatch or corrupted link"))?;

        let payload = serde_json::from_slice(&decrypted)?;
        Ok(payload)
    }
}