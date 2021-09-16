use keta_crypto::PublicKey;
use serde::Deserialize;
use serde::Serialize;

pub type Address = PublicKey;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Account {
    pub balance: u64,
}
