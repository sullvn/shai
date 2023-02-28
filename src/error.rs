use core::fmt::{self, Display, Formatter};
use std::env::VarError;
use std::error;
use std::io;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    NoAPIKey,
    NoResults,
    Ureq(Box<ureq::Error>),
    Var(VarError),
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::IO(err) => err.fmt(f),
            Self::NoAPIKey => "No OpenAI API key. Run `shai help`.".fmt(f),
            Self::NoResults => "No results".fmt(f),
            Self::Ureq(err) => err.fmt(f),
            Self::Var(err) => err.fmt(f),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::IO(err)
    }
}

impl From<ureq::Error> for Error {
    fn from(err: ureq::Error) -> Self {
        Self::Ureq(Box::new(err))
    }
}

impl From<VarError> for Error {
    fn from(err: VarError) -> Self {
        Self::Var(err)
    }
}
