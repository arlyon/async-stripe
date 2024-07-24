use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveForMyAccountBalanceBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveForMyAccountBalanceBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the current account balance, based on the authentication that was used to make the request.
/// For a sample request, see [Accounting for negative balances](https://stripe.com/docs/connect/account-balances#accounting-for-negative-balances).
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveForMyAccountBalance<'a> {
    inner: RetrieveForMyAccountBalanceBuilder<'a>,
}
impl<'a> RetrieveForMyAccountBalance<'a> {
    /// Construct a new `RetrieveForMyAccountBalance`.
    pub fn new() -> Self {
        Self { inner: RetrieveForMyAccountBalanceBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl<'a> Default for RetrieveForMyAccountBalance<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl RetrieveForMyAccountBalance<'_> {
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

impl StripeRequest for RetrieveForMyAccountBalance<'_> {
    type Output = stripe_core::Balance;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/balance").query(&self.inner)
    }
}
