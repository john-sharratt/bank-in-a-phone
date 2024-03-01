use serde::{Deserialize, Serialize};

use crate::ledger_type::LedgerEntry;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LedgerHeader {
    pub id: u64,
    pub signature: u64,
}

impl LedgerHeader {
    pub const ZERO: LedgerHeader = LedgerHeader {
        id: 0,
        signature: 0,
    };
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LedgerMessage {
    pub header: LedgerHeader,
    pub entry: LedgerEntry,
}
