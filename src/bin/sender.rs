//! Silent Ark sender binary
//!
//! Command-line tool for creating Silent Payments within the Ark protocol

use silent_ark::prelude::*;
use silent_ark::sender::payment::SilentSender;
use std::str::FromStr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("Silent Ark Sender");
    println!("==================\n");

    // TODO: Parse command-line arguments
    // Expected arguments:
    // - recipient Silent Payment address
    // - amount to send
    // - vTXO inputs to use

    // TODO: Load vTXOs from wallet
    let available_vtxos = vec![];

    // Create sender
    let sender = SilentSender::new(available_vtxos);

    // TODO: Create payment
    // let recipient_address = SilentAddress::decode("sp1...")?;
    // let output_address = sender.create_payment(&recipient_address, amount, vtxo_indices)?;

    println!("Sender initialized");
    println!("Available balance: {} satoshis", sender.available_balance());

    Ok(())
}
