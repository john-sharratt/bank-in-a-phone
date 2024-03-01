use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AccountType {
    Wallet,
    Savings,
    Printer,
}

impl AccountType {
    pub fn can_move_money(&self) -> bool {
        match self {
            AccountType::Wallet => true,
            AccountType::Savings => true,
            AccountType::Printer => false,
        }
    }

    pub fn can_send_money(&self) -> bool {
        match self {
            AccountType::Wallet => true,
            AccountType::Savings => false,
            AccountType::Printer => false,
        }
    }
}

impl Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountType::Wallet => write!(f, "Wallet Account"),
            AccountType::Savings => write!(f, "Savings Account"),
            AccountType::Printer => write!(f, "Money Printer"),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AccountRef {
    Local { account: AccountType },
    Foreign { bank: String, account: AccountType },
}

impl Display for AccountRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountRef::Local { account } => write!(f, "{account}"),
            AccountRef::Foreign { bank, account } if *account == AccountType::Wallet => {
                write!(f, "{bank}")
            }
            AccountRef::Foreign { bank, account } => write!(f, "{account}({bank})"),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Account {
    pub type_: AccountType,
    pub balance_cents: u64,
}
