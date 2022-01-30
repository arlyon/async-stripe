// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::ids::ThreeDSecureId;
use crate::params::{Object, Timestamp};
use crate::resources::{Card, Currency};

/// The resource representing a Stripe "ThreeDSecure".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ThreeDSecure {
    /// Unique identifier for the object.
    pub id: ThreeDSecureId,

    /// Amount of the charge that you will create when authentication completes.
    pub amount: i64,

    /// True if the cardholder went through the authentication flow and their bank indicated that authentication succeeded.
    pub authenticated: bool,

    pub card: Card,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// If present, this is the URL that you should send the cardholder to for authentication.
    ///
    /// If you are going to use Stripe.js to display the authentication page in an iframe, you should use the value "_callback".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<Box<String>>,

    /// Possible values are `redirect_pending`, `succeeded`, or `failed`.
    ///
    /// When the cardholder can be authenticated, the object starts with status `redirect_pending`.
    /// When liability will be shifted to the cardholder's bank (either because the cardholder was successfully authenticated, or because the bank has not implemented 3D Secure, the object wlil be in status `succeeded`.
    /// `failed` indicates that authentication was attempted unsuccessfully.
    pub status: String,
}

impl Object for ThreeDSecure {
    type Id = ThreeDSecureId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "three_d_secure"
    }
}
