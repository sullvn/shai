use core::fmt::{self, Display, Formatter};
use std::env::VarError;
use std::error;
use std::io;
use ureq;

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    NoAPIKey,
    NoResults,
    UreqError(ureq::Error),
    VarError(VarError),
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::IOError(err) => err.fmt(f),
            Self::NoAPIKey => "No OpenAI API key. Run `shai help`.".fmt(f),
            Self::NoResults => "No results".fmt(f),
            Self::UreqError(err) => err.fmt(f),
            Self::VarError(err) => err.fmt(f),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::IOError(err)
    }
}

impl From<ureq::Error> for Error {
    fn from(err: ureq::Error) -> Self {
        Self::UreqError(err)
    }
}

impl From<VarError> for Error {
    fn from(err: VarError) -> Self {
        Self::VarError(err)
    }
}
