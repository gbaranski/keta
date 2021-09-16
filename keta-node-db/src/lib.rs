use std::convert::TryFrom;

mod accounts;
mod blocks;

pub use accounts::Tree as AccountsTree;
pub use blocks::Tree as BlocksTree;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("sled error: {0}")]
    SledError(#[from] sled::Error),

    #[error("bincode error: {0}")]
    Bincode(#[from] bincode::Error),

    #[error("invalid key: {0}")]
    InvalidKey(String),

    #[error("utf8 error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}

pub trait Tree<K, V>
where
    K: AsRef<[u8]> + TryFrom<sled::IVec>,
    V: serde::ser::Serialize + serde::de::DeserializeOwned,
    Self: AsRef<sled::Tree>,
{
    fn insert(&self, key: &K, value: &V) -> Result<(), Error> {
        let tree = self.as_ref();
        tree.insert(key, bincode::serialize(value)?)?;
        tree.flush()?;
        Ok(())
    }

    fn get(&self, key: &K) -> Result<Option<V>, Error> {
        let tree = self.as_ref();
        match tree.get(key.as_ref()) {
            Ok(Some(value)) => Ok(bincode::deserialize(&value)?),
            Ok(None) => Ok(None),
            Err(err) => Err(Error::SledError(err)),
        }
    }

    fn len(&self) -> usize {
        let tree = self.as_ref();
        tree.len()
    }
}

#[derive(Debug, Clone)]
pub struct Database {
    pub blocks: blocks::Tree,
    pub accounts: accounts::Tree,
}

impl Database {
    pub fn new(path: impl AsRef<std::path::Path>) -> Result<Self, Error> {
        let database = sled::open(path)?;
        Ok(Self {
            blocks: BlocksTree::from(database.open_tree("blocks")?),
            accounts: AccountsTree::from(database.open_tree("accounts")?),
        })
    }
}
