use reqwest::Error as ReqwestError;
use reqwest::header::InvalidHeaderValue;

#[derive(Debug)]
pub enum Error {
    /// provided token was not in a valid format
    InvalidToken,
    Reqwest(ReqwestError),
}

impl From<InvalidHeaderValue> for Error {
    fn from(_: InvalidHeaderValue) -> Error {
        Error::InvalidToken
    }
}

impl From<ReqwestError> for Error {
    fn from(e: ReqwestError) -> Error {
        Error::Reqwest(e)
    }
}
