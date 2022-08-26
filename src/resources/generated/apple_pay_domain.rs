// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{ApplePayDomainId};
use crate::params::{Object, Timestamp};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "ApplePayDomain".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ApplePayDomain {
    /// Unique identifier for the object.
    pub id: ApplePayDomainId,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livemode: Option<bool>,
}

impl Object for ApplePayDomain {
    type Id = ApplePayDomainId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "apple_pay_domain"
    }
}
