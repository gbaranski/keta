use async_trait::async_trait;
use keta_core::account;
use keta_rpc::RpcClient;

#[derive(Debug)]
pub struct Command {
    pub address: account::Address,
}

#[async_trait]
impl super::Command for Command {
    async fn run(self, mut ctx: super::Context) -> anyhow::Result<()> {
        let rpc = ctx.rpc().await?;
        let balance = rpc.get_balance(self.address.clone()).await.unwrap()?;
        tracing::info!("Balance of {} is {}", self.address, balance,);
        Ok(())
    }
}
