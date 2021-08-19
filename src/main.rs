use std::env;

use clap::Clap;
use color_eyre::{eyre::WrapErr, Report};
use futures::stream::StreamExt;
use irc::client::prelude::*;
use tracing::{debug, instrument, trace};
use tracing_error::ErrorLayer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

mod cli;
mod config;
mod error;
mod rink;

use config::Config;
use error::{Error, Kind};

fn setup() -> Result<(), Report> {
    // Override RUST_LOG with a default setting if it's not set by the user
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "rink_irc=trace");
    }

    color_eyre::install()?;

    let fmt = tracing_subscriber::fmt::layer();
    let filter = EnvFilter::from_default_env();
    let collector = tracing_subscriber::Registry::default()
        .with(ErrorLayer::default())
        .with(filter)
        .with(fmt);

    tracing::subscriber::set_global_default(collector)
        .with_context(|| "setting global collector")?;

    Ok(())
}

#[instrument(skip(config), fields(id = %config.identifier()))]
async fn connect(config: Config) -> Result<(), Report> {
    // Connect to the network
    let mut client = Client::from_config(config.into()).await?;

    // Register our nickname and user
    client.identify()?;

    // Start streaming messages
    let mut stream = client.stream()?;

    while let Some(msg) = stream.next().await.transpose()? {
        trace!(?msg, "Received IRC message");
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Report> {
    setup()?;

    let opts = cli::Opts::parse();
    let config = if let Some(ref config_path) = opts.config {
        Config::from_file(config_path)
    } else {
        Config::from_env()
    }?;

    debug!(?config, "Config loaded");

    debug!("Connecting to network");
    connect(config).await?;

    Ok(())
}
