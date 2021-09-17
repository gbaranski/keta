use async_trait::async_trait;
use keta_core::account;
use keta_core::transaction::Transaction;
use keta_crypto::Keypair;
use keta_rpc::RpcClient;

#[derive(Debug)]
pub struct Command {
    pub keypair: Keypair,
    pub to: account::Address,
    pub value: u64,
}

#[async_trait]
impl super::Command for Command {
    async fn run(self, mut ctx: super::Context) -> anyhow::Result<()> {
        let rpc = ctx.rpc().await?;
        let transaction = Transaction {
            from: self.keypair.public.clone(),
            to: self.to,
            value: self.value,
        };
        let transaction = transaction.sign(&self.keypair);
        rpc.send_transaction(transaction.clone()).await.unwrap()?;
        tracing::info!(
            "Sent transaction to {} with value: {}",
            transaction.to,
            transaction.value
        );
        Ok(())
    }
}
