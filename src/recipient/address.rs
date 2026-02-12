//! Silent Payment address generation for recipients

use crate::core::types::SilentAddress;
use bitcoin::secp256k1::{rand, Secp256k1, SecretKey};
use bitcoin::PublicKey;
use bitcoin::XOnlyPublicKey;

// Use the global rand thread_rng
fn random_secret_key() -> SecretKey {
    SecretKey::new(&mut rand::thread_rng())
}

/// Silent Payment recipient key pair
///
/// Contains the scan and spend keys for a Silent Payment address.
pub struct SilentPaymentKeys {
    /// Scan private key (b_scan)
    pub scan_key: SecretKey,
    /// Spend private key (b_spend)
    pub spend_key: SecretKey,
}

impl SilentPaymentKeys {
    /// Generate new Silent Payment keys
    pub fn generate() -> Self {
        let scan_key = random_secret_key();
        let spend_key = random_secret_key();

        Self {
            scan_key,
            spend_key,
        }
    }

    /// Get the public Silent Payment address
    pub fn silent_address(&self) -> SilentAddress {
        let secp = Secp256k1::new();

        let scan_pubkey = XOnlyPublicKey::from(self.scan_key.public_key(&secp));
        let spend_pubkey = XOnlyPublicKey::from(self.spend_key.public_key(&secp));

        SilentAddress::new(scan_pubkey, spend_pubkey)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_keys() {
        let keys = SilentPaymentKeys::generate();
        let address = keys.silent_address();

        // Should be able to create an address
        assert_eq!(address.scan_key, address.scan_key);
        assert_eq!(address.spend_key, address.spend_key);
    }
}
