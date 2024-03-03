use serde::{Deserialize, Serialize};

use crate::{account::AccountType, bank_id::BankId};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ResponseTransfer {
    InvalidBank {
        bank_id: BankId,
    },
    InvalidSignature,
    InvalidAccount {
        bank_id: BankId,
        account: AccountType,
    },
    InsufficientFunds {
        requested: u64,
        available: u64,
        bank_id: BankId,
        account: AccountType,
    },
    Transferred,
}
