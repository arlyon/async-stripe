
/// Returns a list of capabilities associated with the account.
///
/// The capabilities are returned sorted by creation date, with the most recent capability appearing first.
pub fn list(client: &stripe::Client, account: &stripe_types::account::AccountId, params: ListAccountCapability) -> stripe::Response<stripe_types::List<stripe_types::AccountCapability>> {
    client.get_query(&format!("/accounts/{account}/capabilities", account = account), params)
}
/// Retrieves information about the specified Account Capability.
pub fn retrieve(client: &stripe::Client, account: &stripe_types::account::AccountId, capability: &stripe_types::account_capability::CapabilityId, params: RetrieveAccountCapability) -> stripe::Response<stripe_types::AccountCapability> {
    client.get_query(&format!("/accounts/{account}/capabilities/{capability}", account = account, capability = capability), params)
}
/// Updates an existing Account Capability.
pub fn update(client: &stripe::Client, account: &stripe_types::account::AccountId, capability: &stripe_types::account_capability::CapabilityId, params: UpdateAccountCapability) -> stripe::Response<stripe_types::AccountCapability> {
    client.send_form(&format!("/accounts/{account}/capabilities/{capability}", account = account, capability = capability), params, http_types::Method::Post)
}
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateAccountCapability<'a> {
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
impl<'a> UpdateAccountCapability<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
