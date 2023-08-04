#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListBankConnectionsResourceLinkedAccount<'a> {
    /// If present, only return accounts that belong to the specified account holder.
    ///
    /// `account_holder[customer]` and `account_holder[account]` are mutually exclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder: Option<ListBankConnectionsResourceLinkedAccountAccountHolder<'a>>,
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
impl<'a> ListBankConnectionsResourceLinkedAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If present, only return accounts that belong to the specified account holder.
///
/// `account_holder[customer]` and `account_holder[account]` are mutually exclusive.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListBankConnectionsResourceLinkedAccountAccountHolder<'a> {
    /// The ID of the Stripe account whose accounts will be retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<&'a str>,
    /// The ID of the Stripe customer whose accounts will be retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
}
impl<'a> ListBankConnectionsResourceLinkedAccountAccountHolder<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListBankConnectionsResourceLinkedAccount<'a> {
    /// Returns a list of Financial Connections `Account` objects.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_misc::BankConnectionsResourceLinkedAccount>>
    {
        client.get_query("/financial_connections/accounts", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveBankConnectionsResourceLinkedAccount<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveBankConnectionsResourceLinkedAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveBankConnectionsResourceLinkedAccount<'a> {
    /// Retrieves the details of an Financial Connections `Account`.
    pub fn send(
        &self,
        client: &stripe::Client,
        account: &stripe_types::account::AccountId,
    ) -> stripe::Response<stripe_misc::BankConnectionsResourceLinkedAccount> {
        client.get_query(
            &format!("/financial_connections/accounts/{account}", account = account),
            self,
        )
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListOwnersBankConnectionsResourceLinkedAccount<'a> {
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
impl<'a> ListOwnersBankConnectionsResourceLinkedAccount<'a> {
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
impl<'a> ListOwnersBankConnectionsResourceLinkedAccount<'a> {
    /// Lists all owners for a given `Account`.
    pub fn send(
        &self,
        client: &stripe::Client,
        account: &stripe_types::account::AccountId,
    ) -> stripe::Response<stripe_types::List<stripe_misc::BankConnectionsResourceOwner>> {
        client.get_query(
            &format!("/financial_connections/accounts/{account}/owners", account = account),
            self,
        )
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct RefreshBankConnectionsResourceLinkedAccount<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The list of account features that you would like to refresh.
    pub features: &'a [RefreshBankConnectionsResourceLinkedAccountFeatures],
}
impl<'a> RefreshBankConnectionsResourceLinkedAccount<'a> {
    pub fn new(features: &'a [RefreshBankConnectionsResourceLinkedAccountFeatures]) -> Self {
        Self { expand: Default::default(), features }
    }
}
/// The list of account features that you would like to refresh.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RefreshBankConnectionsResourceLinkedAccountFeatures {
    Balance,
    Ownership,
}

impl RefreshBankConnectionsResourceLinkedAccountFeatures {
    pub fn as_str(self) -> &'static str {
        use RefreshBankConnectionsResourceLinkedAccountFeatures::*;
        match self {
            Balance => "balance",
            Ownership => "ownership",
        }
    }
}

impl std::str::FromStr for RefreshBankConnectionsResourceLinkedAccountFeatures {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RefreshBankConnectionsResourceLinkedAccountFeatures::*;
        match s {
            "balance" => Ok(Balance),
            "ownership" => Ok(Ownership),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for RefreshBankConnectionsResourceLinkedAccountFeatures {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for RefreshBankConnectionsResourceLinkedAccountFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for RefreshBankConnectionsResourceLinkedAccountFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for RefreshBankConnectionsResourceLinkedAccountFeatures {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> RefreshBankConnectionsResourceLinkedAccount<'a> {
    /// Refreshes the data associated with a Financial Connections `Account`.
    pub fn send(
        &self,
        client: &stripe::Client,
        account: &stripe_types::account::AccountId,
    ) -> stripe::Response<stripe_misc::BankConnectionsResourceLinkedAccount> {
        client.send_form(
            &format!("/financial_connections/accounts/{account}/refresh", account = account),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DisconnectBankConnectionsResourceLinkedAccount<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> DisconnectBankConnectionsResourceLinkedAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> DisconnectBankConnectionsResourceLinkedAccount<'a> {
    /// Disables your access to a Financial Connections `Account`.
    ///
    /// You will no longer be able to access data associated with the account (e.g.
    /// balances, transactions).
    pub fn send(
        &self,
        client: &stripe::Client,
        account: &stripe_types::account::AccountId,
    ) -> stripe::Response<stripe_misc::BankConnectionsResourceLinkedAccount> {
        client.send_form(
            &format!("/financial_connections/accounts/{account}/disconnect", account = account),
            self,
            http_types::Method::Post,
        )
    }
}
