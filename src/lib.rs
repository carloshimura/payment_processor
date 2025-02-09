use crate::data_models::PaymentRecord;
use crate::errors::TransactionReaderError;
use crate::transactions::{process_transaction, validate_pre_process_record};
use rayon::prelude::*;
use std::collections::HashMap;

pub mod data_models;
pub mod errors;
mod transactions;

pub fn read_and_parse_transactions(
    path: &str,
    output: &mut impl std::io::Write,
) -> Result<(), TransactionReaderError> {
    let mut csv_reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path(path)?;

    let mut client_map = HashMap::new();

    csv_reader
        .deserialize()
        .try_for_each(
            |transaction: Result<PaymentRecord, csv::Error>| match transaction {
                Ok(transaction) => validate_pre_process_record(&mut client_map, transaction),
                Err(err) => Err(err.into()),
            },
        )?;
    client_map
        .par_iter_mut()
        .for_each(|(_client, client_transactions)| {
            client_transactions.transactions.iter_mut().for_each(|tx| {
                process_transaction(&mut client_transactions.client_summary, tx);
            });
        });

    writeln!(output, "client, available, held, total, locked")?;
    for (client, summary) in client_map {
        writeln!(
            output,
            "{},{},{},{},{}",
            client,
            summary.client_summary.available,
            summary.client_summary.held,
            summary.client_summary.total(),
            summary.client_summary.locked
        )?;
    }

    Ok(())
}
