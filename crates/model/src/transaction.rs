use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::pretty::pretty_print_cents;

use super::account::AccountRef;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Transaction {
    pub from: AccountRef,
    pub to: AccountRef,
    pub description: String,
    pub amount_cents: u64,
}

impl Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{} {}->{}]",
            pretty_print_cents(self.amount_cents),
            self.from,
            self.to
        )
    }
}
