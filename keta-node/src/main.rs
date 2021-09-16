mod cli;
mod world;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = cli::parse_args();
    let database = keta_node_db::Database::new(args.database)?;
    Ok(())
}
