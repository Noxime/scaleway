use crate::{TokenId, Serialize, Deserialize, Client, Method, error::ApiError, Error, header, UserId};

#[derive(Debug)]
pub(crate) enum Endpoint {
    Account(Account),
    Compute(Region, Compute),
}

impl From<Account> for Endpoint {
    fn from(account: Account) -> Endpoint {
        Endpoint::Account(account)
    }
}

impl From<(Region, Compute)> for Endpoint {
    fn from((region, compute): (Region, Compute)) -> Endpoint {
        Endpoint::Compute(region, compute)
    }
}

#[derive(Debug)]
pub enum Region {
    Par1,
    Ams1,
}

#[derive(Debug)]
pub(crate) enum Account {
    Tokens,
    Token(TokenId),
    Organizations,
    Users(UserId),
}

#[derive(Debug)]
pub(crate) enum Compute {
    Servers
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct NilType {}
impl From<()> for NilType {
    fn from(_: ()) -> NilType {
        NilType {}
    }
}

impl Into<()> for NilType {
    fn into(self) -> () {
        ()
    }
}

pub(crate) fn url(action: impl Into<Endpoint>) -> String {
    match action.into() {
        Endpoint::Account(Account::Tokens) => String::from("https://account.scaleway.com/tokens"),
        Endpoint::Account(Account::Token(id)) => format!("https://account.scaleway.com/tokens/{}", id.0),
        Endpoint::Account(Account::Organizations) => String::from("https://account.scaleway.com/organizations"),
        Endpoint::Account(Account::Users(id)) => format!("https://account.scaleway.com/users/{}", id.0),
        Endpoint::Compute(region, v) => format!("https://cp-{}.scaleway.com/{}", match region {
            Region::Par1 => "par1",
            Region::Ams1 => "ams1",
        }, match v {
            Compute::Servers => "servers"
        }),
    }
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


pub(crate) fn request<S, D>(
        client: &Client,
        method: Method,
        url: &str,
        body: Option<S>,
    ) -> Result<D, Error>
    where
        S: Serialize,
        for<'de> D: Deserialize<'de>,
{
    let mut request = client.http.request(method, url)
        .header(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"))
        .json(& NilType {});
    if let Some(body) = body {
        request = request.json(&body);
    }
    println!("request: {:#?}", request);
    let mut response = request.send()?;
    println!("reponse: {:#?}", response);
    Ok(response.json::<ApiResponse<D>>()?.into()?)
}

// TODO: I don't like having this second function, do something about it.
pub(crate) fn request_noret<S>(
        client: &Client,
        method: Method,
        url: &str,
        body: Option<S>,
    ) -> Result<(), Error>
    where
        S: Serialize,
{
    let mut request = client.http.request(method, url)
        .header(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"))
        .json(& NilType {});
    if let Some(body) = body {
        request = request.json(&body);
    }
    let mut r = request.send()?;
    if r.status().is_success() {
        Ok(())
    } else {
        Ok(r.json::<ApiResponse<NilType>>()?.into()?.into())
    }
}