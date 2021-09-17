mod cli;
mod commands;

use commands::Command as _;

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
    init_logging();

    let args = cli::parse_args();
    tracing::trace!("args: {:?}", args);
    let context = commands::Context::new(args.rpc_url);
    args.command.run(context).await?;
    Ok(())
}
