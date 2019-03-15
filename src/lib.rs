use chrono::{DateTime, FixedOffset};
use reqwest::Client as Http;
use reqwest::Response;
use serde::Deserialize;
use serde_derive::Deserialize;

mod error;
mod utils;
pub use error::Error;

use utils::{url, Account, Region};

type Id = String;
type Permission = String;
type Date = DateTime<FixedOffset>;

#[derive(Debug)]
pub struct Client {
    http: Http,
}

#[derive(Debug, Deserialize)]
pub struct TokenId(Id);
#[derive(Debug, Deserialize)]
pub struct OrganizationId(Id);
#[derive(Debug, Deserialize)]
pub struct RoleId(Id);
#[derive(Debug, Deserialize)]
pub struct UserId(Id);

#[derive(Debug, Deserialize)]
pub struct Roles {
    organization: Option<OrganizationId>,
    role: Option<RoleId>,
}

#[derive(Debug, Deserialize)]
pub struct Token {
    creation_date: Date,
    expires: Option<Date>,
    id: Option<TokenId>,
    inherits_user_perms: bool,
    permissions: Option<Vec<Permission>>,
    roles: Roles,
    user_id: UserId,
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

    pub fn tokens(&self) -> Result<Vec<Token>, Error> {
        #[derive(Deserialize)]
        struct Inner {
            tokens: Vec<Token>,
        }
        return Ok(self
            .http
            .get(url(Account::Tokens))
            .send()?
            .json::<Inner>()?
            .tokens);
    }
}
