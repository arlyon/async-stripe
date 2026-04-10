use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct FindTaxAssociationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    payment_intent: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FindTaxAssociationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FindTaxAssociationBuilder").finish_non_exhaustive()
    }
}
impl FindTaxAssociationBuilder {
    fn new(payment_intent: impl Into<String>) -> Self {
        Self { expand: None, payment_intent: payment_intent.into() }
    }
}
/// Finds a tax association object by PaymentIntent id.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct FindTaxAssociation {
    inner: FindTaxAssociationBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FindTaxAssociation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FindTaxAssociation").finish_non_exhaustive()
    }
}
impl FindTaxAssociation {
    /// Construct a new `FindTaxAssociation`.
    pub fn new(payment_intent: impl Into<String>) -> Self {
        Self { inner: FindTaxAssociationBuilder::new(payment_intent.into()) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl FindTaxAssociation {
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

impl StripeRequest for FindTaxAssociation {
    type Output = stripe_misc::TaxAssociation;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/tax/associations/find").query(&self.inner)
    }
}
