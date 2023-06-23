use stripe::{Client, Response};

impl stripe_core::balance::Balance {
    /// Retrieves the current account balance, based on the authentication that was used to make the request.
    ///  For a sample request, see [Accounting for negative balances](https://stripe.com/docs/connect/account-balances#accounting-for-negative-balances).
    pub fn retrieve_for_my_account(
        client: &Client,
        params: RetrieveForMyAccountBalance,
    ) -> Response<stripe_core::balance::Balance> {
        client.get_query("/balance", params)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveForMyAccountBalance<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveForMyAccountBalance<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
