use std::fmt::Display;

use crate::{bank_id::BankId, ledger::LedgerMessage, secret::LedgerSecret};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Copied {
    pub bank_secret: LedgerSecret,
    pub entries: Vec<LedgerMessage>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ResponseCopyBank {
    Copied(Copied),
    Denied { err_msg: String },
    DoesNotExist { bank_id: BankId },
}

impl Display for ResponseCopyBank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseCopyBank::Copied(Copied { entries, .. }) => {
                write!(f, "ResponseCopyBank::Copied (entries={})", entries.len())
            }
            ResponseCopyBank::Denied { err_msg } => {
                write!(f, "ResponseCopyBank::Denied ({})", err_msg)
            }
            ResponseCopyBank::DoesNotExist { bank_id } => {
                write!(f, "ResponseCopyBank::DoesNotExist ({})", bank_id)
            }
        }
    }
}
