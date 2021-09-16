use crate::world;
use crate::world::World;
use keta_core::transaction::SignedTransaction;
use keta_node_rpc::Error;
use keta_node_rpc::Rpc;

pub struct Server {
    world: World,
}

impl From<world::Error> for Error {
    fn from(err: world::Error) -> Self {
        Error::World(err.to_string())
    }
}

impl Rpc for Server {
    fn send_transaction(&self, transaction: SignedTransaction) -> Result<(), keta_node_rpc::Error> {
        todo!()
    }

    fn generate_block(&self) -> Result<(), keta_node_rpc::Error> {
        todo!()
    }

    fn get_balance(
        &self,
        address: keta_core::account::Address,
    ) -> Result<u64, keta_node_rpc::Error> {
        todo!()
    }

    fn get_all_blocks(&self) -> Result<Vec<keta_core::block::HashedBlock>, keta_node_rpc::Error> {
        todo!()
    }
}

impl Server {
    pub fn new(world: World) -> Self {
        Self { world }
    }

    pub async fn run(self, address: &std::net::SocketAddr) -> Result<(), std::io::Error> {
        keta_node_rpc::serve_http(self, address).await
    }
}
