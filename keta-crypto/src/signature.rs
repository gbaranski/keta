use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::str::FromStr;

const SIGNATURE_SIZE: usize = 64;

#[derive(Clone, PartialEq, Eq)]
pub struct Signature([u8; SIGNATURE_SIZE]);

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

impl Signature {
    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
        let bytes_length = bytes.as_ref().len();
        let bytes: [u8; SIGNATURE_SIZE] =
            bytes.as_ref().try_into().map_err(|_| Error::InvalidSize {
                expected: &SIGNATURE_SIZE,
                received: bytes_length,
            })?;

        Ok(Self(bytes))
    }

    pub fn as_bytes(&self) -> &[u8; SIGNATURE_SIZE] {
        &self.0
    }

    pub fn to_ed25519_dalek(&self) -> ed25519_dalek::Signature {
        ed25519_dalek::Signature::new(self.0.clone())
    }

    pub fn into_ed25519_dalek(self) -> ed25519_dalek::Signature {
        ed25519_dalek::Signature::new(self.0)
    }

    pub fn from_ed25519_dalek(val: ed25519_dalek::Signature) -> Self {
        Self(val.to_bytes())
    }
}

impl FromStr for Signature {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = hex::decode(s)?;
        Self::from_bytes(bytes)
    }
}

impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SignatureVisitor;

        impl<'de> serde::de::Visitor<'de> for SignatureVisitor {
            type Value = Signature;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("hex encoded signature")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Signature::from_str(v).map_err(serde::de::Error::custom)
            }
        }
        deserializer.deserialize_str(SignatureVisitor)
    }
}

impl std::fmt::Display for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(hex::encode(&self.0).as_str())
    }
}

impl std::fmt::Debug for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(hex::encode(&self.0).as_str())
    }
}
