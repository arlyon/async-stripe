/// Account Links are the means by which a Connect platform grants a connected account permission to access
/// Stripe-hosted applications, such as Connect Onboarding.
///
/// Related guide: [Connect Onboarding](https://stripe.com/docs/connect/connect-onboarding).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AccountLink {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The timestamp at which this account link will expire.
    pub expires_at: stripe_types::Timestamp,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: AccountLinkObject,
    /// The URL for the account link.
    pub url: String,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AccountLinkObject {
    AccountLink,
}

impl AccountLinkObject {
    pub fn as_str(self) -> &'static str {
        use AccountLinkObject::*;
        match self {
            AccountLink => "account_link",
        }
    }
}

impl std::str::FromStr for AccountLinkObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountLinkObject::*;
        match s {
            "account_link" => Ok(AccountLink),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountLinkObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountLinkObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for AccountLinkObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountLinkObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for AccountLinkObject"))
    }
}
pub mod requests;
