// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::CustomerId;
use crate::params::{Expand, Expandable, Object, Timestamp};
use crate::resources::Customer;
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "CustomerSessionResourceCustomerSession".
///
/// For more details see <https://stripe.com/docs/api/customer_sessions/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CustomerSession {
    /// The client secret of this customer session.
    ///
    /// Used on the client to set up secure access to the given `customer`.  The client secret can be used to provide access to `customer` from your frontend.
    /// It should not be stored, logged, or exposed to anyone other than the relevant customer.
    /// Make sure that you have TLS enabled on any page that includes the client secret.
    pub client_secret: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<CustomerSessionResourceComponents>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The customer the customer session was created for.
    pub customer: Expandable<Customer>,

    /// The timestamp at which this customer session will expire.
    pub expires_at: Timestamp,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
}

impl CustomerSession {
    /// Creates a customer session object that includes a single-use client secret that you can use on your front-end to grant client-side API access for certain customer resources.
    pub fn create(client: &Client, params: CreateCustomerSession<'_>) -> Response<CustomerSession> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form("/customer_sessions", &params)
    }
}

impl Object for CustomerSession {
    type Id = ();
    fn id(&self) -> Self::Id {}
    fn object(&self) -> &'static str {
        "customer_session"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CustomerSessionResourceComponents {
    pub buy_button: CustomerSessionResourceComponentsResourceBuyButton,

    pub pricing_table: CustomerSessionResourceComponentsResourcePricingTable,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CustomerSessionResourceComponentsResourceBuyButton {
    /// Whether the buy button is enabled.
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CustomerSessionResourceComponentsResourcePricingTable {
    /// Whether the pricing table is enabled.
    pub enabled: bool,
}

/// The parameters for `CustomerSession::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreateCustomerSession<'a> {
    /// Configuration for each component.
    ///
    /// Exactly 1 component must be enabled.
    pub components: CreateCustomerSessionComponents,

    /// The ID of an existing customer for which to create the customer session.
    pub customer: CustomerId,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],
}

impl<'a> CreateCustomerSession<'a> {
    pub fn new(components: CreateCustomerSessionComponents, customer: CustomerId) -> Self {
        CreateCustomerSession { components, customer, expand: Default::default() }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCustomerSessionComponents {
    /// Configuration for buy button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buy_button: Option<CreateCustomerSessionComponentsBuyButton>,

    /// Configuration for the pricing table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_table: Option<CreateCustomerSessionComponentsPricingTable>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCustomerSessionComponentsBuyButton {
    /// Whether the buy button is enabled.
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCustomerSessionComponentsPricingTable {
    /// Whether the pricing table is enabled.
    pub enabled: bool,
}
