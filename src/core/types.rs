//! Core types for Silent Ark

use bitcoin::{Address, Network, XOnlyPublicKey};
use bitcoin::secp256k1::SecretKey;

/// A Silent Payment address (BIP 352)
///
/// This is a static address that can be reused while maintaining privacy.
/// Format: sp1...
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SilentAddress {
    /// Scan public key (B_scan)
    pub scan_key: XOnlyPublicKey,
    /// Spend public key (B_spend)
    pub spend_key: XOnlyPublicKey,
}

impl SilentAddress {
    /// Create a new Silent Payment address
    pub fn new(scan_key: XOnlyPublicKey, spend_key: XOnlyPublicKey) -> Self {
        Self {
            scan_key,
            spend_key,
        }
    }

    /// Encode as Silent Payment address string (sp1...)
    pub fn encode(&self) -> String {
        // TODO: Implement BIP 352 encoding
        format!("sp1...") // placeholder
    }

    /// Decode from Silent Payment address string
    pub fn decode(_address: &str) -> Result<Self, crate::core::error::SilentArkError> {
        // TODO: Implement BIP 352 decoding
        Err(crate::core::error::SilentArkError::NotImplemented)
    }
}

/// A Virtual TXO (vTXO) in the Ark system
///
/// vTXOs are off-chain claims that can be spent in future rounds.
#[derive(Debug, Clone)]
pub struct VTXO {
    /// The vTXO identifier (outpoint)
    pub outpoint: String,
    /// Amount in satoshis
    pub amount: u64,
    /// The private key controlling this vTXO
    pub private_key: Option<SecretKey>,
    /// The Taproot address for this vTXO
    pub address: Address,
    /// Block height when this vTXO was created (for CSV timelock)
    pub created_at: u32,
}

impl VTXO {
    /// Create a new vTXO
    pub fn new(
        outpoint: String,
        amount: u64,
        private_key: SecretKey,
        address: Address,
        created_at: u32,
    ) -> Self {
        Self {
            outpoint,
            amount,
            private_key: Some(private_key),
            address,
            created_at,
        }
    }

    /// Check if this vTXO can be spent (considering CSV delay)
    pub fn is_spendable(&self, current_height: u32, csv_delay: u32) -> bool {
        current_height >= self.created_at + csv_delay
    }
}

/// A Silent Payment vTXO (derived from Silent Payment address)
///
/// This is a "ghost" vTXO that only the recipient can identify and claim.
#[derive(Debug, Clone)]
pub struct SilentVTXO {
    /// The base vTXO data
    pub vtxo: VTXO,
    /// The shared secret used to derive this vTXO
    pub shared_secret: [u8; 32],
    /// The Silent Payment address this was sent to
    pub recipient_address: SilentAddress,
}

/// An Ark round context
///
/// Contains information about an Ark round for Silent Payment derivation.
#[derive(Debug, Clone)]
pub struct ArkRound {
    /// Round identifier
    pub round_id: u64,
    /// The on-chain transaction outpoint for this round's batch output
    pub batch_outpoint: String,
    /// Block height of this round
    pub height: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vtxo_is_spendable() {
        let vtxo = VTXO {
            outpoint: "test".to_string(),
            amount: 1000,
            private_key: None,
            address: Address::p2tr(
                &bitcoin::secp256k1::Secp256k1::new(),
                XOnlyPublicKey::from_slice(&[2; 32]).unwrap(),
                None,
                Network::Bitcoin,
            ),
            created_at: 100,
        };

        // Not spendable before CSV delay
        assert!(!vtxo.is_spendable(105, 10));

        // Spendable after CSV delay
        assert!(vtxo.is_spendable(110, 10));
    }
}
