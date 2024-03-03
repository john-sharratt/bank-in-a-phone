use crate::{bank::Bank, bank_id::BankId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ResponseUpdatedBank {
    InvalidBank { bank_id: BankId },
    InvalidUpdate { err_msg: String },
    Updated { bank: Bank },
}
