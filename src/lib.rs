use chrono::{DateTime, FixedOffset};
use reqwest::{header, Method, Response, Client as Http};
use serde::{Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};

mod error;
mod utils;
pub use error::Error;

use utils::{url, Account, request, request_noret};

pub trait TryFrom<T>
where
    Self: Sized,
{
    fn try_from(other: T) -> Option<Self>;
}

pub type Id = String;
pub type Permission = String;
pub type Date = DateTime<FixedOffset>;
pub type Ip = std::net::IpAddr;

#[derive(Debug)]
pub struct Client {
    http: Http,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TokenId(Id);

impl<T: Into<Id>> From<T> for TokenId {
    fn from(id: T) -> TokenId {
        TokenId(id.into())
    }
}

// impl<T: Into<TokenId>> From<&T> for TokenId {
//     fn from(id: &T) -> TokenId {
//         unimplemented!()
//     }
// }

// impl<T: Into<TokenId>> From<&T> for TokenId {
//     fn from(id: &T) -> TokenId {
//         TokenId(id.clone().into())
//     }
// }

#[derive(Debug, Deserialize)]
pub struct OrganizationId(Id);
#[derive(Debug, Deserialize)]
pub struct RoleId(Id);
#[derive(Debug, Deserialize)]
pub struct UserId(Id);
#[derive(Debug, Deserialize)]
pub struct AccessKey(String);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenCategory {
    UserCreated,
    Session,
}

#[derive(Debug, Deserialize)]
pub struct Roles {
    organization: Option<OrganizationId>,
    role: Option<RoleId>,
}

#[derive(Debug, Deserialize)]
pub struct Token {
    pub access_key: AccessKey,
    pub category: TokenCategory,
    pub user_id: UserId,
    pub description: String,
    pub roles: Roles,
    pub inherits_user_perms: bool,
    pub deletion_date: Option<Date>,
    pub creation_ip: Ip,
    pub expires: Option<Date>,
    pub creation_date: Date,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ApiResponse<T> {
    Success(T),
    Error(ApiError),
}

impl<T> ApiResponse<T> {
    fn into(self) -> Result<T, Error> {
        match self {
            ApiResponse::Success(v) => Ok(v),
            ApiResponse::Error(e) => Err(Error::Api(e))
        }
    }
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

impl Client {
    /// Login to the scaleway API with your tokens secret key.
    ///
    /// You can create a new token in [Account -> Credentials](https://console.scaleway.com/account/credentials)
    pub fn from_token(token: &str) -> Result<Client, Error> {
        use reqwest::header::{self, HeaderMap};
        let mut headers = HeaderMap::new();
        headers.insert(
            header::HeaderName::from_static("x-auth-token"),
            header::HeaderValue::from_str(token)?,
        );
        println!("headers: {:#?}", headers);
        Ok(Client {
            http: reqwest::ClientBuilder::new()
                .default_headers(headers)
                .build()?,
        })
    }

    /// Authenticates a user against their email, password, and then returns a new Token.
    /// 
    /// Note: Expires field determines if the created token is either Session or UserCreated token.
    /// Session tokens expire after an hour (unless renewed), while UserCreated ones never expire.
    pub fn create_token(
        &self,
        email: impl Into<String>,
        password: impl Into<String>,
        expires: bool,
    ) -> Result<(TokenId, Token), Error> {
        #[derive(Deserialize, Debug)]
        struct O {
            token: I,
        }
        #[derive(Deserialize, Debug)]
        struct I {
            id: TokenId,
            #[serde(flatten)]
            token: Token,
        }
        #[derive(Serialize)]
        struct R {
            email: String,
            password: String,
            expires: bool,
        }
        let I { id, token } = request::<R, O>(self, Method::POST, &url(Account::Tokens), Some(R {
            email: email.into(),
            password: password.into(),
            expires,
        }))?.token;
        Ok((id, token))
    }

    /// List all Tokens associated with your account
    pub fn tokens(&self) -> Result<Vec<Token>, Error> {
        #[derive(Deserialize)]
        struct I {
            tokens: Vec<Token>,
        }
        Ok(request::<(), I>(self, Method::GET, &url(Account::Tokens), None)?.tokens)
    }

    /// List an individual Token
    pub fn token<Id: Clone + Into<TokenId>>(&self, id: &Id) -> Result<Token, Error> {
        #[derive(Deserialize)]
        struct I {
            token: Token,
        }
        Ok(request::<(), I>(self, Method::GET, &url(Account::Token(id.clone().into())), None)?.token)
    }

    /// Increase Token expiration time of 30 minutes
    /// 
    /// Errors: This errors out if token does not have an expiration date set
    pub fn renew_token<Id: Clone + Into<TokenId>>(&self, id: &Id) -> Result<Token, Error> {
        #[derive(Deserialize)]
        struct I {
            token: Token,
        }
        Ok(request::<(), I>(self, Method::PATCH, &url(Account::Token(id.clone().into())), None)?.token)
    }

    /// Delete an individual token
    pub fn delete_token<Id: Clone + Into<TokenId>>(&self, id: &Id) -> Result<(), Error> {
        request_noret::<()>(self, Method::DELETE, &url(Account::Token(id.clone().into())), None)
    }
}
