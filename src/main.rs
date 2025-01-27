use std::{fs::File, path::PathBuf};

use anyhow::{Context, Result};
use clap::Parser;
use tracing::{trace, warn};

mod client;
mod ids;
mod logging;
mod positive_decimal;
mod processor;
mod reader;
mod transaction;

#[derive(Debug, Parser)]
/// Transaction processor
struct Args {
    /// Path to the file containing the transactions
    file: PathBuf,
}

fn main() -> Result<()> {
    // Set up logging
    logging::init_logging();

    // Parse CLI arguments
    let args = Args::parse();

    trace!(?args, "application started");

    // Open the CSV file
    let file = File::open(&args.file).context("open csv file")?;
    trace!(?args.file, "opened csv file");

    // Read the CSV file using the TransactionReader
    // This reader only returns valid transactions
    let mut reader = reader::TransactionReader::new(&file);

    // Create a processor to process the transactions
    let mut processor = processor::Processor::new();

    // Loop through all the the transactions and process them one by one
    while let Some(transaction) = reader.next() {
        trace!(?transaction, "processing transaction");

        // In case the transaction processing fails, print a warning, but don't stop processing
        if let Err(err) = processor.handle_transaction(transaction) {
            warn!(?err, "transaction processing failed");
        }
    }

    // Print the status of all the clients to stdout
    let mut csv_writer = csv::Writer::from_writer(std::io::stdout());
    for status_entry in processor.status_entries() {
        csv_writer
            .serialize(status_entry)
            .context("write status entry")?;
    }
    csv_writer.flush().context("flush csv writer")?;

    trace!(?args, "application finished");

    Ok(())
}
