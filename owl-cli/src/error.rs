use std::io;
use std::num;
use tarpc;
use tarpc::util::Message;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "ParseInt error: {}", _0)]
    ParseInt(#[cause] num::ParseIntError),
    #[fail(display = "io error: {}", _0)]
    Io(#[cause] io::Error),
    #[fail(display = "tarpc deserialization error")]
    TarpcDeserialization,
    #[fail(display = "{}", _0)]
    RpcMessage(String),
    #[fail(display = "not implemented")]
    NotImplemented,
    #[fail(display = "invalid subcommand (assertion failure)")]
    InvalidSubcommand,
}

impl From<num::ParseIntError> for Error {
    fn from(e: num::ParseIntError) -> Self {
        Error::ParseInt(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<tarpc::Error<Message>> for Error {
    fn from(e: tarpc::Error<Message>) -> Self {
        match e {
            tarpc::Error::Io(e) => Error::Io(e),
            tarpc::Error::App(message) => Error::RpcMessage(message.to_string()),
            _ => Error::TarpcDeserialization,
        }
    }
}
