use crate::{TokenId, Serialize, Deserialize, Client, Method, Error, ApiResponse, header};

#[derive(Debug)]
pub(crate) enum Endpoint {
    Account(Account),
    Compute(Region),
}

impl From<Account> for Endpoint {
    fn from(account: Account) -> Endpoint {
        Endpoint::Account(account)
    }
}

impl From<Region> for Endpoint {
    fn from(region: Region) -> Endpoint {
        Endpoint::Compute(region)
    }
}

#[derive(Debug)]
pub(crate) enum Region {
    Par1,
    Ams1,
}

#[derive(Debug)]
pub(crate) enum Account {
    Tokens,
    Token(TokenId),
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
        Endpoint::Account(Account::Token(id)) => {
            format!("https://account.scaleway.com/tokens/{}", id.0)
        }
        Endpoint::Compute(Region::Par1) => String::from("https://cp-par1.scaleway.com/"),
        Endpoint::Compute(Region::Ams1) => String::from("https://cp-ams1.scaleway.com/"),
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
    Ok(request.send()?.json::<ApiResponse<D>>()?.into()?)
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