use clap::App;
use clap::Arg;

fn base_directories() -> xdg::BaseDirectories {
    xdg::BaseDirectories::with_prefix("keta-node").unwrap()
}

fn default_database_path() -> std::path::PathBuf {
    base_directories().get_data_home().join("database")
}

fn default_rpc_port() -> u16 {
    5454
}

fn default_rpc_address() -> std::net::SocketAddr {
    std::net::SocketAddrV4::new(std::net::Ipv4Addr::LOCALHOST, default_rpc_port()).into()
}

#[derive(Debug)]
pub struct Args {
    pub database: std::path::PathBuf,
    pub rpc_address: std::net::SocketAddr,
}

pub fn parse_args() -> Args {
    let default_database_path = default_database_path();
    let default_rpc_address = default_rpc_address().to_string();
    let matches = App::new("keta-node")
        .bin_name(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("Node implementation of keta")
        .arg(
            Arg::with_name("database")
                .long("database")
                .help("Path to database")
                .default_value(default_database_path.as_os_str().to_str().unwrap()),
        )
        .arg(
            Arg::with_name("rpc-address")
                .long("rpc-address")
                .help("RPC listen address")
                .default_value(default_rpc_address.as_str()),
        )
        .get_matches();

    Args {
        database: matches.value_of("database").unwrap().parse().unwrap(),
        rpc_address: matches.value_of("rpc-address").unwrap().parse().unwrap(),
    }
}
