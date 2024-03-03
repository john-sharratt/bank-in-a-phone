use std::fmt::Display;

use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha256::digest;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LedgerSignature {
    pub hash: String,
}

impl LedgerSignature {
    pub const ZERO: LedgerSignature = LedgerSignature {
        hash: String::new(),
    };

    pub fn random() -> LedgerSignature {
        let mut random_data = [0u8; 256];
        let mut rand = rand::thread_rng();
        rand.fill_bytes(&mut random_data);

        LedgerSignature {
            hash: digest(&random_data),
        }
    }
}

impl Display for LedgerSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hash)
    }
}
