use crate::TokenId;

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
