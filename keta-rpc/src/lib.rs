use jsonrpc_derive::rpc;
use keta_core::account::Address;
use keta_core::block::HashedBlock;
use keta_core::transaction::SignedTransaction;

#[derive(Debug, thiserror::Error)]
pub enum Error{
    #[error("internal: {0}")]
    Internal(String),
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
pub async fn connect_http(url: &str) -> Result<RpcClient, jsonrpc_core_client::Error> {
    jsonrpc_core_client::transports::http::connect(url).await
}
