use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{header::LedgerHeader, ledger_type::LedgerEntry};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Ledger {
    pub entries: BTreeMap<LedgerHeader, LedgerEntry>,
}
