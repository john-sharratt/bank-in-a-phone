use serde::{Deserialize, Serialize};

use super::account::{Account, AccountType};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Bank {
    pub owner: String,
    pub accounts: Vec<Account>,
}

impl Bank {
    pub fn new(owner: String) -> Self {
        Bank {
            owner,
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
