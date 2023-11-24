#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListAccountCapability<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> ListAccountCapability<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListAccountCapability<'a> {
    /// Returns a list of capabilities associated with the account.
    ///
    /// The capabilities are returned sorted by creation date, with the most recent capability appearing first.
    pub fn send(
        &self,
        client: &stripe::Client,
        account: &stripe_types::account::AccountId,
    ) -> stripe::Response<stripe_types::List<stripe_types::AccountCapability>> {
        client.get_query(&format!("/accounts/{account}/capabilities"), self)
    }
    pub fn paginate(
        self,
        account: &stripe_types::account::AccountId,
    ) -> stripe::ListPaginator<stripe_types::AccountCapability> {
        stripe::ListPaginator::from_params(&format!("/accounts/{account}/capabilities"), self)
    }
}
impl<'a> stripe::PaginationParams for ListAccountCapability<'a> {}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveAccountCapability<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveAccountCapability<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveAccountCapability<'a> {
    /// Retrieves information about the specified Account Capability.
    pub fn send(
        &self,
        client: &stripe::Client,
        account: &stripe_types::account::AccountId,
        capability: &stripe_types::account_capability::CapabilityId,
    ) -> stripe::Response<stripe_types::AccountCapability> {
        client.get_query(&format!("/accounts/{account}/capabilities/{capability}"), self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapability<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// To request a new capability for an account, pass true.
    ///
    /// There can be a delay before the requested capability becomes active.
    /// If the capability has any activation requirements, the response includes them in the `requirements` arrays.  If a capability isn't permanent, you can remove it from the account by passing false.
    /// Most capabilities are permanent after they've been requested.
    /// Attempting to remove a permanent capability returns an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl<'a> UpdateAccountCapability<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> UpdateAccountCapability<'a> {
    /// Updates an existing Account Capability.
    ///
    /// Request or remove a capability by updating its `requested` parameter.
    pub fn send(
        &self,
        client: &stripe::Client,
        account: &stripe_types::account::AccountId,
        capability: &stripe_types::account_capability::CapabilityId,
    ) -> stripe::Response<stripe_types::AccountCapability> {
        client.send_form(
            &format!("/accounts/{account}/capabilities/{capability}"),
            self,
            http_types::Method::Post,
        )
    }
}
