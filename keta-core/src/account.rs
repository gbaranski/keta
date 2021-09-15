use serde::Deserialize;
use serde::Serialize;
use keta_crypto::PublicKey;

pub type Address = PublicKey;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Account {
    pub balance: u64,
}
