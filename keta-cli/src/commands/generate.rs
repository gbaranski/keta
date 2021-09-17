use async_trait::async_trait;
use keta_rpc::RpcClient;

#[derive(Debug)]
pub struct Command {}

#[async_trait]
impl super::Command for Command {
    async fn run(self, mut ctx: super::Context) -> anyhow::Result<()> {
        let rpc = ctx.rpc().await?;
        let block = rpc.generate_block().await.unwrap();
        tracing::info!("Generated new block: {:?}", block);
        Ok(())
    }
}
