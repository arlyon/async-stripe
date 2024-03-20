#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCustomerSession<'a> {
    /// Configuration for each component. Exactly 1 component must be enabled.
    pub components: CreateCustomerSessionComponents,
    /// The ID of an existing customer for which to create the customer session.
    pub customer: &'a str,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> CreateCustomerSession<'a> {
    pub fn new(components: CreateCustomerSessionComponents, customer: &'a str) -> Self {
        Self { components, customer, expand: None }
    }
}
/// Configuration for each component. Exactly 1 component must be enabled.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCustomerSessionComponents {
    /// Configuration for buy button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buy_button: Option<CreateCustomerSessionComponentsBuyButton>,
    /// Configuration for the pricing table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_table: Option<CreateCustomerSessionComponentsPricingTable>,
}
impl CreateCustomerSessionComponents {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration for buy button.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCustomerSessionComponentsBuyButton {
    /// Whether the buy button is enabled.
    pub enabled: bool,
}
impl CreateCustomerSessionComponentsBuyButton {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Configuration for the pricing table.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCustomerSessionComponentsPricingTable {
    /// Whether the pricing table is enabled.
    pub enabled: bool,
}
impl CreateCustomerSessionComponentsPricingTable {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
impl<'a> CreateCustomerSession<'a> {
    /// Creates a customer session object that includes a single-use client secret that you can use on your front-end to grant client-side API access for certain customer resources.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_core::CustomerSession> {
        client.send_form("/customer_sessions", self, http_types::Method::Post)
    }
}
