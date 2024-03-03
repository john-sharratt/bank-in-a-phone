use crate::{bank::Bank, signature::LedgerSignature};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestUpdateBank {
    pub signature: LedgerSignature,
    pub bank: Bank,
}
