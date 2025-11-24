use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct ListFinancialConnectionsAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_holder: Option<ListFinancialConnectionsAccountAccountHolder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListFinancialConnectionsAccountBuilder {
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListFinancialConnectionsAccountAccountHolder {
    /// The ID of the Stripe account whose accounts will be retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// The ID of the Stripe customer whose accounts will be retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
}
impl ListFinancialConnectionsAccountAccountHolder {
    pub fn new() -> Self {
        Self { account: None, customer: None }
    }
}
impl Default for ListFinancialConnectionsAccountAccountHolder {
    fn default() -> Self {
        Self::new()
    }
}
/// Returns a list of Financial Connections `Account` objects.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListFinancialConnectionsAccount {
    inner: ListFinancialConnectionsAccountBuilder,
}
impl ListFinancialConnectionsAccount {
    /// Construct a new `ListFinancialConnectionsAccount`.
    pub fn new() -> Self {
        Self { inner: ListFinancialConnectionsAccountBuilder::new() }
    }
    /// If present, only return accounts that belong to the specified account holder.
    /// `account_holder[customer]` and `account_holder[account]` are mutually exclusive.
    pub fn account_holder(
        mut self,
        account_holder: impl Into<ListFinancialConnectionsAccountAccountHolder>,
    ) -> Self {
        self.inner.account_holder = Some(account_holder.into());
        self
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: impl Into<String>) -> Self {
        self.inner.ending_before = Some(ending_before.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.inner.limit = Some(limit.into());
        self
    }
    /// If present, only return accounts that were collected as part of the given session.
    pub fn session(mut self, session: impl Into<String>) -> Self {
        self.inner.session = Some(session.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
}
impl Default for ListFinancialConnectionsAccount {
    fn default() -> Self {
        Self::new()
    }
}
impl ListFinancialConnectionsAccount {
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
        stripe_client_core::ListPaginator::new_list("/financial_connections/accounts", &self.inner)
    }
}

impl StripeRequest for ListFinancialConnectionsAccount {
    type Output = stripe_types::List<stripe_misc::FinancialConnectionsAccount>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/financial_connections/accounts").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveFinancialConnectionsAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveFinancialConnectionsAccountBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of an Financial Connections `Account`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveFinancialConnectionsAccount {
    inner: RetrieveFinancialConnectionsAccountBuilder,
    account: stripe_misc::FinancialConnectionsAccountId,
}
impl RetrieveFinancialConnectionsAccount {
    /// Construct a new `RetrieveFinancialConnectionsAccount`.
    pub fn new(account: impl Into<stripe_misc::FinancialConnectionsAccountId>) -> Self {
        Self { account: account.into(), inner: RetrieveFinancialConnectionsAccountBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveFinancialConnectionsAccount {
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

impl StripeRequest for RetrieveFinancialConnectionsAccount {
    type Output = stripe_misc::FinancialConnectionsAccount;

    fn build(&self) -> RequestBuilder {
        let account = &self.account;
        RequestBuilder::new(StripeMethod::Get, format!("/financial_connections/accounts/{account}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct ListOwnersFinancialConnectionsAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    ownership: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListOwnersFinancialConnectionsAccountBuilder {
    fn new(ownership: impl Into<String>) -> Self {
        Self {
            ending_before: None,
            expand: None,
            limit: None,
            ownership: ownership.into(),
            starting_after: None,
        }
    }
}
/// Lists all owners for a given `Account`
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListOwnersFinancialConnectionsAccount {
    inner: ListOwnersFinancialConnectionsAccountBuilder,
    account: stripe_misc::FinancialConnectionsAccountId,
}
impl ListOwnersFinancialConnectionsAccount {
    /// Construct a new `ListOwnersFinancialConnectionsAccount`.
    pub fn new(
        account: impl Into<stripe_misc::FinancialConnectionsAccountId>,
        ownership: impl Into<String>,
    ) -> Self {
        Self {
            account: account.into(),
            inner: ListOwnersFinancialConnectionsAccountBuilder::new(ownership.into()),
        }
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: impl Into<String>) -> Self {
        self.inner.ending_before = Some(ending_before.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.inner.limit = Some(limit.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
}
impl ListOwnersFinancialConnectionsAccount {
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
        let account = &self.account;

        stripe_client_core::ListPaginator::new_list(
            format!("/financial_connections/accounts/{account}/owners"),
            &self.inner,
        )
    }
}

impl StripeRequest for ListOwnersFinancialConnectionsAccount {
    type Output = stripe_types::List<stripe_misc::FinancialConnectionsAccountOwner>;

    fn build(&self) -> RequestBuilder {
        let account = &self.account;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/financial_connections/accounts/{account}/owners"),
        )
        .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct DisconnectFinancialConnectionsAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl DisconnectFinancialConnectionsAccountBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Disables your access to a Financial Connections `Account`.
/// You will no longer be able to access data associated with the account (e.g.
/// balances, transactions).
#[derive(Clone, Debug, serde::Serialize)]
pub struct DisconnectFinancialConnectionsAccount {
    inner: DisconnectFinancialConnectionsAccountBuilder,
    account: stripe_misc::FinancialConnectionsAccountId,
}
impl DisconnectFinancialConnectionsAccount {
    /// Construct a new `DisconnectFinancialConnectionsAccount`.
    pub fn new(account: impl Into<stripe_misc::FinancialConnectionsAccountId>) -> Self {
        Self { account: account.into(), inner: DisconnectFinancialConnectionsAccountBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl DisconnectFinancialConnectionsAccount {
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

impl StripeRequest for DisconnectFinancialConnectionsAccount {
    type Output = stripe_misc::FinancialConnectionsAccount;

    fn build(&self) -> RequestBuilder {
        let account = &self.account;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/financial_connections/accounts/{account}/disconnect"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RefreshFinancialConnectionsAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    features: Vec<RefreshFinancialConnectionsAccountFeatures>,
}
impl RefreshFinancialConnectionsAccountBuilder {
    fn new(features: impl Into<Vec<RefreshFinancialConnectionsAccountFeatures>>) -> Self {
        Self { expand: None, features: features.into() }
    }
}
/// The list of account features that you would like to refresh.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum RefreshFinancialConnectionsAccountFeatures {
    Balance,
    Ownership,
    Transactions,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl RefreshFinancialConnectionsAccountFeatures {
    pub fn as_str(&self) -> &str {
        use RefreshFinancialConnectionsAccountFeatures::*;
        match self {
            Balance => "balance",
            Ownership => "ownership",
            Transactions => "transactions",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for RefreshFinancialConnectionsAccountFeatures {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RefreshFinancialConnectionsAccountFeatures::*;
        match s {
            "balance" => Ok(Balance),
            "ownership" => Ok(Ownership),
            "transactions" => Ok(Transactions),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "RefreshFinancialConnectionsAccountFeatures"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Refreshes the data associated with a Financial Connections `Account`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RefreshFinancialConnectionsAccount {
    inner: RefreshFinancialConnectionsAccountBuilder,
    account: stripe_misc::FinancialConnectionsAccountId,
}
impl RefreshFinancialConnectionsAccount {
    /// Construct a new `RefreshFinancialConnectionsAccount`.
    pub fn new(
        account: impl Into<stripe_misc::FinancialConnectionsAccountId>,
        features: impl Into<Vec<RefreshFinancialConnectionsAccountFeatures>>,
    ) -> Self {
        Self {
            account: account.into(),
            inner: RefreshFinancialConnectionsAccountBuilder::new(features.into()),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RefreshFinancialConnectionsAccount {
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

impl StripeRequest for RefreshFinancialConnectionsAccount {
    type Output = stripe_misc::FinancialConnectionsAccount;

    fn build(&self) -> RequestBuilder {
        let account = &self.account;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/financial_connections/accounts/{account}/refresh"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct SubscribeFinancialConnectionsAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    features: Vec<SubscribeFinancialConnectionsAccountFeatures>,
}
impl SubscribeFinancialConnectionsAccountBuilder {
    fn new(features: impl Into<Vec<SubscribeFinancialConnectionsAccountFeatures>>) -> Self {
        Self { expand: None, features: features.into() }
    }
}
/// The list of account features to which you would like to subscribe.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SubscribeFinancialConnectionsAccountFeatures {
    Transactions,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SubscribeFinancialConnectionsAccountFeatures {
    pub fn as_str(&self) -> &str {
        use SubscribeFinancialConnectionsAccountFeatures::*;
        match self {
            Transactions => "transactions",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for SubscribeFinancialConnectionsAccountFeatures {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscribeFinancialConnectionsAccountFeatures::*;
        match s {
            "transactions" => Ok(Transactions),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "SubscribeFinancialConnectionsAccountFeatures"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Subscribes to periodic refreshes of data associated with a Financial Connections `Account`.
/// When the account status is active, data is typically refreshed once a day.
#[derive(Clone, Debug, serde::Serialize)]
pub struct SubscribeFinancialConnectionsAccount {
    inner: SubscribeFinancialConnectionsAccountBuilder,
    account: stripe_misc::FinancialConnectionsAccountId,
}
impl SubscribeFinancialConnectionsAccount {
    /// Construct a new `SubscribeFinancialConnectionsAccount`.
    pub fn new(
        account: impl Into<stripe_misc::FinancialConnectionsAccountId>,
        features: impl Into<Vec<SubscribeFinancialConnectionsAccountFeatures>>,
    ) -> Self {
        Self {
            account: account.into(),
            inner: SubscribeFinancialConnectionsAccountBuilder::new(features.into()),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl SubscribeFinancialConnectionsAccount {
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

impl StripeRequest for SubscribeFinancialConnectionsAccount {
    type Output = stripe_misc::FinancialConnectionsAccount;

    fn build(&self) -> RequestBuilder {
        let account = &self.account;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/financial_connections/accounts/{account}/subscribe"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UnsubscribeFinancialConnectionsAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    features: Vec<UnsubscribeFinancialConnectionsAccountFeatures>,
}
impl UnsubscribeFinancialConnectionsAccountBuilder {
    fn new(features: impl Into<Vec<UnsubscribeFinancialConnectionsAccountFeatures>>) -> Self {
        Self { expand: None, features: features.into() }
    }
}
/// The list of account features from which you would like to unsubscribe.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UnsubscribeFinancialConnectionsAccountFeatures {
    Transactions,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UnsubscribeFinancialConnectionsAccountFeatures {
    pub fn as_str(&self) -> &str {
        use UnsubscribeFinancialConnectionsAccountFeatures::*;
        match self {
            Transactions => "transactions",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UnsubscribeFinancialConnectionsAccountFeatures {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UnsubscribeFinancialConnectionsAccountFeatures::*;
        match s {
            "transactions" => Ok(Transactions),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UnsubscribeFinancialConnectionsAccountFeatures"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Unsubscribes from periodic refreshes of data associated with a Financial Connections `Account`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UnsubscribeFinancialConnectionsAccount {
    inner: UnsubscribeFinancialConnectionsAccountBuilder,
    account: stripe_misc::FinancialConnectionsAccountId,
}
impl UnsubscribeFinancialConnectionsAccount {
    /// Construct a new `UnsubscribeFinancialConnectionsAccount`.
    pub fn new(
        account: impl Into<stripe_misc::FinancialConnectionsAccountId>,
        features: impl Into<Vec<UnsubscribeFinancialConnectionsAccountFeatures>>,
    ) -> Self {
        Self {
            account: account.into(),
            inner: UnsubscribeFinancialConnectionsAccountBuilder::new(features.into()),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl UnsubscribeFinancialConnectionsAccount {
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

impl StripeRequest for UnsubscribeFinancialConnectionsAccount {
    type Output = stripe_misc::FinancialConnectionsAccount;

    fn build(&self) -> RequestBuilder {
        let account = &self.account;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/financial_connections/accounts/{account}/unsubscribe"),
        )
        .form(&self.inner)
    }
}
