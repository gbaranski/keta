use keta_core::account::Account;
use keta_core::account::Address;

#[derive(Debug, Clone)]
pub struct Tree {
    tree: sled::Tree,
}

impl crate::Tree<Address, Account> for Tree {}

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
