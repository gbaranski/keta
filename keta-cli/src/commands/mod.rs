mod balance;
mod generate;
mod pay;

pub use balance::Command as Balance;
pub use generate::Command as Generate;
pub use pay::Command as Pay;

use anyhow::anyhow;
use async_trait::async_trait;
use keta_rpc::Client;
use url::Url;

#[async_trait]
pub trait Command {
    async fn run(self, ctx: Context) -> anyhow::Result<()>;
}

pub struct Context {
    rpc: Option<Client>,
    rpc_url: Url,
}

impl Context {
    pub fn new(rpc_url: Url) -> Self {
        Self { rpc_url, rpc: None }
    }

    pub async fn rpc(&mut self) -> Result<&keta_rpc::Client, anyhow::Error> {
        match self.rpc {
            Some(ref rpc) => Ok(rpc),
            None => {
                let rpc = keta_rpc::connect(&self.rpc_url)
                    .await
                    .map_err(|err| anyhow!("RPC Error: {}", err.to_string()))?;
                self.rpc = Some(rpc);
                Ok(self.rpc.as_ref().unwrap())
            }
        }
    }
}
