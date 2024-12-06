use std::fmt;

#[derive(Debug)]
pub enum Error {
    BuilderError(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::BuilderError(err) => write!(f, "BuilderError: {}", err),
        }
    }
}
