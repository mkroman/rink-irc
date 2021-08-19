use std::path::Path;

use irc::client::data::Config as IrcConfig;
use tracing::{debug, instrument};

use crate::{Error, Kind};

/// An opaque config
#[derive(Debug, Clone)]
pub struct Config(IrcConfig);

impl Config {
    /// Attempts to create a [`Config`] by reading values from environment variables.
    #[instrument]
    pub fn from_env() -> Result<Config, Error> {
        debug!("Attempting to create config based on environment variables");

        todo!()
    }

    /// Attempts to load the given `path` as a TOML file and deserialize it into a [`Config`]
    #[instrument(fields(path = ?path.as_ref()))]
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Config, Error> {
        debug!("Loading config file");

        let path = path.as_ref();
        let config = IrcConfig::load(path).map_err(|e| Kind::ConfigLoadFailed(e))?;

        Ok(Config(config))
    }

    /// Returns a string that's useful as a network identifier.
    ///
    /// It has the format `<host>:<port>`
    pub fn identifier(&self) -> String {
        let cfg = &self.0;
        let server = cfg.server.clone().unwrap_or("undefined".to_string());
        let port = cfg.port.unwrap_or(if cfg.use_tls() { 6667 } else { 6697 });

        format!("{}:{}", server, port)
    }
}

impl Into<IrcConfig> for Config {
    /// Returns the inner [`IrcConfig`].
    fn into(self) -> IrcConfig {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn it_should_fail_when_partial_config() {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        write!(
            file,
            r##"
            nickname = "test"
            "##
        )
        .unwrap();

        assert!(Config::from_file(file).is_err());
    }
}
