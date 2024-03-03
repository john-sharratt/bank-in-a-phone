use crate::{bank_id::BankId, ledger::LedgerForBank};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ResponseCopyBank {
    Copied { ledger: LedgerForBank },
    Denied { err_msg: String },
    DoesNotExist { bank_id: BankId },
}
