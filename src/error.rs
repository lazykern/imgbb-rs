use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    ReqwestError(reqwest::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::IOError(err) => write!(f, "IO Error: {}", err),
            Self::ReqwestError(err) => write!(f, "Reqwest Error: {}", err),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::ReqwestError(err)
    }
}
