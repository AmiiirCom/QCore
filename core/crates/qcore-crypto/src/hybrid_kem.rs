//! Hybrid Post-Quantum Key Encapsulation Mechanism (KEM).
//!
//! Complies with NIST FIPS 203 (ML-KEM-768) and RFC 7748 (X25519), using
//! BLAKE3 keyed hashing for constant-time shared secret derivation.

use anyhow::{anyhow, Result};
use blake3::Hasher;
use ml_kem::{
    kem::{Decapsulate, DecapsulationKey, Encapsulate, EncapsulationKey},
    Encoded, EncodedSizeUser, KemCore, MlKem768, MlKem768Params,
};
use rand::{thread_rng, RngCore};
use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret};

/// Domain separation context string for the hybrid key combiner.
const KEM_COMBINER_CONTEXT: &[u8] = b"QCORE-HYBRID-KEM-MLKEM768-X25519-HKDF-BLAKE3";

/// Server-side long-term hybrid keypair holding classical and post-quantum keys.
pub struct HybridKeyPair {
    /// Classical Diffie-Hellman secret key (RFC 7748).
    pub classic_secret: StaticSecret,
    /// Classical Diffie-Hellman public key (32 bytes).
    pub classic_public: X25519PublicKey,
    /// NIST FIPS 203 ML-KEM-768 decapsulation key.
    pub pq_decapsulation_key: DecapsulationKey<MlKem768Params>,
    /// Serialized public encapsulation key bytes for ML-KEM-768.
    pub pq_public_bytes: Vec<u8>,
}

impl HybridKeyPair {
    /// Generates a new random classical and post-quantum hybrid keypair.
    pub fn generate() -> Self {
        let mut rng = thread_rng();

        // Generate RFC 7748 X25519 keypair
        let mut classic_bytes = [0u8; 32];
        rng.fill_bytes(&mut classic_bytes);
        let classic_secret = StaticSecret::from(classic_bytes);
        let classic_public = X25519PublicKey::from(&classic_secret);

        // Generate NIST FIPS 203 ML-KEM-768 keypair
        let (decaps_key, encaps_key): (_, EncapsulationKey<MlKem768Params>) =
            MlKem768::generate(&mut rng);
        let pq_public_bytes = encaps_key.as_bytes().as_slice().to_vec();

        Self {
            classic_secret,
            classic_public,
            pq_decapsulation_key: decaps_key,
            pq_public_bytes,
        }
    }
}

/// Client-side hybrid KEM driver.
pub struct HybridClientKEM;

impl HybridClientKEM {
    /// Encapsulates a shared secret using peer classical and post-quantum public keys.
    ///
    /// # Returns
    /// `(client_ephemeral_public, pq_ciphertext, derived_shared_secret)`
    pub fn encapsulate(
        peer_classic_pub: &[u8; 32],
        peer_pq_pub_bytes: &[u8],
    ) -> Result<(Vec<u8>, Vec<u8>, [u8; 32])> {
        let mut rng = thread_rng();

        // Ephemeral classical Diffie-Hellman
        let mut ephemeral_bytes = [0u8; 32];
        rng.fill_bytes(&mut ephemeral_bytes);
        let my_secret = StaticSecret::from(ephemeral_bytes);
        let my_classic_public = X25519PublicKey::from(&my_secret);
        let peer_x25519 = X25519PublicKey::from(*peer_classic_pub);
        let ss_classic = my_secret.diffie_hellman(&peer_x25519);

        // ML-KEM-768 Encapsulation
        let encoded_ek = Encoded::<EncapsulationKey<MlKem768Params>>::try_from(peer_pq_pub_bytes)
            .map_err(|e| anyhow!("Invalid ML-KEM-768 public key length: {e:?}"))?;
        let encaps_key = EncapsulationKey::<MlKem768Params>::from_bytes(&encoded_ek);

        let (pq_ciphertext, ss_pq) = encaps_key
            .encapsulate(&mut rng)
            .map_err(|_| anyhow!("ML-KEM encapsulation routine failed"))?;

        // Cryptographic combiner via BLAKE3 HKDF
        let mut hasher = Hasher::new();
        hasher.update(KEM_COMBINER_CONTEXT);
        hasher.update(ss_classic.as_bytes());
        hasher.update(ss_pq.as_slice());
        let shared_secret = *hasher.finalize().as_bytes();

        Ok((
            my_classic_public.as_bytes().to_vec(),
            pq_ciphertext.as_slice().to_vec(),
            shared_secret,
        ))
    }
}

/// Server-side hybrid KEM driver.
pub struct HybridServerKEM;

impl HybridServerKEM {
    /// Decapsulates client classical public key and ML-KEM-768 ciphertext.
    ///
    /// # Returns
    /// Derived 32-byte symmetric master shared secret.
    pub fn decapsulate(
        classic_secret: &StaticSecret,
        pq_decaps_key: &DecapsulationKey<MlKem768Params>,
        client_classic_pub: &[u8; 32],
        pq_ciphertext_bytes: &[u8],
    ) -> Result<[u8; 32]> {
        // Classical Diffie-Hellman computation
        let client_x25519 = X25519PublicKey::from(*client_classic_pub);
        let ss_classic = classic_secret.diffie_hellman(&client_x25519);

        // ML-KEM-768 Decapsulation
        let ct = ml_kem::Ciphertext::<MlKem768>::try_from(pq_ciphertext_bytes)
            .map_err(|e| anyhow!("Invalid ML-KEM ciphertext length: {e:?}"))?;

        let ss_pq = pq_decaps_key
            .decapsulate(&ct)
            .map_err(|_| anyhow!("ML-KEM decapsulation failed"))?;

        // Cryptographic combiner via BLAKE3 HKDF
        let mut hasher = Hasher::new();
        hasher.update(KEM_COMBINER_CONTEXT);
        hasher.update(ss_classic.as_bytes());
        hasher.update(ss_pq.as_slice());
        Ok(*hasher.finalize().as_bytes())
    }
}