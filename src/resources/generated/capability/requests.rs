use crate::{Client, Response};

impl crate::capability::Capability {
    /// Returns a list of capabilities associated with the account.
    ///
    /// The capabilities are returned sorted by creation date, with the most recent capability appearing first.
    pub fn list(
        client: &Client,
        account: &crate::account::AccountId,
        params: ListCapability,
    ) -> Response<crate::List<crate::capability::Capability>> {
        client.get_query(&format!("/accounts/{account}/capabilities", account = account), params)
    }
    /// Retrieves information about the specified Account Capability.
    pub fn retrieve(
        client: &Client,
        account: &crate::account::AccountId,
        capability: &crate::capability::CapabilityId,
        params: RetrieveCapability,
    ) -> Response<crate::capability::Capability> {
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
        client: &Client,
        account: &crate::account::AccountId,
        capability: &crate::capability::CapabilityId,
        params: UpdateCapability,
    ) -> Response<crate::capability::Capability> {
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
