use std::{
    error::Error as StdError, fmt, io::Error as IoError,
    result::Result as StdResult,
};

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    ConfigError(String),
    OtherError(String),
    IoError(IoError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OtherError(s) | Self::ConfigError(s) => f.write_str(s),
            Self::IoError(e) => fmt::Display::fmt(e, f),
        }
    }
}

impl StdError for Error {
    fn cause(&self) -> Option<&dyn StdError> {
        match self {
            Self::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Self::IoError(e)
    }
}

impl From<&str> for Error {
    fn from(msg: &str) -> Self {
        Self::OtherError(msg.to_string())
    }
}
