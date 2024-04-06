use std::fmt::{Display, Formatter};

use crate::AccountId;
use crate::ApiVersion;
use crate::ApplicationId;

#[derive(Clone, Default)]
pub struct AppInfo {
    pub name: String,
    pub url: Option<String>,
    pub version: Option<String>,
}

impl Display for AppInfo {
    /// Formats a plugin's 'App Info' into a string that can be added to the end of an User-Agent string.
    ///
    /// This formatting matches that of other libraries, and if changed then it should be changed everywhere.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match (&self.version, &self.url) {
            (Some(a), Some(b)) => write!(f, "{}/{a} ({b})", &self.name),
            (Some(a), None) => write!(f, "{}/{a}", &self.name),
            (None, Some(b)) => write!(f, "{} ({b})", &self.name),
            _ => f.write_str(&self.name),
        }
    }
}

#[derive(Clone)]
pub struct Headers {
    pub stripe_version: ApiVersion,
    pub user_agent: String,

    pub client_id: Option<ApplicationId>,
    pub stripe_account: Option<AccountId>,
}

impl Headers {
    pub fn to_array(&self) -> [(&str, Option<&str>); 4] {
        [
            ("Client-Id", self.client_id.as_deref()),
            ("Stripe-Account", self.stripe_account.as_deref()),
            ("Stripe-Version", Some(self.stripe_version.as_str())),
            ("User-Agent", Some(&self.user_agent)),
        ]
    }
}
