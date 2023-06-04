use crate::{Client, Response};

impl crate::financial_connections::account::Account {
    /// Returns a list of Financial Connections `Account` objects.
    pub fn list(
        client: &Client,
        params: ListAccount,
    ) -> Response<crate::List<crate::financial_connections::account::Account>> {
        client.get_query("/financial_connections/accounts", params)
    }
    /// Retrieves the details of an Financial Connections `Account`.
    pub fn retrieve(
        client: &Client,
        account: &crate::account::AccountId,
        params: RetrieveAccount,
    ) -> Response<crate::financial_connections::account::Account> {
        client.get_query(
            &format!("/financial_connections/accounts/{account}", account = account),
            params,
        )
    }
    /// Lists all owners for a given `Account`.
    pub fn list_owners(
        client: &Client,
        account: &crate::account::AccountId,
        params: ListOwnersAccount,
    ) -> Response<crate::List<crate::financial_connections::account_owner::AccountOwner>> {
        client.get_query(
            &format!("/financial_connections/accounts/{account}/owners", account = account),
            params,
        )
    }
    /// Refreshes the data associated with a Financial Connections `Account`.
    pub fn refresh(
        client: &Client,
        account: &crate::account::AccountId,
        params: RefreshAccount,
    ) -> Response<crate::financial_connections::account::Account> {
        client.send_form(
            &format!("/financial_connections/accounts/{account}/refresh", account = account),
            params,
            http_types::Method::Post,
        )
    }
    /// Disables your access to a Financial Connections `Account`.
    ///
    /// You will no longer be able to access data associated with the account (e.g.
    /// balances, transactions).
    pub fn disconnect(
        client: &Client,
        account: &crate::account::AccountId,
        params: DisconnectAccount,
    ) -> Response<crate::financial_connections::account::Account> {
        client.send_form(
            &format!("/financial_connections/accounts/{account}/disconnect", account = account),
            params,
            http_types::Method::Post,
        )
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListAccount<'a> {
    /// If present, only return accounts that belong to the specified account holder.
    ///
    /// `account_holder[customer]` and `account_holder[account]` are mutually exclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder: Option<ListAccountAccountHolder<'a>>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// If present, only return accounts that were collected as part of the given session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<&'a str>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}
impl<'a> ListAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If present, only return accounts that belong to the specified account holder.
///
/// `account_holder[customer]` and `account_holder[account]` are mutually exclusive.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListAccountAccountHolder<'a> {
    /// The ID of the Stripe account whose accounts will be retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<&'a str>,
    /// The ID of the Stripe customer whose accounts will be retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
}
impl<'a> ListAccountAccountHolder<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveAccount<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListOwnersAccount<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The ID of the ownership object to fetch owners from.
    pub ownership: &'a str,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}
impl<'a> ListOwnersAccount<'a> {
    pub fn new(ownership: &'a str) -> Self {
        Self {
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            ownership,
            starting_after: Default::default(),
        }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct RefreshAccount<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The list of account features that you would like to refresh.
    pub features: &'a [RefreshAccountFeatures],
}
impl<'a> RefreshAccount<'a> {
    pub fn new(features: &'a [RefreshAccountFeatures]) -> Self {
        Self { expand: Default::default(), features }
    }
}
/// The list of account features that you would like to refresh.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RefreshAccountFeatures {
    Balance,
    Ownership,
}

impl RefreshAccountFeatures {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Balance => "balance",
            Self::Ownership => "ownership",
        }
    }
}

impl AsRef<str> for RefreshAccountFeatures {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for RefreshAccountFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DisconnectAccount<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> DisconnectAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
