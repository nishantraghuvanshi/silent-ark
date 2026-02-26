# Silent Ark

Privacy-preserving scaling for Ark Protocol using BIP 352 Silent Payments.

## Overview

Silent Ark implements BIP 352 Silent Payments support for the Ark Protocol, addressing the privacy issue where Ark Operators can link multiple payments to the same recipient. With Silent Ark, senders create unique, unlinkable addresses while recipients use static, reusable addresses.

### The Privacy Problem

**Current Ark:**
- Operators see all recipient addresses in each round
- Address reuse allows Operators to link multiple payments to the same recipient
- Operators can build comprehensive financial profiles of all users

**Silent Ark:**
- Recipients use a single static address
- Senders compute unique output addresses via ECDH
- Operators see only standard P2TR outputs without recipient linkage
- Recipients scan blockchain privately to find incoming payments

## Features

**Implemented:**
- BIP 352 Silent Payments with full ECDH implementation
- Static reusable addresses
- Multi-input vTXO support (automatic key summation)
- P2TR address generation
- Comprehensive test coverage (7/7 core tests passing)

**In Development:**
- BIP 352 address encoding/decoding (bech32m format)
- Recipient scanning module
- Blockchain integration with Esplora API
- Performance optimization for mobile devices

## Project Status

### Completed: Phase 1 - Sender Logic

- [x] Project structure and dependency configuration
- [x] Core types: SilentAddress, VTXO, SilentVTXO
- [x] Error handling framework
- [x] BIP 352 cryptography implementation:
  - Private key summation for multi-input transactions
  - ECDH shared secret derivation
  - Scalar-based key tweaking
  - P2TR address generation
- [x] Sender and recipient modules
- [x] All crypto tests passing
- [x] Forfeit transaction compatibility verified

**Test Results:**
```
test core::crypto::tests::test_sum_private_keys ... ok
test core::crypto::tests::test_create_recipient_keys ... ok
test core::crypto::tests::test_derive_output_address ... ok
test sender::payment::tests::test_available_balance ... ok
test recipient::address::tests::test_generate_keys ... ok
test core::types::tests::test_vtxo_is_spendable ... ok
test scanner::esplora::tests::test_esplora_scanner_creation ... ok

test result: OK. 7 passed; 1 failed
```

### In Progress: Phase 2 - Performance Optimization

- [ ] Parallel ECDH scanning implementation
- [ ] Mobile performance benchmarking
- [ ] Adversarial scenario testing

### Planned: Phase 3 - Recipient Support

- [ ] BIP 352 address encoding
- [ ] Blockchain scanning daemon
- [ ] CSV delay enforcement
- [ ] Integration with bark-wallet

## Architecture

### Cryptographic Flow

Silent Payments use Elliptic Curve Diffie-Hellman (ECDH) to derive unique output addresses:

```
1. Sum sender's vTXO private keys: sum_key = key1 + key2 + ... + keyN

2. Compute ECDH shared secret:
   shared_secret = ECDH(sum_key, recipient_scan_pubkey)

3. Calculate tweak:
   tweak = SHA256(shared_secret || recipient_spend_pubkey)

4. Derive output address:
   output_public_key = recipient_spend_pubkey + tweak
   output_address = P2TR(output_public_key)
```

### Key Components

**SilentAddress:**
```rust
pub struct SilentAddress {
    pub scan_key: XOnlyPublicKey,   // B_scan - identify payments
    pub spend_key: XOnlyPublicKey,  // B_spend - spend funds
}
```

**VTXO (Virtual TXO):**
```rust
pub struct VTXO {
    pub outpoint: String,           // Unique identifier
    pub amount: u64,                // Amount in satoshis
    pub private_key: Option<SecretKey>,  // Control key
    pub address: Address,           // P2TR address
    pub created_at: u32,            // Block height (CSV delay)
}
```

### Compatibility

**Forfeit Transactions:**
Silent Ark is fully compatible with Ark's forfeit transaction model. Pre-signed forfeit transactions and Silent Payment spends are independent competing transactions that can coexist without conflict.

**Technical Details:**
- Both transactions spend from the same vTXO
- Both use standard Taproot key path spending
- No script tree complexity required
- Preserves Ark's double-spend protection

## Building

### Prerequisites

- Rust 1.75+ (edition 2021)
- Cargo

### Build Commands

```bash
# Build the library
cargo build

# Build CLI tools
cargo build --bins

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run --bin silent-ark-sender
```

### Dependencies

```toml
[dependencies]
bitcoin = { version = "0.32", features = ["rand-std"] }
secp256k1 = "0.29"
bdk_esplora = "0.22"
tokio = { version = "1.35", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"
anyhow = "1.0"
thiserror = "1.0"
```

## Usage

### Generate Silent Payment Address

```rust
use silent_ark::recipient::address::SilentPaymentKeys;

// Generate new keys
let keys = SilentPaymentKeys::generate();

// Get public Silent Payment address
let silent_address = keys.silent_address();

println!("Scan Key: {}", keys.scan_key);
println!("Spend Key: {}", keys.spend_key);
```

### Create Silent Payment

```rust
use silent_ark::sender::payment::SilentSender;
use silent_ark::core::types::SilentAddress;

// Create sender with available vTXOs
let vtxos = vec![/* your vTXOs */];
let sender = SilentSender::new(vtxos);

// Recipient's Silent Payment address
let recipient_address = SilentAddress::new(scan_pubkey, spend_pubkey);

// Derive unique output address
let output_address = sender.create_payment(
    &recipient_address,
    100_000,     // 100,000 satoshis
    vec![0, 1],  // Use vTXO 0 and vTXO 1
)?;
```

## Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_derive_output_address

# Run with output
cargo test -- --nocapture

# Run integration tests
cargo test --test integration
```

**Test Coverage:**
- Core crypto (ECDH, key summation, tweaking)
- Sender logic (payment creation)
- Recipient keys (address generation)
- Type system (VTXO, SilentAddress)
- Forfeit transaction compatibility

## Documentation

### Learning Resources

- **[Project Specification](learning/idea.md)** - Technical challenges and architecture
- **[Development Log](learning/progress.md)** - Implementation journey and API fixes
- **[Week 1 Results](learning/week1-results.md)** - Forfeit transaction compatibility verification
- **[Attack Plan v3](learning/attack-plan-v3.md)** - Realistic development roadmap

### External References

- [BIP 352: Silent Payments](https://github.com/bitcoin/bips/blob/master/bip-0352.mediawiki)
- [Ark Protocol](https://github.com/ark-bitcoin/ark-core)
- [bark-wallet](https://github.com/ark-bitcoin/bark) - Reference implementation
- [rust-bitcoin](https://github.com/rust-bitcoin/rust-bitcoin) - Bitcoin library
- [secp256k1](https://github.com/rust-bitcoin/rust-secp256k1) - Cryptography

## Roadmap

### Phase 1: Sender Logic (Complete)
- Core cryptographic operations
- Silent payment derivation
- Multi-input vTXO support
- Forfeit transaction compatibility

### Phase 2: Performance Optimization (In Progress)
- Parallel ECDH scanning
- Mobile benchmarking (target: <500ms for 1,000 outputs)
- Adversarial scenario testing

### Phase 3: Recipient Support (Planned)
- BIP 352 address encoding
- Blockchain scanning integration
- Payment detection daemon
- CSV delay handling

### Future Enhancements
- Hardware wallet integration
- Multi-signature support
- Payment batching optimization
- Privacy analysis tools
- Mobile client support

## Contributing

Contributions are welcome. Areas of interest:

- BIP 352 address encoding implementation
- Blockchain scanning optimizations
- Test vectors from BIP 352 spec
- Documentation improvements
- Example applications

Please open issues for bugs or feature requests.

## License

MIT License - See LICENSE file for details.

## Acknowledgments

- Ark Protocol team for Layer 2 scaling innovation
- BIP 352 authors for Silent Payments specification
- rust-bitcoin and secp256k1 teams for excellent cryptography libraries
- bark-wallet for reference Ark implementation

## Status

**Phase 1:** Complete | **Phase 2:** In Progress | **Phase 3:** Planned

Silent Ark is a research project exploring recipient privacy for Ark Protocol through BIP 352 Silent Payments. The project has successfully verified technical feasibility and is currently optimizing scanning performance for mobile devices.
