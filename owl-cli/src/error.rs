use std::io;
use tarpc;
use tarpc::util::Message;

#[derive(Debug, Fail)]
pub enum Error {
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

impl From<tarpc::Error<Message>> for Error {
    fn from(e: tarpc::Error<Message>) -> Self {
        match e {
            tarpc::Error::Io(e) => Error::Io(e),
            tarpc::Error::App(message) => Error::RpcMessage(message.to_string()),
            _ => Error::TarpcDeserialization,
        }
    }
}
