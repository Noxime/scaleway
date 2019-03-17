use reqwest::header::InvalidHeaderValue;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;
use serde_derive::Deserialize;

#[derive(Debug)]
pub enum Error {
    /// provided token was not in a valid format
    InvalidToken,
    Reqwest(ReqwestError),
    Serde(SerdeError),
    Api(ApiError),
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    fields: Option<std::collections::HashMap<String, Vec<String>>>,
    message: String,
    r#type: ApiErrorType,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ApiErrorType {
    InvalidRequestError,
    InvalidAuth,
    AuthorizationRequired,
    UnknownResource,
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

impl From<ApiError> for Error {
    fn from(e: ApiError) -> Error {
        Error::Api(e)
    }
}
