use crate::{bank_id::BankId, password_hash::PasswordHash};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestCopyBank {
    pub password: PasswordHash,
    pub bank: BankId,
}
