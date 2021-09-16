mod cli;
mod rpc;
mod world;

const LOG_ENVIRONMENT_VARIABLE: &str = "KETA_LOG";

fn init_logging() {
    use std::env::VarError;
    use std::str::FromStr;
    use tracing_subscriber::EnvFilter;

    let env_filter = match std::env::var(LOG_ENVIRONMENT_VARIABLE) {
        Ok(env) => env,
        Err(VarError::NotPresent) => "info".to_string(),
        Err(VarError::NotUnicode(_)) => panic!(
            "{} environment variable is not valid unicode",
            LOG_ENVIRONMENT_VARIABLE
        ),
    };
    let env_filter = EnvFilter::from_str(&env_filter).unwrap_or_else(|err| {
        panic!(
            "invalid {} environment variable {}",
            LOG_ENVIRONMENT_VARIABLE, err
        )
    });
    tracing_subscriber::fmt().with_env_filter(env_filter).init();
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    use world::World;

    init_logging();
    let args = cli::parse_args();
    tracing::trace!("args: {:?}", args);
    let database = keta_node_db::Database::new(args.database)?;
    let world = World::new(database)?;
    let rpc_server = rpc::Server::new(world);
    tracing::info!("Start RPC-Server at {}", &args.rpc_address);
    rpc_server.run(&args.rpc_address).await?;
    Ok(())
}
