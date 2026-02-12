//! Scanning logic for finding Silent Payments in Ark Batch Outputs
//!
//! This module handles scanning the blockchain to identify payments
//! sent to the recipient's Silent Payment address.

use crate::core::{
    crypto::SilentPaymentCrypto,
    error::Result,
    types::{SilentAddress, SilentVTXO, VTXO},
};

/// Silent Payment scanner
///
/// Scans Ark Batch Outputs to find payments intended for the recipient.
pub struct SilentScanner {
    /// The recipient's Silent Payment address
    address: SilentAddress,
    /// The recipient's private scan key
    scan_key: bitcoin::secp256k1::SecretKey,
}

impl SilentScanner {
    /// Create a new SilentScanner
    pub fn new(address: SilentAddress, scan_key: bitcoin::secp256k1::SecretKey) -> Self {
        Self {
            address,
            scan_key,
        }
    }

    /// Scan a single Ark Batch Output for Silent Payments
    ///
    /// For each output in the batch, attempt to derive the shared secret
    /// and check if it belongs to this recipient.
    ///
    /// # Arguments
    /// * `batch_outputs` - Outputs in the Ark Batch transaction
    /// * `input_outpoints` - The input outpoints used in this batch
    ///
    /// # Returns
    /// List of SilentVTXOs that belong to this recipient
    pub fn scan_batch(
        &self,
        batch_outputs: &[VTXO],
        input_outpoints: &[String],
    ) -> Result<Vec<SilentVTXO>> {
        let mut found_vtxos = Vec::new();

        for output in batch_outputs {
            // Try to match this output
            if let Some(vtxo) = self.try_match_output(output, input_outpoints)? {
                found_vtxos.push(vtxo);
            }
        }

        Ok(found_vtxos)
    }

    /// Try to match a single output
    ///
    /// Returns Some if the output belongs to this recipient, None otherwise
    fn try_match_output(
        &self,
        _output: &VTXO,
        _input_outpoints: &[String],
    ) -> Result<Option<SilentVTXO>> {
        // TODO: Implement the scanning logic
        // 1. Get the lexicographically smallest outpoint
        // 2. Derive the shared secret
        // 3. Check if the output address matches
        Ok(None)
    }

    /// Get the lexicographically smallest outpoint
    ///
    /// This is used for deterministic shared secret derivation in BIP 352.
    fn get_smallest_outpoint(&self, outpoints: &[String]) -> Option<String> {
        outpoints.iter().min().cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_smallest_outpoint() {
        let scanner = SilentScanner {
            address: SilentAddress {
                scan_key: bitcoin::XOnlyPublicKey::from(
                    bitcoin::PublicKey::from_slice(&[2; 33]).unwrap()
                ),
                spend_key: bitcoin::XOnlyPublicKey::from(
                    bitcoin::PublicKey::from_slice(&[3; 33]).unwrap()
                ),
            },
            scan_key: bitcoin::secp256k1::SecretKey::from_slice(&[1; 32]).unwrap(),
        };

        let outpoints = vec![
            "outpoint_c".to_string(),
            "outpoint_a".to_string(),
            "outpoint_b".to_string(),
        ];

        let smallest = scanner.get_smallest_outpoint(&outpoints);
        assert_eq!(smallest, Some("outpoint_a".to_string()));
    }
}
