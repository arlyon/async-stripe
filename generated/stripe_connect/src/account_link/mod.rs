/// Account Links are the means by which a Connect platform grants a connected account permission to access
/// Stripe-hosted applications, such as Connect Onboarding.
///
/// Related guide: [Connect Onboarding](https://stripe.com/docs/connect/custom/hosted-onboarding).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AccountLink {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The timestamp at which this account link will expire.
    pub expires_at: stripe_types::Timestamp,
    /// The URL for the account link.
    pub url: String,
}
#[cfg(feature = "account_link")]
mod requests;
#[cfg(feature = "account_link")]
pub use requests::*;
