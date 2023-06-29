use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    TokioError(tokio::io::Error),
    ReqwestError(reqwest::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::TokioError(err) => write!(f, "Tokio Error: {}", err),
            Self::ReqwestError(err) => write!(f, "Reqwest Error: {}", err),
        }
    }
}

impl From<tokio::io::Error> for Error {
    fn from(err: tokio::io::Error) -> Self {
        Self::TokioError(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::ReqwestError(err)
    }
}
