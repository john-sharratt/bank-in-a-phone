use super::account::{Account, AccountType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Bank {
    pub owner: String,
    pub secret: String,
    pub accounts: Vec<Account>,
}

impl Bank {
    pub fn new(owner: String, password_hash: String) -> Self {
        Bank {
            owner,
            secret: password_hash,
            accounts: vec![
                Account {
                    type_: AccountType::Wallet,
                    balance_cents: 0,
                },
                Account {
                    type_: AccountType::Savings,
                    balance_cents: 0,
                },
            ],
        }
    }

    pub fn find_account(&mut self, type_: AccountType) -> Option<&mut Account> {
        self.accounts.iter_mut().find(|a| a.type_ == type_)
    }
}
