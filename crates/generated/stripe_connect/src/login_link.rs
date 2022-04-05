// ======================================
// This file was automatically generated.
// ======================================

use async_stripe_client::params::Object;
use serde::{Deserialize, Serialize};
use stripe::params::Timestamp;

/// The resource representing a Stripe "LoginLink".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LoginLink {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The URL for the login link.
    pub url: String,
}

impl Object for LoginLink {
    type Id = ();
    fn id(&self) -> Self::Id {}
    fn object(&self) -> &'static str {
        "login_link"
    }
}
