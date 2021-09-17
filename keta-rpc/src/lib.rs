use jsonrpsee::proc_macros::rpc;
use keta_core::account::Address;
use keta_core::block::HashedBlock;
use keta_core::transaction::SignedTransaction;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, thiserror::Error)]
pub enum Error {
    #[error("world: {0}")]
    World(String),
    #[error("json-rpc: {0}")]
    JsonRPC(String),
}

impl From<jsonrpsee::types::Error> for Error {
    fn from(err: jsonrpsee::types::Error) -> Self {
        Self::JsonRPC(err.to_string())
    }
}

#[cfg_attr(feature = "server", rpc(server))]
#[cfg_attr(feature = "client", rpc(client))]
pub trait Rpc {
    #[method(name = "sendTransaction")]
    fn send_transaction(&self, transaction: SignedTransaction) -> Result<(), Error>;
    #[method(name = "generateBlock")]
    fn generate_block(&self) -> Result<HashedBlock, Error>;
    #[method(name = "getBalance")]
    fn get_balance(&self, address: Address) -> Result<u64, Error>;
    #[method(name = "getAllBlocks")]
    fn get_all_blocks(&self) -> Result<Vec<HashedBlock>, Error>;
}

#[cfg(feature = "client")]
pub type Client = jsonrpsee::ws_client::WsClient;

#[cfg(feature = "client")]
pub async fn connect(url: &url::Url) -> Result<jsonrpsee::ws_client::WsClient, Error> {
    use jsonrpsee::ws_client::WsClientBuilder;

    let client = WsClientBuilder::default().build(url.as_str()).await?;
    Ok(client)
}

#[cfg(feature = "server")]
pub async fn serve(
    server: impl RpcServer,
    address: impl tokio::net::ToSocketAddrs,
) -> Result<(), Error> {
    use jsonrpsee::ws_server::WsServerBuilder;

    let ws_server = WsServerBuilder::default().build(address).await?;
    ws_server.start(server.into_rpc()).await;

    Ok(())
}
