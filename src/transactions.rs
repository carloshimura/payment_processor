use crate::data_models::{
    ClientId, ClientSummary, ClientTransactions, PaymentRecord, TransactionType,
};
use crate::errors::TransactionReaderError;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub fn process_transaction(client_summary: &mut ClientSummary, transaction: &mut PaymentRecord) {
    match &transaction.transaction_type {
        TransactionType::Deposit => match &transaction.amount {
            Some(amount) => {
                let entry = client_summary
                    .previous_processed_transactions
                    .entry(transaction.tx);
                match entry {
                    Entry::Occupied(e) => {
                        eprintln!("Withdrawal {} already exists", e.key())
                    }
                    Entry::Vacant(e) => {
                        client_summary.available += *amount;
                        e.insert(*amount);
                    }
                }
            }
            None => eprintln!("Deposit {} has no amount", transaction.tx.value),
        },
        TransactionType::Withdrawal => match &transaction.amount {
            Some(amount) => {
                if client_summary.total() < *amount {
                    eprintln!(
                        "Withdrawal {} cannot be completed. Not enough funds {} > {}",
                        transaction.tx, amount, client_summary.available
                    );
                    return;
                }
                let entry = client_summary
                    .previous_processed_transactions
                    .entry(transaction.tx);
                match entry {
                    Entry::Occupied(e) => {
                        eprintln!("Deposit {} already exists", e.key())
                    }
                    Entry::Vacant(e) => {
                        client_summary.available -= *amount;
                        e.insert(amount.get_flipped_value());
                    }
                }
            }
            None => eprintln!("Withdrawal {} has no amount", transaction.tx),
        },
        TransactionType::Dispute => {
            if client_summary
                .contested_transactions
                .contains(&transaction.tx)
            {
                eprintln!("Duplicated dispute");
                return;
            }
            client_summary.contested_transactions.insert(transaction.tx);
            if let Some(disputed) = client_summary
                .previous_processed_transactions
                .get(&transaction.tx)
            {
                client_summary.available -= *disputed;
                client_summary.held += *disputed;
            }
        }
        TransactionType::Resolve => {
            if client_summary
                .contested_transactions
                .take(&transaction.tx)
                .is_none()
            {
                eprintln!("Trying to resolve transaction never contested");
                return;
            }
            if let Some(resolved) = client_summary
                .previous_processed_transactions
                .get(&transaction.tx)
            {
                client_summary.available += *resolved;
                client_summary.held -= *resolved;
            }
        }
        TransactionType::Chargeback => {
            if client_summary
                .contested_transactions
                .take(&transaction.tx)
                .is_none()
            {
                eprintln!("Trying to chargeback transaction never contested");
                return;
            }
            if let Some(chargeback) = client_summary
                .previous_processed_transactions
                .get(&transaction.tx)
            {
                client_summary.held -= *chargeback;
                client_summary.locked = true;
            }
        }
    }
}

pub fn validate_pre_process_record(
    client_map: &mut HashMap<ClientId, ClientTransactions>,
    transaction: PaymentRecord,
) -> Result<(), TransactionReaderError> {
    match transaction.transaction_type {
        TransactionType::Withdrawal => {
            if transaction.amount.is_none() {
                Err(TransactionReaderError::InvalidWithdrawal(transaction.tx))?
            }
        }
        TransactionType::Deposit => {
            if transaction.amount.is_none() {
                Err(TransactionReaderError::InvalidDeposit(transaction.tx))?
            }
        }
        TransactionType::Dispute => {
            if transaction.amount.is_some() {
                Err(TransactionReaderError::InvalidDisputeFormat(transaction.tx))?
            }
        }
        TransactionType::Resolve => {
            if transaction.amount.is_some() {
                Err(TransactionReaderError::InvalidResolveFormat(transaction.tx))?
            }
        }
        TransactionType::Chargeback => {
            if transaction.amount.is_some() {
                Err(TransactionReaderError::InvalidChargebackFormat(
                    transaction.tx,
                ))?
            }
        }
    };
    client_map
        .entry(transaction.client)
        .or_default()
        .transactions
        .push(transaction);
    Ok(())
}
