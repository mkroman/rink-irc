use std::path::Path;

use irc::client::data::Config as IrcConfig;
use tracing::{debug, instrument};

use crate::{Error, Kind};

#[derive(Debug)]
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
