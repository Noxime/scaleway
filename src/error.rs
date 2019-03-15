use reqwest::header::InvalidHeaderValue;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;

#[derive(Debug)]
pub enum Error {
    /// provided token was not in a valid format
    InvalidToken,
    Reqwest(ReqwestError),
    Serde(SerdeError),
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

impl From<SerdeError> for Error {
    fn from(e: SerdeError) -> Error {
        Error::Serde(e)
    }
}
