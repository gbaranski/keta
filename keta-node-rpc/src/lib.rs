use jsonrpc_derive::rpc;
use keta_core::account::Address;
use keta_core::block::HashedBlock;
use keta_core::transaction::SignedTransaction;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("world: {0}")]
    World(String),
}

impl From<Error> for jsonrpc_core::Error {
    fn from(val: Error) -> Self {
        Self {
            code: jsonrpc_core::ErrorCode::ServerError(1),
            message: val.to_string(),
            data: None,
        }
    }
}

#[rpc]
pub trait Rpc {
    #[rpc(name = "sendTransaction")]
    fn send_transaction(&self, transaction: SignedTransaction) -> Result<(), Error>;

    #[rpc(name = "generateBlock")]
    fn generate_block(&self) -> Result<(), Error>;

    #[rpc(name = "getBalance")]
    fn get_balance(&self, address: Address) -> Result<u64, Error>;

    #[rpc(name = "getAllBlocks")]
    fn get_all_blocks(&self) -> Result<Vec<HashedBlock>, Error>;
}

#[cfg(feature = "client-http")]
pub async fn connect_http(url: &str) -> Result<RpcClient, jsonrpc_core_client::RpcError> {
    jsonrpc_core_client::transports::http::connect(url).await
}

#[cfg(feature = "server-http")]
pub async fn serve_http(
    server: impl Rpc,
    address: &std::net::SocketAddr,
) -> Result<(), std::io::Error> {
    use jsonrpc_http_server::ServerBuilder;

    let mut io = jsonrpc_core::IoHandler::new();
    io.extend_with(server.to_delegate());
    let server = ServerBuilder::new(io)
        .threads(1)
        .event_loop_executor(tokio::runtime::Handle::current())
        .start_http(address)?;
    let task = tokio::task::spawn(async move {
        server.wait();
        Ok(())
    });
    task.await.unwrap()
}
