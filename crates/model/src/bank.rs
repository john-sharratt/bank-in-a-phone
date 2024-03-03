use crate::{bank_id::BankId, password_hash::PasswordHash};

use super::account::{Account, AccountType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Bank {
    pub owner: BankId,
    pub password: PasswordHash,
    pub accounts: Vec<Account>,
}

impl Bank {
    pub fn new(owner: BankId, password: PasswordHash) -> Self {
        Bank {
            owner,
            password,
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

    pub fn id(&self) -> BankId {
        self.owner.clone().into()
    }

    pub fn account(&mut self, type_: AccountType) -> Option<&Account> {
        self.accounts.iter().find(|a| a.type_ == type_)
    }

    pub fn account_mut(&mut self, type_: AccountType) -> Option<&mut Account> {
        self.accounts.iter_mut().find(|a| a.type_ == type_)
    }

    pub fn total_funds(&self) -> u64 {
        self.accounts.iter().map(|a| a.balance_cents).sum()
    }
}
