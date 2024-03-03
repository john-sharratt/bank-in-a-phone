use serde::{Deserialize, Serialize};
use serde::{Deserializer, Serializer};
use std::convert::TryInto;

pub fn serialize<S: Serializer, const C: usize>(v: &[u8; C], s: S) -> Result<S::Ok, S::Error> {
    #[allow(deprecated)]
    let base64 = base64::encode(v);
    String::serialize(&base64, s)
}

pub fn deserialize<'de, D: Deserializer<'de>, const C: usize>(d: D) -> Result<[u8; C], D::Error> {
    let base64 = String::deserialize(d)?;
    #[allow(deprecated)]
    let bytes = base64::decode(base64).map_err(serde::de::Error::custom)?;
    let bytes: [u8; C] = bytes.try_into().map_err(|data: Vec<u8>| {
        serde::de::Error::custom(format!("failed to convert array (len={})", data.len()))
    })?;
    Ok(bytes)
}
