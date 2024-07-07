use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListFinancialConnectionsAccountBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_holder: Option<ListFinancialConnectionsAccountAccountHolder<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListFinancialConnectionsAccountBuilder<'a> {
    fn new() -> Self {
        Self {
            account_holder: None,
            ending_before: None,
            expand: None,
            limit: None,
            session: None,
            starting_after: None,
        }
    }
}
/// If present, only return accounts that belong to the specified account holder.
/// `account_holder[customer]` and `account_holder[account]` are mutually exclusive.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ListFinancialConnectionsAccountAccountHolder<'a> {
    /// The ID of the Stripe account whose accounts will be retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<&'a str>,
    /// The ID of the Stripe customer whose accounts will be retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
}
impl<'a> ListFinancialConnectionsAccountAccountHolder<'a> {
    pub fn new() -> Self {
        Self { account: None, customer: None }
    }
}
impl<'a> Default for ListFinancialConnectionsAccountAccountHolder<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Returns a list of Financial Connections `Account` objects.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListFinancialConnectionsAccount<'a> {
    inner: ListFinancialConnectionsAccountBuilder<'a>,
}
impl<'a> ListFinancialConnectionsAccount<'a> {
    /// Construct a new `ListFinancialConnectionsAccount`.
    pub fn new() -> Self {
        Self { inner: ListFinancialConnectionsAccountBuilder::new() }
    }
    /// If present, only return accounts that belong to the specified account holder.
    /// `account_holder[customer]` and `account_holder[account]` are mutually exclusive.
    pub fn account_holder(
        mut self,
        account_holder: ListFinancialConnectionsAccountAccountHolder<'a>,
    ) -> Self {
        self.inner.account_holder = Some(account_holder);
        self
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: &'a str) -> Self {
        self.inner.ending_before = Some(ending_before);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// If present, only return accounts that were collected as part of the given session.
    pub fn session(mut self, session: &'a str) -> Self {
        self.inner.session = Some(session);
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
}
impl<'a> Default for ListFinancialConnectionsAccount<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListFinancialConnectionsAccount<'_> {
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

    pub fn paginate(
        &self,
    ) -> stripe_client_core::ListPaginator<
        stripe_types::List<stripe_misc::FinancialConnectionsAccount>,
    > {
        stripe_client_core::ListPaginator::new_list("/financial_connections/accounts", self.inner)
    }
}

impl StripeRequest for ListFinancialConnectionsAccount<'_> {
    type Output = stripe_types::List<stripe_misc::FinancialConnectionsAccount>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/financial_connections/accounts").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveFinancialConnectionsAccountBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveFinancialConnectionsAccountBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of an Financial Connections `Account`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveFinancialConnectionsAccount<'a> {
    inner: RetrieveFinancialConnectionsAccountBuilder<'a>,
    account: &'a stripe_misc::FinancialConnectionsAccountId,
}
impl<'a> RetrieveFinancialConnectionsAccount<'a> {
    /// Construct a new `RetrieveFinancialConnectionsAccount`.
    pub fn new(account: &'a stripe_misc::FinancialConnectionsAccountId) -> Self {
        Self { account, inner: RetrieveFinancialConnectionsAccountBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveFinancialConnectionsAccount<'_> {
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

impl StripeRequest for RetrieveFinancialConnectionsAccount<'_> {
    type Output = stripe_misc::FinancialConnectionsAccount;

    fn build(&self) -> RequestBuilder {
        let account = self.account;
        RequestBuilder::new(StripeMethod::Get, format!("/financial_connections/accounts/{account}"))
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListOwnersFinancialConnectionsAccountBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    ownership: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListOwnersFinancialConnectionsAccountBuilder<'a> {
    fn new(ownership: &'a str) -> Self {
        Self { ending_before: None, expand: None, limit: None, ownership, starting_after: None }
    }
}
/// Lists all owners for a given `Account`
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListOwnersFinancialConnectionsAccount<'a> {
    inner: ListOwnersFinancialConnectionsAccountBuilder<'a>,
    account: &'a stripe_misc::FinancialConnectionsAccountId,
}
impl<'a> ListOwnersFinancialConnectionsAccount<'a> {
    /// Construct a new `ListOwnersFinancialConnectionsAccount`.
    pub fn new(
        account: &'a stripe_misc::FinancialConnectionsAccountId,
        ownership: &'a str,
    ) -> Self {
        Self { account, inner: ListOwnersFinancialConnectionsAccountBuilder::new(ownership) }
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: &'a str) -> Self {
        self.inner.ending_before = Some(ending_before);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
}
impl ListOwnersFinancialConnectionsAccount<'_> {
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

    pub fn paginate(
        &self,
    ) -> stripe_client_core::ListPaginator<
        stripe_types::List<stripe_misc::FinancialConnectionsAccountOwner>,
    > {
        let account = self.account;

        stripe_client_core::ListPaginator::new_list(
            format!("/financial_connections/accounts/{account}/owners"),
            self.inner,
        )
    }
}

impl StripeRequest for ListOwnersFinancialConnectionsAccount<'_> {
    type Output = stripe_types::List<stripe_misc::FinancialConnectionsAccountOwner>;

    fn build(&self) -> RequestBuilder {
        let account = self.account;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/financial_connections/accounts/{account}/owners"),
        )
        .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct DisconnectFinancialConnectionsAccountBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> DisconnectFinancialConnectionsAccountBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Disables your access to a Financial Connections `Account`.
/// You will no longer be able to access data associated with the account (e.g.
/// balances, transactions).
#[derive(Clone, Debug, serde::Serialize)]
pub struct DisconnectFinancialConnectionsAccount<'a> {
    inner: DisconnectFinancialConnectionsAccountBuilder<'a>,
    account: &'a stripe_misc::FinancialConnectionsAccountId,
}
impl<'a> DisconnectFinancialConnectionsAccount<'a> {
    /// Construct a new `DisconnectFinancialConnectionsAccount`.
    pub fn new(account: &'a stripe_misc::FinancialConnectionsAccountId) -> Self {
        Self { account, inner: DisconnectFinancialConnectionsAccountBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl DisconnectFinancialConnectionsAccount<'_> {
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

impl StripeRequest for DisconnectFinancialConnectionsAccount<'_> {
    type Output = stripe_misc::FinancialConnectionsAccount;

    fn build(&self) -> RequestBuilder {
        let account = self.account;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/financial_connections/accounts/{account}/disconnect"),
        )
        .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RefreshFinancialConnectionsAccountBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    features: &'a [RefreshFinancialConnectionsAccountFeatures],
}
impl<'a> RefreshFinancialConnectionsAccountBuilder<'a> {
    fn new(features: &'a [RefreshFinancialConnectionsAccountFeatures]) -> Self {
        Self { expand: None, features }
    }
}
/// The list of account features that you would like to refresh.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RefreshFinancialConnectionsAccountFeatures {
    Balance,
    Ownership,
    Transactions,
}
impl RefreshFinancialConnectionsAccountFeatures {
    pub fn as_str(self) -> &'static str {
        use RefreshFinancialConnectionsAccountFeatures::*;
        match self {
            Balance => "balance",
            Ownership => "ownership",
            Transactions => "transactions",
        }
    }
}

impl std::str::FromStr for RefreshFinancialConnectionsAccountFeatures {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RefreshFinancialConnectionsAccountFeatures::*;
        match s {
            "balance" => Ok(Balance),
            "ownership" => Ok(Ownership),
            "transactions" => Ok(Transactions),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for RefreshFinancialConnectionsAccountFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for RefreshFinancialConnectionsAccountFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for RefreshFinancialConnectionsAccountFeatures {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for RefreshFinancialConnectionsAccountFeatures {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for RefreshFinancialConnectionsAccountFeatures")
        })
    }
}
/// Refreshes the data associated with a Financial Connections `Account`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RefreshFinancialConnectionsAccount<'a> {
    inner: RefreshFinancialConnectionsAccountBuilder<'a>,
    account: &'a stripe_misc::FinancialConnectionsAccountId,
}
impl<'a> RefreshFinancialConnectionsAccount<'a> {
    /// Construct a new `RefreshFinancialConnectionsAccount`.
    pub fn new(
        account: &'a stripe_misc::FinancialConnectionsAccountId,
        features: &'a [RefreshFinancialConnectionsAccountFeatures],
    ) -> Self {
        Self { account, inner: RefreshFinancialConnectionsAccountBuilder::new(features) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RefreshFinancialConnectionsAccount<'_> {
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

impl StripeRequest for RefreshFinancialConnectionsAccount<'_> {
    type Output = stripe_misc::FinancialConnectionsAccount;

    fn build(&self) -> RequestBuilder {
        let account = self.account;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/financial_connections/accounts/{account}/refresh"),
        )
        .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct SubscribeFinancialConnectionsAccountBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    features: &'a [SubscribeFinancialConnectionsAccountFeatures],
}
impl<'a> SubscribeFinancialConnectionsAccountBuilder<'a> {
    fn new(features: &'a [SubscribeFinancialConnectionsAccountFeatures]) -> Self {
        Self { expand: None, features }
    }
}
/// The list of account features to which you would like to subscribe.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SubscribeFinancialConnectionsAccountFeatures {
    Transactions,
}
impl SubscribeFinancialConnectionsAccountFeatures {
    pub fn as_str(self) -> &'static str {
        use SubscribeFinancialConnectionsAccountFeatures::*;
        match self {
            Transactions => "transactions",
        }
    }
}

impl std::str::FromStr for SubscribeFinancialConnectionsAccountFeatures {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscribeFinancialConnectionsAccountFeatures::*;
        match s {
            "transactions" => Ok(Transactions),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for SubscribeFinancialConnectionsAccountFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SubscribeFinancialConnectionsAccountFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SubscribeFinancialConnectionsAccountFeatures {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SubscribeFinancialConnectionsAccountFeatures {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for SubscribeFinancialConnectionsAccountFeatures",
            )
        })
    }
}
/// Subscribes to periodic refreshes of data associated with a Financial Connections `Account`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct SubscribeFinancialConnectionsAccount<'a> {
    inner: SubscribeFinancialConnectionsAccountBuilder<'a>,
    account: &'a stripe_misc::FinancialConnectionsAccountId,
}
impl<'a> SubscribeFinancialConnectionsAccount<'a> {
    /// Construct a new `SubscribeFinancialConnectionsAccount`.
    pub fn new(
        account: &'a stripe_misc::FinancialConnectionsAccountId,
        features: &'a [SubscribeFinancialConnectionsAccountFeatures],
    ) -> Self {
        Self { account, inner: SubscribeFinancialConnectionsAccountBuilder::new(features) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl SubscribeFinancialConnectionsAccount<'_> {
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

impl StripeRequest for SubscribeFinancialConnectionsAccount<'_> {
    type Output = stripe_misc::FinancialConnectionsAccount;

    fn build(&self) -> RequestBuilder {
        let account = self.account;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/financial_connections/accounts/{account}/subscribe"),
        )
        .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UnsubscribeFinancialConnectionsAccountBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    features: &'a [UnsubscribeFinancialConnectionsAccountFeatures],
}
impl<'a> UnsubscribeFinancialConnectionsAccountBuilder<'a> {
    fn new(features: &'a [UnsubscribeFinancialConnectionsAccountFeatures]) -> Self {
        Self { expand: None, features }
    }
}
/// The list of account features from which you would like to unsubscribe.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UnsubscribeFinancialConnectionsAccountFeatures {
    Transactions,
}
impl UnsubscribeFinancialConnectionsAccountFeatures {
    pub fn as_str(self) -> &'static str {
        use UnsubscribeFinancialConnectionsAccountFeatures::*;
        match self {
            Transactions => "transactions",
        }
    }
}

impl std::str::FromStr for UnsubscribeFinancialConnectionsAccountFeatures {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UnsubscribeFinancialConnectionsAccountFeatures::*;
        match s {
            "transactions" => Ok(Transactions),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UnsubscribeFinancialConnectionsAccountFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UnsubscribeFinancialConnectionsAccountFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UnsubscribeFinancialConnectionsAccountFeatures {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UnsubscribeFinancialConnectionsAccountFeatures {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UnsubscribeFinancialConnectionsAccountFeatures",
            )
        })
    }
}
/// Unsubscribes from periodic refreshes of data associated with a Financial Connections `Account`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UnsubscribeFinancialConnectionsAccount<'a> {
    inner: UnsubscribeFinancialConnectionsAccountBuilder<'a>,
    account: &'a stripe_misc::FinancialConnectionsAccountId,
}
impl<'a> UnsubscribeFinancialConnectionsAccount<'a> {
    /// Construct a new `UnsubscribeFinancialConnectionsAccount`.
    pub fn new(
        account: &'a stripe_misc::FinancialConnectionsAccountId,
        features: &'a [UnsubscribeFinancialConnectionsAccountFeatures],
    ) -> Self {
        Self { account, inner: UnsubscribeFinancialConnectionsAccountBuilder::new(features) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl UnsubscribeFinancialConnectionsAccount<'_> {
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

impl StripeRequest for UnsubscribeFinancialConnectionsAccount<'_> {
    type Output = stripe_misc::FinancialConnectionsAccount;

    fn build(&self) -> RequestBuilder {
        let account = self.account;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/financial_connections/accounts/{account}/unsubscribe"),
        )
        .form(&self.inner)
    }
}
