use crate::commands;
use async_trait::async_trait;
use clap::App;
use clap::Arg;
use url::Url;
use clap::SubCommand;
use std::str::FromStr;

const DEFAULT_RPC_URL: &str = "ws://localhost:5454";

#[derive(Debug)]
pub struct Args {
    pub rpc_url: Url,
    pub command: Command,
}

#[derive(Debug)]
pub enum Command {
    Balance(commands::Balance),
    Generate(commands::Generate),
    Pay(commands::Pay),
}

#[async_trait]
impl commands::Command for Command {
    async fn run(self, ctx: commands::Context) -> anyhow::Result<()> {
        match self {
            Command::Balance(command) => command.run(ctx),
            Command::Generate(command) => command.run(ctx),
            Command::Pay(command) => command.run(ctx),
        }.await
    }
}

pub fn parse_args() -> Args {
    let matches = App::new("keta-cli")
        .bin_name(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("keta cli implementation")
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("rpc-url")
                .long("rpc-url")
                .help("RPC URL")
                .default_value(DEFAULT_RPC_URL),
        )
        .subcommand(balance())
        .get_matches();

    let (name, sub_matches) = match matches.subcommand() {
        (name, Some(sub_matches)) => (name, sub_matches),
        _ => panic!("sub matches of subcommand is None"),
    };
    let command = match name {
        "balance" => {
            use keta_core::account;

            let address = sub_matches.value_of("account-address").unwrap();

            Command::Balance(commands::Balance {
                address: account::Address::from_str(address).unwrap(),
            })
        }
        _ => panic!("unexpected command"),
    };

    Args {
        rpc_url: matches.value_of("rpc-url").unwrap().parse().unwrap(),
        command,
    }
}

fn balance() -> App<'static, 'static> {
    SubCommand::with_name("balance")
        .about("View balance of an account")
        .arg(
            Arg::with_name("account-address")
                .help("Address of which balance to check")
                .required(true)
                .takes_value(true),
        )
}
