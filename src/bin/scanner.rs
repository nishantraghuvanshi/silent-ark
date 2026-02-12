//! Silent Ark scanner binary
//!
//! Command-line tool for scanning Ark Batch Outputs for Silent Payments

use silent_ark::prelude::*;
use silent_ark::scanner::daemon::ScanningDaemon;
use silent_ark::recipient::scanner::SilentScanner;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("Silent Ark Scanner");
    println!("==================\n");

    // TODO: Parse command-line arguments
    // Expected arguments:
    // - Silent Payment address
    // - Private scan key

    // TODO: Load Silent Payment keys
    // let address = ...;
    // let scan_key = ...;

    // Create scanner
    // let scanner = SilentScanner::new(address, scan_key);

    // Create and start daemon
    // let daemon = ScanningDaemon::new(scanner, Duration::from_secs(60));
    // daemon.start().await?;

    println!("Scanner initialized");
    println!("Monitoring for incoming Silent Payments...");

    Ok(())
}
