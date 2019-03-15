use serde::Deserialize;
use serde_derive::Deserialize;
use reqwest::Client as Http;
use reqwest::Response;

mod utils;
mod error;
pub use error::Error;

use utils::{url, Action, Region};

type Date = String;
type Id = String;
type Permission = String;

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
    id: TokenId,
    inherits_user_perms: bool,
    permissions: Vec<Permission>,
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
        headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(token)?);
        Ok(Client {
            http: reqwest::ClientBuilder::new().default_headers(headers).build()?
        })
    }

    pub fn tokens(&self) -> Result<Vec<Token>, Error> {
        let response = self.http.get(url(Action::Account)).send()?;
        Self::parse(response)?
    }

    pub(crate) fn parse<T: Deserialize>(response: Response) -> Result<T, Error> {
        if response.status().is_success() {
            response.json()?
        } else {
            Err(response.status())
        }
    }
}


fn main() {
    let c = Client::from_token("d0c1a638-e298-443f-82ec-25de4ff9e159").unwrap();
    println!("{:#?}", c.tokens());
}