use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BankId {
    name: String,
}

impl BankId {
    pub fn as_str(&self) -> &str {
        &self.name
    }
}

impl From<String> for BankId {
    fn from(value: String) -> Self {
        BankId {
            name: value.to_lowercase(),
        }
    }
}

impl From<&str> for BankId {
    fn from(value: &str) -> Self {
        BankId {
            name: value.to_lowercase(),
        }
    }
}

impl Display for BankId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.name)
    }
}
