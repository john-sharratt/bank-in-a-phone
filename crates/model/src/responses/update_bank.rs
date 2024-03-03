use std::fmt::Display;

use crate::{bank::Bank, bank_id::BankId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ResponseUpdatedBank {
    InvalidBank { bank_id: BankId },
    InvalidUpdate { err_msg: String },
    Updated { bank: Bank },
}

impl Display for ResponseUpdatedBank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseUpdatedBank::InvalidBank { bank_id } => {
                write!(f, "ResponseUpdatedBank::InvalidBank ({})", bank_id)
            }
            ResponseUpdatedBank::InvalidUpdate { err_msg } => {
                write!(f, "ResponseUpdatedBank::InvalidUpdate ({})", err_msg)
            }
            ResponseUpdatedBank::Updated { bank } => {
                write!(f, "ResponseUpdatedBank::Updated ({})", bank.owner)
            }
        }
    }
}
