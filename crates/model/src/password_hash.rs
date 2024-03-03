use sha256::digest;
use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PasswordHash {
    hash: String,
}

impl PasswordHash {
    pub fn from_password(seed: &str, password: &str) -> PasswordHash {
        let part1 = digest(seed.as_bytes());
        let part2 = digest(password.as_bytes());
        let parts = format!("{part1}-{part2}");

        let hash = digest(parts.as_bytes());
        PasswordHash { hash }
    }
}

impl Display for PasswordHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.hash)
    }
}
