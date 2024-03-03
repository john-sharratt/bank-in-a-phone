use serde::{Deserialize, Serialize};

use crate::{bank::Bank, secret::LedgerSecret, transaction::Transaction};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LedgerEntry {
    NewBank {
        bank_secret: LedgerSecret,
        bank: Bank,
    },
    UpdateBank(Bank),
    Transaction {
        transaction: Transaction,
    },
}
