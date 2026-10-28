use anyhow::{anyhow, Result};
use blake3::Hasher;
use ml_kem::{
    kem::{Decapsulate, DecapsulationKey, Encapsulate, EncapsulationKey},
    Encoded, EncodedSizeUser, KemCore, MlKem768, MlKem768Params,
};
use rand::{thread_rng, RngCore};
use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret};

pub struct HybridKeyPair {
    pub classic_secret: StaticSecret,
    pub classic_public: X25519PublicKey,
    pub pq_decapsulation_key: DecapsulationKey<MlKem768Params>,
    pub pq_public_bytes: Vec<u8>,
}

impl HybridKeyPair {
    pub fn generate() -> Self {
        let mut rng = thread_rng();

        // 1. کلید کلاسیک X25519
        let mut classic_bytes = [0u8; 32];
        rng.fill_bytes(&mut classic_bytes);
        let classic_secret = StaticSecret::from(classic_bytes);
        let classic_public = X25519PublicKey::from(&classic_secret);

        // 2. کلید پسا-کوانتوم ML-KEM-768
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

pub struct HybridClientKEM;

impl HybridClientKEM {
    /// کپسوله‌سازی در سمت کلاینت
    pub fn encapsulate(
        peer_classic_pub: &[u8; 32],
        peer_pq_pub_bytes: &[u8],
    ) -> Result<(Vec<u8>, Vec<u8>, [u8; 32])> {
        let mut rng = thread_rng();

        let mut ephemeral_bytes = [0u8; 32];
        rng.fill_bytes(&mut ephemeral_bytes);
        let my_secret = StaticSecret::from(ephemeral_bytes);
        let my_classic_public = X25519PublicKey::from(&my_secret);
        let peer_x25519 = X25519PublicKey::from(*peer_classic_pub);
        let ss_classic = my_secret.diffie_hellman(&peer_x25519);

        let encoded_ek = Encoded::<EncapsulationKey<MlKem768Params>>::try_from(peer_pq_pub_bytes)
            .map_err(|e| anyhow!("Invalid ML-KEM-768 public key length: {e:?}"))?;
        let encaps_key = EncapsulationKey::<MlKem768Params>::from_bytes(&encoded_ek);

        let (pq_ciphertext, ss_pq) = encaps_key
            .encapsulate(&mut rng)
            .map_err(|_| anyhow!("ML-KEM encapsulation failed"))?;

        let mut hasher = Hasher::new();
        hasher.update(b"QCORE_HYBRID_KEM_v5_COMBINER");
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

pub struct HybridServerKEM;

impl HybridServerKEM {
    /// دکپسوله‌سازی در سمت سرور
    pub fn decapsulate(
        classic_secret: &StaticSecret,
        pq_decaps_key: &DecapsulationKey<MlKem768Params>,
        client_classic_pub: &[u8; 32],
        pq_ciphertext_bytes: &[u8],
    ) -> Result<[u8; 32]> {
        let client_x25519 = X25519PublicKey::from(*client_classic_pub);
        let ss_classic = classic_secret.diffie_hellman(&client_x25519);

        let ct = ml_kem::Ciphertext::<MlKem768>::try_from(pq_ciphertext_bytes)
            .map_err(|e| anyhow!("Invalid ML-KEM ciphertext: {e:?}"))?;
        
        // استخراج مقدار با map_err از خروجی Result
        let ss_pq = pq_decaps_key
            .decapsulate(&ct)
            .map_err(|_| anyhow!("ML-KEM decapsulation failed"))?;

        let mut hasher = Hasher::new();
        hasher.update(b"QCORE_HYBRID_KEM_v5_COMBINER");
        hasher.update(ss_classic.as_bytes());
        hasher.update(ss_pq.as_slice());
        Ok(*hasher.finalize().as_bytes())
    }
}