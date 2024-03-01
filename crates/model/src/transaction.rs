use serde::{Deserialize, Serialize};

use super::account::AccountRef;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Transaction {
    pub from: AccountRef,
    pub to: AccountRef,
    pub description: String,
    pub amount_cents: u64,
}
