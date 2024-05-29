use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pubkey([u8; 32]);

impl Pubkey {
    pub fn new(data: &[u8; 32]) -> Self {
        Self(*data)
    }

    pub fn hex(&self) -> String {
        hex::encode(self.bytes())
    }

    pub fn bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn from_hex(data: &str) -> crate::Result<Self> {
        let decoded = Self(hex::decode(data)?.as_slice().try_into()?);
        Ok(decoded)
    }
}

impl Serialize for Pubkey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.hex().as_str())
    }
}

struct PubkeyVisitor;

impl<'de> serde::de::Visitor<'de> for PubkeyVisitor {
    type Value = Pubkey;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a hex string representing a 32-byte array")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Pubkey::from_hex(value).map_err(serde::de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for Pubkey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(PubkeyVisitor)
    }
}
