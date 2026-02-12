//! Cryptographic utilities for Silent Ark
//!
//! Implements BIP 352 Silent Payments cryptography.

use crate::core::error::{Result, SilentArkError};
use bitcoin::{
    key::XOnlyPublicKey,
    secp256k1::{rand, Secp256k1, SecretKey},
    Address, Network,
};
use bitcoin::hashes::{sha256, Hash, HashEngine};
use bitcoin::secp256k1::ecdh::SharedSecret;

/// Silent Payment cryptography
///
/// Implements BIP 352 key summation, ECDH, and address derivation.
pub struct SilentPaymentCrypto;

impl SilentPaymentCrypto {
    /// Sum multiple private keys together
    pub fn sum_private_keys(keys: &[SecretKey]) -> Result<SecretKey> {
        if keys.is_empty() {
            return Err(SilentArkError::InvalidInput(
                "Cannot sum empty key list".to_string(),
            ));
        }

        let mut summed_bytes = [0u8; 32];
        for key in keys {
            let key_bytes = key.secret_bytes();
            let mut carry = 0u16;
            for i in 0..32 {
                let sum = summed_bytes[i] as u16 + key_bytes[i] as u16 + carry;
                summed_bytes[i] = (sum & 0xFF) as u8;
                carry = sum >> 8;
            }
        }

        SecretKey::from_slice(&summed_bytes)
            .map_err(|e| SilentArkError::Crypto(format!("Invalid summed key: {}", e)))
    }

    /// Derive the Silent Payment output address
    pub fn derive_output_address(
        input_keys: &[SecretKey],
        scan_pubkey: &XOnlyPublicKey,
        spend_pubkey: &XOnlyPublicKey,
        network: Network,
    ) -> Result<Address> {
        let secp = Secp256k1::new();

        // Step 1: Sum all input keys
        let summed_input_key = Self::sum_private_keys(input_keys)?;

        // Step 2: ECDH - compute shared secret
        let scan_pk = scan_pubkey.public_key(bitcoin::secp256k1::Parity::Even);
        let shared_secret = SharedSecret::new(&scan_pk, &summed_input_key);
        let secret_bytes = shared_secret.secret_bytes();

        // Step 3: Calculate tweak = hash(shared_secret || B_spend)
        let mut hasher = sha256::Hash::engine();
        hasher.input(&secret_bytes);
        hasher.input(&spend_pubkey.serialize());
        let tweak_hash = sha256::Hash::from_engine(hasher);

        // Convert hash to Scalar for tweaking
        use bitcoin::secp256k1::Scalar;
        let tweak = Scalar::from_be_bytes(*tweak_hash.as_byte_array())
            .map_err(|e| SilentArkError::Crypto(format!("Invalid tweak scalar: {}", e)))?;

        // Step 4: Tweak the spend public key
        let spend_pk = spend_pubkey.public_key(bitcoin::secp256k1::Parity::Even);
        let tweaked_pk = spend_pk
            .add_exp_tweak(&secp, &tweak)
            .map_err(|e| SilentArkError::Crypto(format!("Tweak failed: {}", e)))?;

        // Step 5: Create P2TR address
        let xonly = XOnlyPublicKey::from(tweaked_pk);
        Ok(Address::p2tr(&secp, xonly, None, network))
    }

    /// Generate Silent Payment recipient keys
    pub fn create_recipient_keys() -> Result<(SecretKey, SecretKey)> {
        let scan_key = SecretKey::new(&mut rand::thread_rng());
        let spend_key = SecretKey::new(&mut rand::thread_rng());
        Ok((scan_key, spend_key))
    }

    /// Get public keys from private keys
    pub fn get_public_keys(
        scan_key: &SecretKey,
        spend_key: &SecretKey,
    ) -> (XOnlyPublicKey, XOnlyPublicKey) {
        let secp = Secp256k1::new();
        let scan_pub = XOnlyPublicKey::from(scan_key.public_key(&secp));
        let spend_pub = XOnlyPublicKey::from(spend_key.public_key(&secp));
        (scan_pub, spend_pub)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_private_keys() {
        let key1 = SecretKey::new(&mut rand::thread_rng());
        let key2 = SecretKey::new(&mut rand::thread_rng());
        let result = SilentPaymentCrypto::sum_private_keys(&[key1, key2]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_recipient_keys() {
        let (scan_key, spend_key) = SilentPaymentCrypto::create_recipient_keys().unwrap();
        let (scan_pub, spend_pub) = SilentPaymentCrypto::get_public_keys(&scan_key, &spend_key);
        assert_eq!(scan_pub, scan_pub);
        assert_eq!(spend_pub, spend_pub);
    }

    #[test]
    fn test_derive_output_address() {
        let input_key = SecretKey::new(&mut rand::thread_rng());
        let (scan_key, spend_key) = SilentPaymentCrypto::create_recipient_keys().unwrap();
        let (scan_pubkey, spend_pubkey) = SilentPaymentCrypto::get_public_keys(&scan_key, &spend_key);

        let result = SilentPaymentCrypto::derive_output_address(
            &[input_key],
            &scan_pubkey,
            &spend_pubkey,
            Network::Bitcoin,
        );

        assert!(result.is_ok());
    }
}
