use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveMandateBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveMandateBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a Mandate object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveMandate<'a> {
    inner: RetrieveMandateBuilder<'a>,
    mandate: &'a stripe_shared::MandateId,
}
impl<'a> RetrieveMandate<'a> {
    /// Construct a new `RetrieveMandate`.
    pub fn new(mandate: &'a stripe_shared::MandateId) -> Self {
        Self { mandate, inner: RetrieveMandateBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveMandate<'_> {
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

impl StripeRequest for RetrieveMandate<'_> {
    type Output = stripe_shared::Mandate;

    fn build(&self) -> RequestBuilder {
        let mandate = self.mandate;
        RequestBuilder::new(StripeMethod::Get, format!("/mandates/{mandate}")).query(&self.inner)
    }
}
