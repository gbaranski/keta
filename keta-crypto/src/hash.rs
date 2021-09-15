const HASH_SIZE: usize = 32;

#[derive(Clone, PartialEq, Eq)]
pub struct Hash([u8; HASH_SIZE]);

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("hex error: {0}")]
    FromHexError(#[from] hex::FromHexError),

    #[error("invalid size: {received}, expected: {expected}")]
    InvalidSize {
        expected: &'static usize,
        received: usize,
    },
}

impl Hash {
    pub const ZERO: Self = Self([0; HASH_SIZE]);

    pub fn as_bytes(&self) -> &[u8; HASH_SIZE] {
        &self.0
    }
}

use std::convert::TryInto;
use std::str::FromStr;

impl FromStr for Hash {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = hex::decode(s)?;
        let bytes_length = bytes.len();
        let bytes: [u8; HASH_SIZE] = bytes.try_into().map_err(|_| Error::InvalidSize {
            expected: &HASH_SIZE,
            received: bytes_length,
        })?;
        Ok(Self(bytes))
    }
}

impl std::fmt::Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(hex::encode(self.0).as_str())
    }
}

impl std::fmt::Debug for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(hex::encode(self.0).as_str())
    }
}

impl serde::ser::Serialize for Hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> serde::de::Deserialize<'de> for Hash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct HashVisitor;

        impl<'de> serde::de::Visitor<'de> for HashVisitor {
            type Value = Hash;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("hex encoded key")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Hash::from_str(v).map_err(serde::de::Error::custom)
            }
        }
        deserializer.deserialize_str(HashVisitor)
    }
}

use tiny_keccak::Hasher;

impl From<tiny_keccak::Sha3> for Hash {
    fn from(sha3: tiny_keccak::Sha3) -> Self {
        let mut output = [0; 32];
        sha3.finalize(&mut output);
        Self(output)
    }
}