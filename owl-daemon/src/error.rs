use diesel;
use r2d2;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "r2d2 error: {}", _0)]
    R2D2(#[cause] r2d2::Error),
    #[fail(display = "diesel error: {}", _0)]
    Diesel(#[cause] diesel::result::Error),
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
