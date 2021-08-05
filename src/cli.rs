use std::path::PathBuf;

use clap::Clap;

#[derive(Clap, Debug)]
pub struct Opts {
    /// Path to a TOML config file
    #[clap(short, long, value_name = "config.toml")]
    pub config: Option<PathBuf>,
}
