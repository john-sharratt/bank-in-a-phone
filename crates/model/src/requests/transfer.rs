use crate::{signature::LedgerSignature, transaction::Transaction};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestTransfer {
    pub signature: LedgerSignature,
    pub trans: Transaction,
}
