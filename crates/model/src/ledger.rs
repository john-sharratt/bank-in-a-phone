use serde::{Deserialize, Serialize};

use crate::ledger_entry::LedgerEntry;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Ledger {
    pub entries: Vec<LedgerEntry>,
}
