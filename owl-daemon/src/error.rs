use std::io;

use diesel;
use exploit::ExploitError;
use r2d2;
use toml;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "r2d2 error: {}", _0)]
    R2D2(#[cause] r2d2::Error),
    #[fail(display = "diesel error: {}", _0)]
    Diesel(#[cause] diesel::result::Error),
    #[fail(display = "I/O error: {}", _0)]
    Io(#[cause] io::Error),
    #[fail(display = "Toml deserialization error: {}", _0)]
    TomlDe(#[cause] toml::de::Error),
    #[fail(display = "Exploit error: {}", _0)]
    ExploitError(ExploitError),
    #[fail(display = "Permission error, check your token")]
    PermissionError,
    #[fail(display = "Unknown error")]
    Unknown,
    #[fail(display = "error: {}", _0)]
    Message(String),
}

impl From<r2d2::Error> for Error {
    fn from(e: r2d2::Error) -> Self {
        Error::R2D2(e)
    }
}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        Error::Diesel(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Error::TomlDe(e)
    }
}

impl From<()> for Error {
    fn from(_: ()) -> Self {
        Error::Unknown
    }
}
