//! Post-quantum cryptography module
//!
//! Implements quantum-resistant cryptographic primitives using:
//! - Dilithium for digital signatures
//! - Kyber for key encapsulation

use crate::error::{ManusError, Result};
use pqcrypto_dilithium::dilithium5;
use pqcrypto_kyber::kyber1024;
use pqcrypto_traits::kem::{Ciphertext, PublicKey as KemPublicKey, SecretKey as KemSecretKey, SharedSecret};
use pqcrypto_traits::sign::{PublicKey as SigPublicKey, SecretKey as SigSecretKey, SignedMessage};

/// Dilithium keypair for signatures
pub struct DilithiumKeypair {
    /// Public key
    pub public_key: dilithium5::PublicKey,
    /// Secret key
    pub secret_key: dilithium5::SecretKey,
}

/// Kyber keypair for key encapsulation
pub struct KyberKeypair {
    /// Public key
    pub public_key: kyber1024::PublicKey,
    /// Secret key
    pub secret_key: kyber1024::SecretKey,
}

impl DilithiumKeypair {
    /// Generate a new Dilithium keypair
    pub fn generate() -> Self {
        let (public_key, secret_key) = dilithium5::keypair();
        Self {
            public_key,
            secret_key,
        }
    }

    /// Sign a message
    pub fn sign(&self, message: &[u8]) -> Result<Vec<u8>> {
        let signed = dilithium5::sign(message, &self.secret_key);
        Ok(signed.as_bytes().to_vec())
    }

    /// Verify a signed message
    pub fn verify(public_key: &dilithium5::PublicKey, signed_message: &[u8]) -> Result<Vec<u8>> {
        let signed = dilithium5::SignedMessage::from_bytes(signed_message)
            .map_err(|e| ManusError::Crypto(format!("Invalid signed message: {:?}", e)))?;
        
        let message = dilithium5::open(&signed, public_key)
            .map_err(|e| ManusError::Crypto(format!("Signature verification failed: {:?}", e)))?;
        
        Ok(message.to_vec())
    }
}

impl KyberKeypair {
    /// Generate a new Kyber keypair
    pub fn generate() -> Self {
        let (public_key, secret_key) = kyber1024::keypair();
        Self {
            public_key,
            secret_key,
        }
    }

    /// Encapsulate a shared secret
    pub fn encapsulate(public_key: &kyber1024::PublicKey) -> Result<(Vec<u8>, Vec<u8>)> {
        let (shared_secret, ciphertext) = kyber1024::encapsulate(public_key);
        Ok((shared_secret.as_bytes().to_vec(), ciphertext.as_bytes().to_vec()))
    }

    /// Decapsulate a shared secret
    pub fn decapsulate(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        let ct = kyber1024::Ciphertext::from_bytes(ciphertext)
            .map_err(|e| ManusError::Crypto(format!("Invalid ciphertext: {:?}", e)))?;
        
        let shared_secret = kyber1024::decapsulate(&ct, &self.secret_key);
        Ok(shared_secret.as_bytes().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dilithium_sign_verify() {
        let keypair = DilithiumKeypair::generate();
        let message = b"Hello, quantum-resistant world!";
        
        let signed = keypair.sign(message).unwrap();
        let verified = DilithiumKeypair::verify(&keypair.public_key, &signed).unwrap();
        
        assert_eq!(message.to_vec(), verified);
    }

    #[test]
    fn test_kyber_encapsulate_decapsulate() {
        let keypair = KyberKeypair::generate();
        
        let (shared_secret1, ciphertext) = KyberKeypair::encapsulate(&keypair.public_key).unwrap();
        let shared_secret2 = keypair.decapsulate(&ciphertext).unwrap();
        
        assert_eq!(shared_secret1, shared_secret2);
    }
}

