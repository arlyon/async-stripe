impl stripe_core::capability::Capability {
    /// Returns a list of capabilities associated with the account.
    ///
    /// The capabilities are returned sorted by creation date, with the most recent capability appearing first.
    pub fn list(
        client: &stripe::Client,
        account: &stripe_types::AccountId,
        params: ListCapability,
    ) -> stripe::Response<stripe_types::List<stripe_core::capability::Capability>> {
        client.get_query(&format!("/accounts/{account}/capabilities", account = account), params)
    }
    /// Retrieves information about the specified Account Capability.
    pub fn retrieve(
        client: &stripe::Client,
        account: &stripe_types::AccountId,
        capability: &stripe_core::capability::CapabilityId,
        params: RetrieveCapability,
    ) -> stripe::Response<stripe_core::capability::Capability> {
        client.get_query(
            &format!(
                "/accounts/{account}/capabilities/{capability}",
                account = account,
                capability = capability
            ),
            params,
        )
    }
    /// Updates an existing Account Capability.
    pub fn update(
        client: &stripe::Client,
        account: &stripe_types::AccountId,
        capability: &stripe_core::capability::CapabilityId,
        params: UpdateCapability,
    ) -> stripe::Response<stripe_core::capability::Capability> {
        client.send_form(
            &format!(
                "/accounts/{account}/capabilities/{capability}",
                account = account,
                capability = capability
            ),
            params,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListCapability<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> ListCapability<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveCapability<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveCapability<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateCapability<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl<'a> UpdateCapability<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
