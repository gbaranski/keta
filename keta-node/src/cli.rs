use clap::App;
use clap::Arg;

fn base_directories() -> xdg::BaseDirectories {
    xdg::BaseDirectories::with_prefix("keta-node").unwrap()
}

fn default_database_path() -> std::path::PathBuf {
    base_directories().get_data_home().join("database")
}

pub struct Args {
    pub database: std::path::PathBuf,
}

pub fn parse_args() -> Args {
    let default_database_path = default_database_path();
    let default_database_path = default_database_path.as_os_str().to_str().unwrap();
    let matches = App::new("keta-node")
        .bin_name(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("Node implementation of keta")
        .arg(
            Arg::with_name("database")
                .long("database")
                .help("Path to database")
                .default_value(default_database_path),
        )
        .get_matches();
    Args {
        database: matches.value_of("database").unwrap().parse().unwrap(),
    }
}
