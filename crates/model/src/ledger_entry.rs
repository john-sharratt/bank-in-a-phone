use serde::{Deserialize, Serialize};

use crate::ledger_type::LedgerType;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LedgerEntry {
    pub id: u64,
    pub entry: LedgerType,
}
