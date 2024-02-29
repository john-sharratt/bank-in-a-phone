use serde::{Serialize, Deserialize};

use super::account::AccountRef;

#[derive(Deserialize, Serialize)]
pub struct Transaction {
    pub from: AccountRef,
    pub to: AccountRef,
    pub description: String,
    pub amount_cents: u64
}
