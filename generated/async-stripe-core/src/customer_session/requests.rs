use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct CreateCustomerSessionBuilder {
    components: CreateCustomerSessionComponents,
    customer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl CreateCustomerSessionBuilder {
    fn new(
        components: impl Into<CreateCustomerSessionComponents>,
        customer: impl Into<String>,
    ) -> Self {
        Self { components: components.into(), customer: customer.into(), expand: None }
    }
}
/// Configuration for each component. Exactly 1 component must be enabled.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
        Self { buy_button: None, pricing_table: None }
    }
}
impl Default for CreateCustomerSessionComponents {
    fn default() -> Self {
        Self::new()
    }
}
/// Configuration for buy button.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCustomerSessionComponentsBuyButton {
    /// Whether the buy button is enabled.
    pub enabled: bool,
}
impl CreateCustomerSessionComponentsBuyButton {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into() }
    }
}
/// Configuration for the pricing table.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCustomerSessionComponentsPricingTable {
    /// Whether the pricing table is enabled.
    pub enabled: bool,
}
impl CreateCustomerSessionComponentsPricingTable {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into() }
    }
}
/// Creates a customer session object that includes a single-use client secret that you can use on your front-end to grant client-side API access for certain customer resources.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCustomerSession {
    inner: CreateCustomerSessionBuilder,
}
impl CreateCustomerSession {
    /// Construct a new `CreateCustomerSession`.
    pub fn new(
        components: impl Into<CreateCustomerSessionComponents>,
        customer: impl Into<String>,
    ) -> Self {
        Self { inner: CreateCustomerSessionBuilder::new(components.into(), customer.into()) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl CreateCustomerSession {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for CreateCustomerSession {
    type Output = stripe_core::CustomerSession;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/customer_sessions").form(&self.inner)
    }
}
