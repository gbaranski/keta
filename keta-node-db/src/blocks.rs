use super::Error;
use keta_core::block;
use keta_core::block::HashedBlock;
use sled::IVec;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct Tree {
    tree: sled::Tree,
}

impl Tree {
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = Result<HashedBlock, Error>> {
        fn key_value_to_item(key: IVec, value: IVec) -> Result<HashedBlock, Error> {
            let block: HashedBlock = bincode::deserialize(&value)?;
            let index =
                block::Index::try_from(key).map_err(|err| Error::InvalidKey(err.to_string()))?;
            assert_eq!(index, block.index);
            Ok(block)
        }

        let iter = self.tree.iter().map(|item| match item {
            Ok((key, value)) => key_value_to_item(key, value),
            Err(err) => Err(Error::SledError(err)),
        });

        iter
    }
}

impl crate::Tree<block::Index, HashedBlock> for Tree {}

impl AsRef<sled::Tree> for Tree {
    fn as_ref(&self) -> &sled::Tree {
        &self.tree
    }
}

impl From<sled::Tree> for Tree {
    fn from(tree: sled::Tree) -> Self {
        Self { tree }
    }
}
