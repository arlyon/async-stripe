use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct ListTreasuryDebitReversalBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    financial_account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    received_debit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resolution: Option<ListTreasuryDebitReversalResolution>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<ListTreasuryDebitReversalStatus>,
}
impl ListTreasuryDebitReversalBuilder {
    fn new(financial_account: impl Into<String>) -> Self {
        Self {
            ending_before: None,
            expand: None,
            financial_account: financial_account.into(),
            limit: None,
            received_debit: None,
            resolution: None,
            starting_after: None,
            status: None,
        }
    }
}
/// Only return DebitReversals for a given resolution.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListTreasuryDebitReversalResolution {
    Lost,
    Won,
}
impl ListTreasuryDebitReversalResolution {
    pub fn as_str(self) -> &'static str {
        use ListTreasuryDebitReversalResolution::*;
        match self {
            Lost => "lost",
            Won => "won",
        }
    }
}

impl std::str::FromStr for ListTreasuryDebitReversalResolution {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListTreasuryDebitReversalResolution::*;
        match s {
            "lost" => Ok(Lost),
            "won" => Ok(Won),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ListTreasuryDebitReversalResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListTreasuryDebitReversalResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListTreasuryDebitReversalResolution {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ListTreasuryDebitReversalResolution {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for ListTreasuryDebitReversalResolution")
        })
    }
}
/// Only return DebitReversals for a given status.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListTreasuryDebitReversalStatus {
    Canceled,
    Completed,
    Processing,
}
impl ListTreasuryDebitReversalStatus {
    pub fn as_str(self) -> &'static str {
        use ListTreasuryDebitReversalStatus::*;
        match self {
            Canceled => "canceled",
            Completed => "completed",
            Processing => "processing",
        }
    }
}

impl std::str::FromStr for ListTreasuryDebitReversalStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListTreasuryDebitReversalStatus::*;
        match s {
            "canceled" => Ok(Canceled),
            "completed" => Ok(Completed),
            "processing" => Ok(Processing),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ListTreasuryDebitReversalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListTreasuryDebitReversalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListTreasuryDebitReversalStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ListTreasuryDebitReversalStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for ListTreasuryDebitReversalStatus")
        })
    }
}
/// Returns a list of DebitReversals.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListTreasuryDebitReversal {
    inner: ListTreasuryDebitReversalBuilder,
}
impl ListTreasuryDebitReversal {
    /// Construct a new `ListTreasuryDebitReversal`.
    pub fn new(financial_account: impl Into<String>) -> Self {
        Self { inner: ListTreasuryDebitReversalBuilder::new(financial_account.into()) }
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
    /// Only return DebitReversals for the ReceivedDebit ID.
    pub fn received_debit(mut self, received_debit: impl Into<String>) -> Self {
        self.inner.received_debit = Some(received_debit.into());
        self
    }
    /// Only return DebitReversals for a given resolution.
    pub fn resolution(
        mut self,
        resolution: impl Into<ListTreasuryDebitReversalResolution>,
    ) -> Self {
        self.inner.resolution = Some(resolution.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
    /// Only return DebitReversals for a given status.
    pub fn status(mut self, status: impl Into<ListTreasuryDebitReversalStatus>) -> Self {
        self.inner.status = Some(status.into());
        self
    }
}
impl ListTreasuryDebitReversal {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_treasury::TreasuryDebitReversal>>
    {
        stripe_client_core::ListPaginator::new_list("/treasury/debit_reversals", &self.inner)
    }
}

impl StripeRequest for ListTreasuryDebitReversal {
    type Output = stripe_types::List<stripe_treasury::TreasuryDebitReversal>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/treasury/debit_reversals").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveTreasuryDebitReversalBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveTreasuryDebitReversalBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a DebitReversal object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveTreasuryDebitReversal {
    inner: RetrieveTreasuryDebitReversalBuilder,
    debit_reversal: stripe_treasury::TreasuryDebitReversalId,
}
impl RetrieveTreasuryDebitReversal {
    /// Construct a new `RetrieveTreasuryDebitReversal`.
    pub fn new(debit_reversal: impl Into<stripe_treasury::TreasuryDebitReversalId>) -> Self {
        Self {
            debit_reversal: debit_reversal.into(),
            inner: RetrieveTreasuryDebitReversalBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveTreasuryDebitReversal {
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

impl StripeRequest for RetrieveTreasuryDebitReversal {
    type Output = stripe_treasury::TreasuryDebitReversal;

    fn build(&self) -> RequestBuilder {
        let debit_reversal = &self.debit_reversal;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/treasury/debit_reversals/{debit_reversal}"),
        )
        .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreateTreasuryDebitReversalBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    received_debit: String,
}
impl CreateTreasuryDebitReversalBuilder {
    fn new(received_debit: impl Into<String>) -> Self {
        Self { expand: None, metadata: None, received_debit: received_debit.into() }
    }
}
/// Reverses a ReceivedDebit and creates a DebitReversal object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTreasuryDebitReversal {
    inner: CreateTreasuryDebitReversalBuilder,
}
impl CreateTreasuryDebitReversal {
    /// Construct a new `CreateTreasuryDebitReversal`.
    pub fn new(received_debit: impl Into<String>) -> Self {
        Self { inner: CreateTreasuryDebitReversalBuilder::new(received_debit.into()) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
}
impl CreateTreasuryDebitReversal {
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

impl StripeRequest for CreateTreasuryDebitReversal {
    type Output = stripe_treasury::TreasuryDebitReversal;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/treasury/debit_reversals").form(&self.inner)
    }
}
