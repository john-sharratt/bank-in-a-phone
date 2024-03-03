use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha256::digest;

use crate::base64_array;
use crate::signature::LedgerSignature;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LedgerSecret {
    #[serde(with = "base64_array")]
    data1: [u8; 32],
    #[serde(with = "base64_array")]
    data2: [u8; 32],
    #[serde(with = "base64_array")]
    data3: [u8; 32],
    #[serde(with = "base64_array")]
    data4: [u8; 32],
}

impl LedgerSecret {
    pub fn new() -> LedgerSecret {
        let mut ret = LedgerSecret {
            data1: [0u8; 32],
            data2: [0u8; 32],
            data3: [0u8; 32],
            data4: [0u8; 32],
        };

        let mut rand = rand::thread_rng();
        rand.fill_bytes(&mut ret.data1);
        rand.fill_bytes(&mut ret.data2);
        rand.fill_bytes(&mut ret.data3);
        rand.fill_bytes(&mut ret.data4);

        ret
    }

    pub const fn from_static(
        data1: [u8; 32],
        data2: [u8; 32],
        data3: [u8; 32],
        data4: [u8; 32],
    ) -> LedgerSecret {
        LedgerSecret {
            data1,
            data2,
            data3,
            data4,
        }
    }

    pub fn sign<W>(&self, what: &W) -> LedgerSignature
    where
        W: Serialize,
    {
        match self.sign_internal(&what) {
            Ok(s) => s,
            Err(err) => {
                log::debug!("failed to sign - {}", err);
                LedgerSignature::random()
            }
        }
    }

    pub fn sign_ext<W, W2>(&self, what: &W, other: &W2) -> LedgerSignature
    where
        W: Serialize,
        W2: Serialize,
    {
        match self.sign_ext_internal(&what, &other) {
            Ok(s) => s,
            Err(err) => {
                log::debug!("failed to sign - {}", err);
                LedgerSignature::random()
            }
        }
    }

    fn sign_internal<W>(&self, what: &W) -> anyhow::Result<LedgerSignature>
    where
        W: Serialize,
    {
        let what_data = bincode::serialize(&what)?;
        let mut cat_data = Vec::with_capacity(
            self.data1.len()
                + self.data2.len()
                + self.data3.len()
                + self.data4.len()
                + what_data.len(),
        );
        cat_data.extend_from_slice(&self.data1);
        cat_data.extend_from_slice(&self.data2);
        cat_data.extend_from_slice(&self.data3);
        cat_data.extend_from_slice(&self.data4);
        cat_data.extend_from_slice(&what_data);

        Ok(LedgerSignature {
            hash: digest(&cat_data),
        })
    }

    fn sign_ext_internal<W, W2>(&self, what: &W, other: &W2) -> anyhow::Result<LedgerSignature>
    where
        W: Serialize,
        W2: Serialize,
    {
        let what_data = bincode::serialize(&what)?;
        let other_data = bincode::serialize(&other)?;
        let mut cat_data = Vec::with_capacity(
            self.data1.len()
                + self.data2.len()
                + self.data3.len()
                + self.data4.len()
                + what_data.len(),
        );
        cat_data.extend_from_slice(&self.data1);
        cat_data.extend_from_slice(&self.data2);
        cat_data.extend_from_slice(&self.data3);
        cat_data.extend_from_slice(&self.data4);
        cat_data.extend_from_slice(&what_data);
        cat_data.extend_from_slice(&other_data);

        Ok(LedgerSignature {
            hash: digest(&cat_data),
        })
    }
}
