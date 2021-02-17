// ======================================
// This file was automatically generated.
// ======================================

use crate::config::{Client, Response};
use crate::ids::{BillingPortalSessionId, CustomerId};
use crate::params::{Expand, Object, Timestamp};
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

impl BillingPortalSession {

    /// Creates a session of the customer portal.
    pub fn create(client: &Client, params: CreateBillingPortalSession<'_>) -> Response<BillingPortalSession> {
        client.post_form("/billing_portal/sessions", &params)
    }
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

/// The parameters for `BillingPortalSession::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreateBillingPortalSession<'a> {

    /// The ID of an existing customer.
    pub customer: CustomerId,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// The URL to which Stripe should send customers when they click on the link to return to your website.
    ///
    /// This field is required if a default return URL has not been configured for the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
}

impl<'a> CreateBillingPortalSession<'a> {
    pub fn new(customer: CustomerId) -> Self {
        CreateBillingPortalSession {
            customer,
            expand: Default::default(),
            return_url: Default::default(),
        }
    }
}
