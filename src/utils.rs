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
}

pub(crate) fn url(action: impl Into<Endpoint>) -> &'static str {
    match action.into() {
        Endpoint::Account(Account::Tokens) => "https://account.scaleway.com/tokens",
        Endpoint::Compute(Region::Par1) => "https://cp-par1.scaleway.com/",
        Endpoint::Compute(Region::Ams1) => "https://cp-ams1.scaleway.com/",
    }
}
