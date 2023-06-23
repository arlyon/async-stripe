/// Account Links are the means by which a Connect platform grants a connected account permission to access
/// Stripe-hosted applications, such as Connect Onboarding.
///
/// Related guide: [Connect Onboarding](https://stripe.com/docs/connect/connect-onboarding).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
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
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AccountLink {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum AccountLinkObject {
    AccountLink,
}

impl AccountLinkObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AccountLink => "account_link",
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
pub mod requests;
