use crate::account::Address;
use keta_crypto::Keypair;
use keta_crypto::PublicKey;
use keta_crypto::Signature;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, thiserror::Error)]
pub enum VerifyError {
    #[error("invalid signature")]
    InvalidSignature,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transaction {
    pub from: Address,
    pub to: Address,
    pub value: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedTransaction {
    pub signature: Signature,
    pub transaction: Transaction,
}

impl Transaction {
    pub fn sign(self, keypair: &Keypair) -> SignedTransaction {
        let serialized = bincode::serialize(&self).unwrap();
        let signature = keypair.sign(serialized);

        SignedTransaction {
            transaction: self,
            signature,
        }
    }
}

impl SignedTransaction {
    pub fn verify(&self, key: &PublicKey) -> Result<(), VerifyError> {
        let serialized = bincode::serialize(&self).unwrap();
        key.verify(&serialized, self.signature.clone())
            .map_err(|_| VerifyError::InvalidSignature)
    }
}

impl std::ops::Deref for SignedTransaction {
    type Target = Transaction;

    fn deref(&self) -> &Self::Target {
        &self.transaction
    }
}
