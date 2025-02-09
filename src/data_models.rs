use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::ops::{Add, AddAssign, SubAssign};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Default, Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone)]
#[serde(transparent)]
pub struct ClientId {
    pub value: u16,
}

impl Display for ClientId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Copy)]
#[serde(transparent)]
pub struct TransactionId {
    pub value: u32,
}

impl Display for TransactionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Default, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Clone, Copy)]
#[serde(transparent)]
pub struct Amount {
    pub value: f64,
}

impl Add for Amount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            value: self.value + rhs.value,
        }
    }
}

impl AddAssign for Amount {
    fn add_assign(&mut self, other: Self) {
        self.value += other.value;
    }
}

impl SubAssign for Amount {
    fn sub_assign(&mut self, other: Self) {
        self.value -= other.value;
    }
}

impl Amount {
    pub fn get_flipped_value(self) -> Self {
        Self { value: -self.value }
    }
}

impl Display for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.value.abs() == 0.0 {
            write!(f, "0")
        } else {
            write!(f, "{}", self.value)
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PaymentRecord {
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,
    pub client: ClientId,
    pub tx: TransactionId,
    #[serde(default)]
    pub amount: Option<Amount>,
}

#[derive(Debug, Default)]
pub struct ClientTransactions {
    pub transactions: Vec<PaymentRecord>,
    pub client_summary: ClientSummary,
}

#[derive(Debug, Default)]
pub struct ClientSummary {
    pub available: Amount,
    pub held: Amount,
    pub locked: bool,
    pub previous_processed_transactions: HashMap<TransactionId, Amount>,
    pub contested_transactions: HashSet<TransactionId>,
}

impl ClientSummary {
    pub fn total(&self) -> Amount {
        self.available + self.held
    }
}
