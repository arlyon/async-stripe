#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListFinancialConnectionsAccount<'a> {
    /// If present, only return accounts that belong to the specified account holder.
    /// `account_holder[customer]` and `account_holder[account]` are mutually exclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder: Option<ListFinancialConnectionsAccountAccountHolder<'a>>,
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// If present, only return accounts that were collected as part of the given session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<&'a str>,
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}
impl<'a> ListFinancialConnectionsAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If present, only return accounts that belong to the specified account holder.
/// `account_holder[customer]` and `account_holder[account]` are mutually exclusive.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
        Self::default()
    }
}
impl<'a> ListFinancialConnectionsAccount<'a> {
    /// Returns a list of Financial Connections `Account` objects.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_misc::FinancialConnectionsAccount>> {
        client.get_query("/financial_connections/accounts", self)
    }
    pub fn paginate(
        self,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_misc::FinancialConnectionsAccount>> {
        stripe::ListPaginator::from_list_params("/financial_connections/accounts", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveFinancialConnectionsAccount<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveFinancialConnectionsAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveFinancialConnectionsAccount<'a> {
    /// Retrieves the details of an Financial Connections `Account`.
    pub fn send(
        &self,
        client: &stripe::Client,
        account: &stripe_misc::FinancialConnectionsAccountId,
    ) -> stripe::Response<stripe_misc::FinancialConnectionsAccount> {
        client.get_query(&format!("/financial_connections/accounts/{account}"), self)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ListOwnersFinancialConnectionsAccount<'a> {
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The ID of the ownership object to fetch owners from.
    pub ownership: &'a str,
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}
impl<'a> ListOwnersFinancialConnectionsAccount<'a> {
    pub fn new(ownership: &'a str) -> Self {
        Self { ending_before: None, expand: None, limit: None, ownership, starting_after: None }
    }
}
impl<'a> ListOwnersFinancialConnectionsAccount<'a> {
    /// Lists all owners for a given `Account`
    pub fn send(
        &self,
        client: &stripe::Client,
        account: &stripe_misc::FinancialConnectionsAccountId,
    ) -> stripe::Response<stripe_types::List<stripe_misc::FinancialConnectionsAccountOwner>> {
        client.get_query(&format!("/financial_connections/accounts/{account}/owners"), self)
    }
    pub fn paginate(
        self,
        account: &stripe_misc::FinancialConnectionsAccountId,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_misc::FinancialConnectionsAccountOwner>>
    {
        stripe::ListPaginator::from_list_params(
            &format!("/financial_connections/accounts/{account}/owners"),
            self,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DisconnectFinancialConnectionsAccount<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> DisconnectFinancialConnectionsAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> DisconnectFinancialConnectionsAccount<'a> {
    /// Disables your access to a Financial Connections `Account`.
    /// You will no longer be able to access data associated with the account (e.g.
    /// balances, transactions).
    pub fn send(
        &self,
        client: &stripe::Client,
        account: &stripe_misc::FinancialConnectionsAccountId,
    ) -> stripe::Response<stripe_misc::FinancialConnectionsAccount> {
        client.send_form(
            &format!("/financial_connections/accounts/{account}/disconnect"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct RefreshFinancialConnectionsAccount<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The list of account features that you would like to refresh.
    pub features: &'a [RefreshFinancialConnectionsAccountFeatures],
}
impl<'a> RefreshFinancialConnectionsAccount<'a> {
    pub fn new(features: &'a [RefreshFinancialConnectionsAccountFeatures]) -> Self {
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RefreshFinancialConnectionsAccountFeatures::*;
        match s {
            "balance" => Ok(Balance),
            "ownership" => Ok(Ownership),
            "transactions" => Ok(Transactions),
            _ => Err(()),
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
impl<'a> RefreshFinancialConnectionsAccount<'a> {
    /// Refreshes the data associated with a Financial Connections `Account`.
    pub fn send(
        &self,
        client: &stripe::Client,
        account: &stripe_misc::FinancialConnectionsAccountId,
    ) -> stripe::Response<stripe_misc::FinancialConnectionsAccount> {
        client.send_form(
            &format!("/financial_connections/accounts/{account}/refresh"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct SubscribeFinancialConnectionsAccount<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The list of account features to which you would like to subscribe.
    pub features: &'a [SubscribeFinancialConnectionsAccountFeatures],
}
impl<'a> SubscribeFinancialConnectionsAccount<'a> {
    pub fn new(features: &'a [SubscribeFinancialConnectionsAccountFeatures]) -> Self {
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscribeFinancialConnectionsAccountFeatures::*;
        match s {
            "transactions" => Ok(Transactions),
            _ => Err(()),
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
impl<'a> SubscribeFinancialConnectionsAccount<'a> {
    /// Subscribes to periodic refreshes of data associated with a Financial Connections `Account`.
    pub fn send(
        &self,
        client: &stripe::Client,
        account: &stripe_misc::FinancialConnectionsAccountId,
    ) -> stripe::Response<stripe_misc::FinancialConnectionsAccount> {
        client.send_form(
            &format!("/financial_connections/accounts/{account}/subscribe"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UnsubscribeFinancialConnectionsAccount<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The list of account features from which you would like to unsubscribe.
    pub features: &'a [UnsubscribeFinancialConnectionsAccountFeatures],
}
impl<'a> UnsubscribeFinancialConnectionsAccount<'a> {
    pub fn new(features: &'a [UnsubscribeFinancialConnectionsAccountFeatures]) -> Self {
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UnsubscribeFinancialConnectionsAccountFeatures::*;
        match s {
            "transactions" => Ok(Transactions),
            _ => Err(()),
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
impl<'a> UnsubscribeFinancialConnectionsAccount<'a> {
    /// Unsubscribes from periodic refreshes of data associated with a Financial Connections `Account`.
    pub fn send(
        &self,
        client: &stripe::Client,
        account: &stripe_misc::FinancialConnectionsAccountId,
    ) -> stripe::Response<stripe_misc::FinancialConnectionsAccount> {
        client.send_form(
            &format!("/financial_connections/accounts/{account}/unsubscribe"),
            self,
            http_types::Method::Post,
        )
    }
}
