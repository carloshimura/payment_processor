use crate::data_models::TransactionId;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransactionReaderError {
    #[error("Error while reading CSV file: {0}")]
    CSVReaderError(#[from] csv::Error),
    #[error("Withdrawal without an amount: {0:?}")]
    InvalidWithdrawal(TransactionId),
    #[error("Deposit without and amount: {0:?}")]
    InvalidDeposit(TransactionId),
    #[error("Dispute contains an amount: {0:?}")]
    InvalidDisputeFormat(TransactionId),
    #[error("Resolve contains an amount: {0:?}")]
    InvalidResolveFormat(TransactionId),
    #[error("Chargeback contains an amount: {0:?}")]
    InvalidChargebackFormat(TransactionId),
    #[error("Unable to write to stdout: {0}")]
    UnableToWriteToStdout(#[from] std::io::Error),
}
