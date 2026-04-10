use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrieveForMyAccountBalanceBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveForMyAccountBalanceBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveForMyAccountBalanceBuilder").finish_non_exhaustive()
    }
}
impl RetrieveForMyAccountBalanceBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the current account balance, based on the authentication that was used to make the request.
/// For a sample request, see [Accounting for negative balances](https://stripe.com/docs/connect/account-balances#accounting-for-negative-balances).
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrieveForMyAccountBalance {
    inner: RetrieveForMyAccountBalanceBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveForMyAccountBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveForMyAccountBalance").finish_non_exhaustive()
    }
}
impl RetrieveForMyAccountBalance {
    /// Construct a new `RetrieveForMyAccountBalance`.
    pub fn new() -> Self {
        Self { inner: RetrieveForMyAccountBalanceBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl Default for RetrieveForMyAccountBalance {
    fn default() -> Self {
        Self::new()
    }
}
impl RetrieveForMyAccountBalance {
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

impl StripeRequest for RetrieveForMyAccountBalance {
    type Output = stripe_core::Balance;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/balance").query(&self.inner)
    }
}
