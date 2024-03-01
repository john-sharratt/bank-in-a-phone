use serde::{Deserialize, Serialize};

use crate::{bank::Bank, transaction::Transaction};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LedgerType {
    NewBank(Bank),
    UpdateBank(Bank),
    Transfer {
        local_bank: String,
        transaction: Transaction,
    },
    Error(String),
}
