#[derive(Debug)]
pub enum Error {
    TokioError(tokio::io::Error),
    ReqwestError(reqwest::Error),
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
