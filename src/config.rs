use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde::{de::DeserializeOwned, Deserialize};
use tracing::{debug, instrument};

use crate::{Error, Kind};

#[derive(Debug, Deserialize)]
pub struct Config {
    /// The client's nickname
    pub nickname: String,
    /// The client's username, if not set, `nickname` will be used
    pub username: Option<String>,
    /// The client's real name, if not set, `username` will be used
    pub realname: Option<String>,
    /// The servers hostname
    pub server: String,
    /// The servers port number
    pub port: Option<u16>,
    /// A list of channels to join when connection is established
    pub channels: Vec<String>,
}

/// Reads the file at `path` and tries to deserialize into `T` as TOML.
fn read_toml_file<P, T>(path: P) -> Result<T, Error>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let mut buf = String::new();
    File::open(path)?.read_to_string(&mut buf)?;
    let res = toml::from_str(&buf).map_err(|e| Error::from(Kind::ConfigFileParseError(e)))?;

    Ok(res)
}

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
        let config: Config = read_toml_file(path)?;

        Ok(config)
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
