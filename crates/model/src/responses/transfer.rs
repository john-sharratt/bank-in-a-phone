use std::fmt::Display;

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

impl Display for ResponseTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseTransfer::InvalidBank { bank_id } => {
                write!(f, "ResponseTransfer::InvalidBank ({})", bank_id)
            }
            ResponseTransfer::InvalidSignature => write!(f, "ResponseTransfer::InvalidSignature"),
            ResponseTransfer::InvalidAccount { bank_id, account } => write!(
                f,
                "ResponseTransfer::InvalidAccount ({})[{}]",
                bank_id, account
            ),
            ResponseTransfer::InsufficientFunds {
                requested,
                available,
                bank_id,
                account,
            } => write!(
                f,
                "ResponseTransfer::InsufficientFunds ({})[{}] {} vs {}",
                bank_id, account, requested, available
            ),
            ResponseTransfer::Transferred => write!(f, "ResponseTransfer::Transferred"),
        }
    }
}
