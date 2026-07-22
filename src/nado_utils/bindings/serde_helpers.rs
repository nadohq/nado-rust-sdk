use std::str::FromStr;

use alloy::primitives::{Address, U256};
use serde::de::Error;
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serializer};

pub(crate) fn serialize_u256<S>(value: &U256, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&value.to_string())
}

pub(crate) fn deserialize_u256<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    U256::from_str_radix(&s, 10).map_err(|_| D::Error::custom("invalid u256 value"))
}

pub(crate) fn serialize_vec_u256_hex<S>(value: &[U256], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(value.len()))?;
    for e in value {
        seq.serialize_element(&format!("{e:#x}"))?;
    }
    seq.end()
}

pub(crate) fn deserialize_vec_u256_hex<'de, D>(deserializer: D) -> Result<Vec<U256>, D::Error>
where
    D: Deserializer<'de>,
{
    Vec::<String>::deserialize(deserializer)?
        .iter()
        .map(|s| parse_u256_hex(s))
        .collect()
}

pub(crate) fn serialize_address<S>(value: &Address, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format!("0x{}", hex::encode(value.as_slice())))
}

pub(crate) fn deserialize_address<'de, D>(deserializer: D) -> Result<Address, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Address::from_str(&s).map_err(|_| D::Error::custom("invalid address value"))
}

fn parse_u256_hex<E: Error>(s: &str) -> Result<U256, E> {
    let trimmed = s
        .strip_prefix("0x")
        .or_else(|| s.strip_prefix("0X"))
        .unwrap_or(s);
    U256::from_str_radix(trimmed, 16).map_err(|_| E::custom("invalid u256 hex value"))
}
