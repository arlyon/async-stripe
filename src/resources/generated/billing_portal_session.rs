// ======================================
// This file was automatically generated.
// ======================================

use crate::config::{Client, Response};
use crate::ids::{BillingPortalSessionId, CustomerId};
use crate::params::{Expand, Expandable, Object, Timestamp};
use crate::resources::{BillingPortalConfiguration};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "PortalSession".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BillingPortalSession {
    /// Unique identifier for the object.
    pub id: BillingPortalSessionId,

    /// The configuration used by this session, describing the features available.
    pub configuration: Expandable<BillingPortalConfiguration>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The ID of the customer for this session.
    pub customer: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The account for which the session was created on behalf of.
    ///
    /// When specified, only subscriptions and invoices with this `on_behalf_of` account appear in the portal.
    /// For more information, see the [docs](https://stripe.com/docs/connect/charges-transfers#on-behalf-of).
    /// Use the [Accounts API](https://stripe.com/docs/api/accounts/object#account_object-settings-branding) to modify the `on_behalf_of` account's branding settings, which the portal displays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,

    /// The URL to redirect customers to when they click on the portal's link to return to your website.
    pub return_url: String,

    /// The short-lived URL of the session that gives customers access to the customer portal.
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

    /// The [configuration](https://stripe.com/docs/api/customer_portal/configuration) to use for this session, describing its functionality and features.
    ///
    /// If not specified, the session uses the default configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<&'a str>,

    /// The ID of an existing customer.
    pub customer: CustomerId,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// The `on_behalf_of` account to use for this session.
    ///
    /// When specified, only subscriptions and invoices with this `on_behalf_of` account appear in the portal.
    /// For more information, see the [docs](https://stripe.com/docs/connect/charges-transfers#on-behalf-of).
    /// Use the [Accounts API](https://stripe.com/docs/api/accounts/object#account_object-settings-branding) to modify the `on_behalf_of` account's branding settings, which the portal displays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,

    /// The default URL to redirect customers to when they click on the portal's link to return to your website.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
}

impl<'a> CreateBillingPortalSession<'a> {
    pub fn new(customer: CustomerId) -> Self {
        CreateBillingPortalSession {
            configuration: Default::default(),
            customer,
            expand: Default::default(),
            on_behalf_of: Default::default(),
            return_url: Default::default(),
        }
    }
}
