
#[derive(Debug)]
pub(crate) enum Action {
    Account,
    Compute(Region)
}

impl From<Region> for Action {
    fn from(region: Region) -> Action {
        Action::Compute(region)
    }
}

#[derive(Debug)]
pub(crate) enum Region {
    Par1,
    Ams1,
}

pub(crate) fn url(action: impl Into<Action>) -> &'static str {
    match action.into() {
        Action::Account => "https://account.scaleway.com/",
        Action::Compute(Region::Par1) => "https://cp-par1.scaleway.com/",
        Action::Compute(Region::Ams1) => "https://cp-ams1.scaleway.com/",
    }
}