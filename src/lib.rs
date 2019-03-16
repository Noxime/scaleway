use chrono::{DateTime, FixedOffset};
use reqwest::Client as Http;
use reqwest::Response;
use serde::Deserialize;
use serde_derive::Deserialize;

mod error;
mod utils;
pub use error::Error;

use utils::{url, Account, Region};

pub trait TryFrom<T>
where
    Self: Sized,
{
    fn try_from(other: T) -> Option<Self>;
}

type Id = String;
type Permission = String;
type Date = DateTime<FixedOffset>;
type Ip = std::net::IpAddr;

#[derive(Debug)]
pub struct Client {
    http: Http,
}

#[derive(Debug, Deserialize)]
pub struct TokenId(Id);

impl<T: Into<Id>> From<T> for TokenId {
    fn from(id: T) -> TokenId {
        TokenId(id.into())
    }
}


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

    /// Authenticates a user against their email, password, and then returns a new Token, which can be used until it expires.
    pub fn create_token(&self, email: impl Into<String>, password: impl Into<String>, expires: Option<Date>) -> Result<(TokenId, Token), Error> {
        #[derive(Deserialize)]
        struct Outer {
            token: Inner,
        }
        #[derive(Deserialize)]

        struct Inner {
            id: TokenId,
            #[serde(flatten)]
            token: Token,
        }
        let r = self
            .http
            .post(&url(Account::Tokens))
            .json(&{
                let mut m = std::collections::HashMap::new();
                m.insert("email", email.into());
                m.insert("password", password.into());
                m
            })
            .send()?
            .json::<Outer>()?.token;
        Ok((r.id, r.token))
    }

    /// List all Tokens associated with your account
    pub fn tokens(&self) -> Result<Vec<Token>, Error> {
        #[derive(Deserialize)]
        struct Inner {
            tokens: Vec<Token>,
        }
        // the wrath of serenitycord
        Ok(self
            .http
            .get(&url(Account::Tokens))
            .send()?
            .json::<Inner>()?
            .tokens)
    }

    /// List an individual Token
    pub fn token(&self, id: impl Into<TokenId>) -> Result<Token, Error> {
        #[derive(Deserialize)]
        struct Inner {
            token: Token,
        }
        Ok(self
            .http
            .get(&url(Account::Token(id.into())))
            .send()?
            .json::<Inner>()?
            .token)
    }

    /// Increase Token expiration time of 30 minutes
    pub fn renew_token(&self, id: impl Into<TokenId>) -> Result<Token, Error> {
        #[derive(Deserialize)]
        struct Inner {
            token: Token,
        }
        Ok(self
            .http
            .patch(&url(Account::Token(id.into())))
            .send()?
            .json::<Inner>()?
            .token)
    }
}
