use crate::{bank::Bank, secret::LedgerSecret};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestNewBank {
    pub secret: LedgerSecret,
    pub bank: Bank,
}
