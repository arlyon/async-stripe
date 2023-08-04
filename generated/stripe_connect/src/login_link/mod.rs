/// Login Links are single-use login link for an Express account to access their Stripe dashboard.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LoginLink {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The URL for the login link.
    pub url: String,
}
#[cfg(feature = "login_link")]
mod requests;
#[cfg(feature = "login_link")]
pub use requests::*;
