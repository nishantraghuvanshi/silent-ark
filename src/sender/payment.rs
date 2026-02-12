//! Payment creation for Silent Ark sender
//!
//! Handles the logic for creating Silent Payments within Ark rounds.

use crate::core::{
    crypto::SilentPaymentCrypto,
    error::{Result, SilentArkError},
    types::{SilentAddress, VTXO},
};
use bitcoin::Address;

/// Silent Payment sender
///
/// Manages creating Silent Payments to Silent Payment addresses.
pub struct SilentSender {
    /// The vTXOs available to spend
    available_vtxos: Vec<VTXO>,
}

impl SilentSender {
    /// Create a new SilentSender
    pub fn new(available_vtxos: Vec<VTXO>) -> Self {
        Self {
            available_vtxos,
        }
    }

    /// Create a Silent Payment to a Silent Payment address
    ///
    /// # Arguments
    /// * `recipient` - The recipient's Silent Payment address
    /// * `amount` - Amount to send in satoshis
    /// * `vtxo_indices` - Which vTXOs to use as inputs
    ///
    /// # Returns
    /// The derived output address to include in the Ark round
    pub fn create_payment(
        &self,
        recipient: &SilentAddress,
        _amount: u64,
        vtxo_indices: Vec<usize>,
    ) -> Result<Address> {
        // Validate vTXO indices
        for idx in &vtxo_indices {
            if *idx >= self.available_vtxos.len() {
                return Err(SilentArkError::InvalidInput(
                    "Invalid vTXO index".to_string(),
                ));
            }
        }

        // Collect private keys from selected vTXOs
        let mut input_keys = Vec::new();
        for idx in &vtxo_indices {
            if let Some(vtxo) = self.available_vtxos.get(*idx) {
                if let Some(key) = &vtxo.private_key {
                    input_keys.push(*key);
                } else {
                    return Err(SilentArkError::VTXO(
                        "vTXO missing private key".to_string(),
                    ));
                }
            }
        }

        // Derive the unique output address using Silent Payments
        let output_address = SilentPaymentCrypto::derive_output_address(
            input_keys.as_slice(),
            &recipient.scan_key,
            &recipient.spend_key,
            bitcoin::Network::Bitcoin,
        )?;

        Ok(output_address)
    }

    /// Get available vTXO balance
    pub fn available_balance(&self) -> u64 {
        self.available_vtxos.iter().map(|v| v.amount).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::secp256k1::{rand, SecretKey};

    #[test]
    fn test_available_balance() {
        let mut vtxos = Vec::new();

        for _ in 0..3 {
            let key = SecretKey::new(&mut rand::thread_rng());
            let secp = bitcoin::secp256k1::Secp256k1::new();
            let pubkey = bitcoin::key::XOnlyPublicKey::from(key.public_key(&secp));
            let vtxo = VTXO {
                outpoint: format!("outpoint_{}", vtxos.len()),
                amount: 1000,
                private_key: Some(key),
                address: bitcoin::Address::p2tr(
                    &secp,
                    pubkey,
                    None,
                    bitcoin::Network::Bitcoin,
                ),
                created_at: 0,
            };
            vtxos.push(vtxo);
        }

        let sender = SilentSender::new(vtxos);
        assert_eq!(sender.available_balance(), 3000);
    }
}
