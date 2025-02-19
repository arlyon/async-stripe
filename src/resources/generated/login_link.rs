// ======================================
// This file was automatically generated.
// ======================================

use crate::params::{Object, Timestamp};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "LoginLink".
///
/// For more details see <https://stripe.com/docs/api/accounts/login_link/object>
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
