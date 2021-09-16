use crate::transaction::SignedTransaction;
use chrono::DateTime;
use chrono::Utc;
use keta_crypto::Hash;
use keta_crypto::Nonce;
use serde::Deserialize;
use serde::Serialize;

// #[cfg(feature = "sled")]
// impl std::convert::TryFrom<sled::IVec> for BlockIndex {
//     type Error = BlockIndexError;
//
//     fn try_from(value: sled::IVec) -> Result<Self, Self::Error> {
//         let bytes: &[u8] = value.as_ref();
//         let bytes: [u8; 8] = std::convert::TryInto::try_into(bytes)?;
//         Ok(Self(bytes))
//     }
// }

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlockIndex([u8; 8]);

impl Serialize for BlockIndex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.to_u64())
    }
}

impl<'de> Deserialize<'de> for BlockIndex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct BlockIndexVisitor;

        impl<'de> serde::de::Visitor<'de> for BlockIndexVisitor {
            type Value = BlockIndex;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("unsigned 64 bit value")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(BlockIndex::from(v))
            }
        }
        deserializer.deserialize_u64(BlockIndexVisitor)
    }
}

impl AsRef<[u8]> for BlockIndex {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<[u8; 8]> for BlockIndex {
    fn from(val: [u8; 8]) -> Self {
        Self(val)
    }
}

impl From<BlockIndex> for [u8; 8] {
    fn from(val: BlockIndex) -> Self {
        val.0
    }
}

impl From<u64> for BlockIndex {
    fn from(val: u64) -> Self {
        Self(val.to_be_bytes())
    }
}

impl From<&BlockIndex> for u64 {
    fn from(val: &BlockIndex) -> Self {
        u64::from_be_bytes(val.0)
    }
}

impl std::fmt::Display for BlockIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_u64())
    }
}

impl std::fmt::Debug for BlockIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_u64())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BlockIndexError {
    #[error("try from slice error: {0}")]
    TryFromSliceError(#[from] std::array::TryFromSliceError),
}

impl std::convert::TryFrom<Vec<u8>> for BlockIndex {
    type Error = BlockIndexError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let bytes: &[u8] = value.as_ref();
        let bytes: [u8; 8] = std::convert::TryInto::try_into(bytes)?;
        Ok(Self(bytes))
    }
}

impl BlockIndex {
    pub const ZERO: Self = Self(u64::to_be_bytes(0));

    pub fn increment(&self) -> Self {
        Self::from(self.to_u64() + 1)
    }

    pub fn to_u64(&self) -> u64 {
        self.into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Block {
    pub index: BlockIndex,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
    pub transactions: Vec<SignedTransaction>,
    pub prev_hash: Hash,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HashedBlock {
    pub block: Block,
    pub hash: Hash,
    pub nonce: Nonce,
}

impl std::ops::Deref for HashedBlock {
    type Target = Block;

    fn deref(&self) -> &Self::Target {
        &self.block
    }
}

impl Block {
    pub fn generate(prev_block: &HashedBlock, transactions: Vec<SignedTransaction>) -> Self {
        Self {
            index: prev_block.block.index.increment(),
            timestamp: Utc::now(),
            prev_hash: prev_block.hash.clone(),
            transactions,
        }
    }

    pub fn hash_with_nonce(&self, nonce: Nonce) -> Hash {
        let serialized = bincode::serialize(self).unwrap();
        keta_crypto::Hash::new_with_nonce(serialized, nonce)
    }
}

#[cfg(test)]
mod test {
    use super::{Block, BlockIndex, Hash};
    use std::str::FromStr;

    #[test]
    fn calculate_hash_with_nonce() {
        let block = Block {
            index: BlockIndex::ZERO,
            timestamp: chrono::MAX_DATETIME,
            prev_hash: Hash::ZERO,
            transactions: Vec::new(),
        };
        let hash = block.hash_with_nonce(10);
        assert_eq!(
            hash,
            Hash::from_str("3f3049b0c7069ae1110828dcf9861f99d50b0e2e9b56f8d11b97fd957a20413d")
                .unwrap()
        );
    }
}
