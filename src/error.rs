use std::fmt;

#[derive(Debug)]
pub enum Error {
    BuilderError(String),
    IoError(std::io::Error),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::BuilderError(err) => write!(f, "BuilderError: {}", err),
            Error::IoError(err) => write!(f, "IoError: {}", err),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IoError(err)
    }
}
