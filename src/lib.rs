//! Silent Ark - Privacy-Preserving Scaling via BIP 352 and vTXOs
//!
//! This library implements Silent Payments support for the Ark Protocol,
//! enabling static reusable addresses with sender privacy.

pub mod core;
pub mod sender;
pub mod recipient;
pub mod scanner;

// Re-export commonly used types
pub use core::types::{SilentAddress, VTXO};
pub use core::error::SilentArkError;

pub mod prelude {
    //! Prelude module with common imports
    pub use crate::core::types::{SilentAddress, VTXO};
    pub use crate::core::error::SilentArkError;
}
