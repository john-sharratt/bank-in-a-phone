use std::fmt::Display;

use crate::{bank_id::BankId, ledger::LedgerForBank};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ResponseCopyBank {
    Copied { ledger: LedgerForBank },
    Denied { err_msg: String },
    DoesNotExist { bank_id: BankId },
}

impl Display for ResponseCopyBank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseCopyBank::Copied { ledger } => write!(
                f,
                "ResponseCopyBank::Copied (entries={})",
                ledger.entries.len()
            ),
            ResponseCopyBank::Denied { err_msg } => {
                write!(f, "ResponseCopyBank::Denied ({})", err_msg)
            }
            ResponseCopyBank::DoesNotExist { bank_id } => {
                write!(f, "ResponseCopyBank::DoesNotExist ({})", bank_id)
            }
        }
    }
}
