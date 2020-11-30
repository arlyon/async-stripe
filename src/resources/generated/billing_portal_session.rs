// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{BillingPortalSessionId};
use crate::params::{Object, Timestamp};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "PortalSession".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BillingPortalSession {
    /// Unique identifier for the object.
    pub id: BillingPortalSessionId,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The ID of the customer for this session.
    pub customer: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The URL to which Stripe should send customers when they click on the link to return to your website.
    pub return_url: String,

    /// The short-lived URL of the session giving customers access to the customer portal.
    pub url: String,
}

impl Object for BillingPortalSession {
    type Id = BillingPortalSessionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "billing_portal.session"
    }
}
