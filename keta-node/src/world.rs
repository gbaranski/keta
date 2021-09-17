use keta_core::account::Address;
use keta_core::block::Block;
use keta_core::block::HashedBlock;
use keta_core::transaction::SignedTransaction;
use keta_miner::mine_block;
use keta_miner::MineResult;
use keta_node_db::Database;
use keta_node_db::Tree;
use std::collections::VecDeque;
use std::sync::Mutex;

#[derive(Debug)]
pub struct World {
    database: Database,
    pending_transactions: Mutex<VecDeque<SignedTransaction>>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("database: {0}")]
    Database(#[from] keta_node_db::Error),
}

impl World {
    pub fn new(database: Database) -> Result<Self, Error> {
        Ok(Self {
            pending_transactions: Default::default(),
            database,
        })
    }

    pub fn generate_block(&self) -> Result<HashedBlock, Error> {
        let transactions = self
            .pending_transactions
            .lock()
            .unwrap()
            .drain(..)
            .collect();
        let block = Block::generate(
            &self.database.blocks.iter().next_back().unwrap()?,
            transactions,
        );
        let MineResult { hash, nonce } = mine_block(&block);
        let block = HashedBlock { block, hash, nonce };
        self.database.blocks.insert(&block.index, &block)?;
        Ok(block)
    }

    pub fn get_balance(&self, address: &Address) -> Result<u64, Error> {
        let balance = self
            .database
            .accounts
            .get(address)?
            .map(|account| account.balance)
            .unwrap_or(0);
        Ok(balance)
    }

    pub fn send_transaction(&self, transaction: SignedTransaction) -> Result<(), Error> {
        self.pending_transactions
            .lock()
            .unwrap()
            .push_back(transaction);
        Ok(())
    }
}
