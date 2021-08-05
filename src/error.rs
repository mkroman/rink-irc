use std::fmt;
use std::io;

use tracing_error::TracedError;

/// Error wrapper that wraps an error [`Kind`] in a [`TracedError`] to preserve error spans in
/// tracing events.
#[derive(Debug, thiserror::Error)]
pub struct Error {
    source: TracedError<Kind>,
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.source, fmt)
    }
}

impl<E> From<E> for Error
where
    Kind: From<E>,
{
    fn from(source: E) -> Self {
        Self {
            source: Kind::from(source).into(),
        }
    }
}

/// The possible types of errors.
#[derive(Debug, thiserror::Error)]
pub enum Kind {
    /// There was an error when trying to parse the TOML config file
    #[error("Could not load the config file")]
    ConfigLoadFailed(#[from] irc::error::Error),
    /// Non-specialized IO error
    #[error("I/O error")]
    IoError(#[from] io::Error),
}
