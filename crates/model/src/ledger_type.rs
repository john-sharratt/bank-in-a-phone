use std::fmt::Display;

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

impl Display for LedgerEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LedgerEntry::NewBank { bank, .. } => {
                write!(f, "LedgerEntry::NewBank ({})", bank.owner)
            }
            LedgerEntry::UpdateBank(bank) => write!(f, "LedgerEntry::UpdateBank ({})", bank.owner),
            LedgerEntry::Transaction { transaction } => {
                write!(f, "LedgerEntry::Transaction ({})", transaction)
            }
        }
    }
}
