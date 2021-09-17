use crate::world;
use crate::world::World;
use keta_core::block::HashedBlock;
use keta_core::transaction::SignedTransaction;
use keta_rpc::Error;
use keta_rpc::RpcServer;

pub struct Server {
    world: World,
}

impl From<world::Error> for Error {
    fn from(err: world::Error) -> Self {
        Error::World(err.to_string())
    }
}

impl RpcServer for Server {
    fn send_transaction(&self, transaction: SignedTransaction) -> Result<(), keta_rpc::Error> {
        self.world.send_transaction(transaction)?;
        Ok(())
    }

    fn generate_block(&self) -> Result<HashedBlock, keta_rpc::Error> {
        let block = self.world.generate_block()?;
        Ok(block)
    }

    fn get_balance(&self, address: keta_core::account::Address) -> Result<u64, keta_rpc::Error> {
        let balance = self.world.get_balance(&address)?;
        Ok(balance)
    }

    fn get_all_blocks(&self) -> Result<Vec<keta_core::block::HashedBlock>, keta_rpc::Error> {
        todo!()
    }
}

impl Server {
    pub fn new(world: World) -> Self {
        Self { world }
    }

    pub async fn run(self, address: &std::net::SocketAddr) -> Result<(), keta_rpc::Error> {
        keta_rpc::serve(self, address).await
    }
}
