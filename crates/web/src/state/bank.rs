use serde::{Serialize, Deserialize};

use super::{account::{Account, AccountRef, AccountType}, transaction::Transaction};

#[derive(Deserialize, Serialize)]
pub struct Bank {
    pub accounts: Vec<Account>,
    pub history: Vec<Transaction>,
}

impl Default
for Bank {
    fn default() -> Self {
        Bank {
            accounts: vec![
                Account {
                    type_: AccountType::Wallet,
                    balance_cents: 1000_00,
                },
                Account {
                    type_: AccountType::Savings,
                    balance_cents: 999_000_00,
                },
            ],
            history: vec![Transaction {
                from: AccountRef::Foreign { username: "John".to_string(), account: AccountType::Printer },
                to: AccountRef::Local { account: AccountType::Wallet },
                description: "Donation from money printer".to_string(),
                amount_cents: 1_000_000_00
            },Transaction {
                from: AccountRef::Local { account: AccountType::Wallet },
                to: AccountRef::Local { account: AccountType::Savings },
                description: "Squirreling some money away".to_string(),
                amount_cents: 999_000_00
            },],
        }
    }
}