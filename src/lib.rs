use chrono::{DateTime, FixedOffset};
use reqwest::{header, Method, Client as Http};
use serde::{Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};

mod error;
mod utils;
pub use error::Error;

use utils::{url, Account, request, request_noret, Compute};
pub use utils::Region;

pub type Id = String;
pub type Permission = String;
pub type Date = DateTime<FixedOffset>;
pub type Ip = std::net::IpAddr;

#[derive(Debug)]
pub struct Client {
    http: Http,
    token: TokenId,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TokenId(Id);

impl<T: Into<Id>> From<T> for TokenId {
    fn from(id: T) -> TokenId {
        TokenId(id.into())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationId(Id);

impl<T: Into<Id>> From<T> for OrganizationId {
    fn from(id: T) -> Self {
        Self(id.into())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleId(Id);

impl<T: Into<Id>> From<T> for RoleId {
    fn from(id: T) -> Self {
        Self(id.into())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserId(Id);

impl<T: Into<Id>> From<T> for UserId {
    fn from(id: T) -> Self {
        Self(id.into())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageId(Id);

impl<T: Into<Id>> From<T> for ImageId {
    fn from(id: T) -> Self {
        Self(id.into())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerId(Id);

impl<T: Into<Id>> From<T> for ServerId {
    fn from(id: T) -> Self {
        Self(id.into())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeId(Id);

impl<T: Into<Id>> From<T> for VolumeId {
    fn from(id: T) -> Self {
        Self(id.into())
    }
}

#[derive(Debug, Deserialize)]
pub struct AccessKey(String);
#[derive(Debug, Deserialize)]
pub struct SshKeyId(Id);

impl<T: Into<Id>> From<T> for SshKeyId {
    fn from(id: T) -> Self {
        Self(id.into())
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenCategory {
    UserCreated,
    Session,
}

#[derive(Debug, Deserialize)]
pub struct Roles {
    organization: Option<Organization>,
    role: Role,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Manager,
    Admin,
}

#[derive(Debug, Deserialize)]
pub struct Token {
    pub access_key: AccessKey,
    pub category: TokenCategory,
    pub user_id: UserId,
    pub description: String,
    // pub roles: Roles,
    pub inherits_user_perms: bool,
    pub deletion_date: Option<Date>,
    pub creation_ip: Ip,
    pub expires: Option<Date>,
    pub creation_date: Date,
}

#[derive(Debug, Deserialize)]
pub struct User {
    phone_number: String,
    double_auth_enabled: bool,
    firstname: String,
    lastname: String,
    creation_date: Date,
    ssh_public_keys: Vec<SshKey>,
    id: UserId,
    organizations: Vec<Organization>,
    modification_date: Option<Date>,
    roles: Vec<Roles>,
    fullname: String,
    email: String,

}

#[derive(Debug, Deserialize)]
pub struct SshKey {
    port: u16,
    email: Option<String>,
    // user_agent: {}
    modification_date: Option<Date>,
    key: String,
    fingerprint: String,
    ip: Option<Ip>,
    description: Option<String>,
    id: SshKeyId,
    creation_date: Date,
}

#[derive(Debug, Deserialize)]
pub struct Organization {
    id: OrganizationId,
    name: String,
    users: Option<Vec<User>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CommercialType {
    #[serde(rename = "START1-S")]
    Start1S,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BootType {
    Local,
}

#[derive(Debug, Serialize)]
pub struct ServerOptions {
    organization: Option<OrganizationId>,
    name: String,
    image: ImageId,
    commercial_type: CommercialType,
    tags: Vec<String>,
    enable_ipv6: bool,
    boot_type: BootType
}

#[derive(Debug, Deserialize)]
pub struct Server {
    bootscript: Option<String>, // check this
    dynamic_ip_required: bool,
    id: ServerId,
    image: Image,
    name: String,
    organization: OrganizationId,
    private_ip: Option<Ip>,
    public_ip: Option<Ip>,
    enable_ipv6: bool,
    state: ServerState,
    ipv6: Option<Ip>,
    commercial_type: CommercialType,
    arch: Arch,
    boot_type: BootType,
    tags: Vec<String>,
    volumes: std::collections::HashMap<String, Volume>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ShortServer {
    id: ServerId,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Volume {
    export_uri: Option<String>, // make this an actual uri
    id: VolumeId,
    name: String,
    organization: OrganizationId,
    server: ShortServer,
    size: usize,
    volume_type: VolumeType
}

#[derive(Debug, Deserialize, Serialize)]
pub enum VolumeType {
    #[serde(rename = "l_ssd")]
    LSsd,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Arch {
    #[serde(rename = "x86_64")]
    X86_64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ServerState {
    Stopped,
    Running
}

#[derive(Debug, Deserialize)]
pub struct Image {
    id: ImageId,
    name: String,
}

impl Client {
    /// Login to the scaleway API with your tokens secret key.
    ///
    /// You can create a new token in [Account -> Credentials](https://console.scaleway.com/account/credentials)
    pub fn from_token<Id: Clone + Into<TokenId>>(token: &Id) -> Result<Client, Error> {
        use reqwest::header::{self, HeaderMap};
        let token = &token.clone().into();
        let mut headers = HeaderMap::new();
        headers.insert(
            header::HeaderName::from_static("x-auth-token"),
            header::HeaderValue::from_str(&token.0)?,
        );
        println!("headers: {:#?}", headers);
        Ok(Client {
            http: reqwest::ClientBuilder::new()
                .default_headers(headers)
                .build()?,
            token: token.clone()
        })
    }

    /// Create a new session token and a client from an email and password
    pub fn from_user(email: impl Into<String>, password: impl Into<String>) -> Result<Client, Error> {
        let client = Client {
            http: reqwest::Client::new(),
            token: "".into() // will get replaced, so this is okay
        };
        let (token, _) = Self::create_token(&client, email, password, true)?;
        Self::from_token(&token)
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
            secret_key: TokenId,
        }
        #[derive(Serialize)]
        struct R {
            email: String,
            password: String,
            expires: bool,
        }
        let I { id, token, .. } = request::<R, O>(self, Method::POST, &url(Account::Tokens), Some(R {
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

    // TODO: Disabled for now, something is off?
    /// List all Organizations associate with your account
    pub fn organizations(&self) -> Result<Vec<Organization>, Error> {
        #[derive(Deserialize)]
        struct I {
            organizations: Vec<Organization>,
        }
        Ok(request::<(), I>(self, Method::GET, &url(Account::Organizations), None)?.organizations)
    }

    /// List information about your user account
    pub fn user<Id: Clone + Into<UserId>>(&self, id: &Id) -> Result<User, Error> {
        #[derive(Deserialize)]
        struct I {
            user: User,
        }
        Ok(request::<(), I>(self, Method::GET, &url(Account::Users(id.clone().into())), None)?.user)
    }

    /// Update the SSH keys linked to your user account.
    pub fn update_ssh_keys<Id: Clone + Into<UserId>>(&self, keys: Vec<(String, Option<String>)>, user_id: &Id) -> Result<User, Error> {
        #[derive(Serialize)]
        struct O {
            ssh_public_keys: Vec<I>,
        }
        #[derive(Serialize)]
        struct I {
            key: String,
            description: Option<String>,
        }
        #[derive(Deserialize)]
        struct I2 {
            user: User
        }
        Ok(request::<O, I2>(self, Method::PATCH, &url(Account::Users(user_id.clone().into())), Some(O {
            ssh_public_keys: keys.into_iter().map(|(k, d)| I { key: k, description: d }).collect()
        }))?.user)
    }

    /// Create a new server
    pub fn create_server(&self, region: Region, opts: ServerOptions) -> Result<Server, Error> {
        #[derive(Deserialize)]
        struct I {
            server: Server
        }
        Ok(request::<ServerOptions, I>(self, Method::POST, &url((region, Compute::Servers)), Some(opts))?.server)
    }

    // TODO: We need an implementation of pagination first
    // /// List all servers associated with your account
    // pub fn servers(&self, region: Region) -> Result<Vec<Server>, Error> {
    //     #[derive(Deserialize)]
    //     struct I {
    //         servers: Vec<Server>
    //     }
    //     Ok(request::<(), I>(self, Method::GET, &url((region, Compute::Servers)), None)?.servers)
    // }
}
