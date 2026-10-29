//! # QCore Cryptographic Primitives
//!
//! Provides hybrid post-quantum key encapsulation mechanisms (NIST FIPS 203 ML-KEM-768
//! combined with X25519 via HKDF-BLAKE3) and secure authenticated payload serialization.

pub mod hybrid_kem;
pub mod share_link;

pub use hybrid_kem::*;
pub use share_link::*;