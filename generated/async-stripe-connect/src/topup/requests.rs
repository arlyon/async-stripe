use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct ListTopupBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<ListTopupStatus>,
}
impl ListTopupBuilder {
    fn new() -> Self {
        Self {
            amount: None,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            status: None,
        }
    }
}
/// Only return top-ups that have the given status.
/// One of `canceled`, `failed`, `pending` or `succeeded`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListTopupStatus {
    Canceled,
    Failed,
    Pending,
    Succeeded,
}
impl ListTopupStatus {
    pub fn as_str(self) -> &'static str {
        use ListTopupStatus::*;
        match self {
            Canceled => "canceled",
            Failed => "failed",
            Pending => "pending",
            Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for ListTopupStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListTopupStatus::*;
        match s {
            "canceled" => Ok(Canceled),
            "failed" => Ok(Failed),
            "pending" => Ok(Pending),
            "succeeded" => Ok(Succeeded),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ListTopupStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListTopupStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListTopupStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ListTopupStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ListTopupStatus"))
    }
}
/// Returns a list of top-ups.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct ListTopup {
    inner: ListTopupBuilder,
}
impl ListTopup {
    /// Construct a new `ListTopup`.
    pub fn new() -> Self {
        Self { inner: ListTopupBuilder::new() }
    }
    /// A positive integer representing how much to transfer.
    pub fn amount(mut self, amount: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.amount = Some(amount.into());
        self
    }
    /// A filter on the list, based on the object `created` field.
    /// The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    pub fn created(mut self, created: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.created = Some(created.into());
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
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
    /// Only return top-ups that have the given status.
    /// One of `canceled`, `failed`, `pending` or `succeeded`.
    pub fn status(mut self, status: impl Into<ListTopupStatus>) -> Self {
        self.inner.status = Some(status.into());
        self
    }
}
impl Default for ListTopup {
    fn default() -> Self {
        Self::new()
    }
}
impl ListTopup {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::Topup>> {
        stripe_client_core::ListPaginator::new_list("/topups", &self.inner)
    }
}

impl StripeRequest for ListTopup {
    type Output = stripe_types::List<stripe_shared::Topup>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/topups").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct RetrieveTopupBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveTopupBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of a top-up that has previously been created.
/// Supply the unique top-up ID that was returned from your previous request, and Stripe will return the corresponding top-up information.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct RetrieveTopup {
    inner: RetrieveTopupBuilder,
    topup: stripe_shared::TopupId,
}
impl RetrieveTopup {
    /// Construct a new `RetrieveTopup`.
    pub fn new(topup: impl Into<stripe_shared::TopupId>) -> Self {
        Self { topup: topup.into(), inner: RetrieveTopupBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveTopup {
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

impl StripeRequest for RetrieveTopup {
    type Output = stripe_shared::Topup;

    fn build(&self) -> RequestBuilder {
        let topup = &self.topup;
        RequestBuilder::new(StripeMethod::Get, format!("/topups/{topup}")).query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct CreateTopupBuilder {
    amount: i64,
    currency: stripe_types::Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statement_descriptor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_group: Option<String>,
}
impl CreateTopupBuilder {
    fn new(amount: impl Into<i64>, currency: impl Into<stripe_types::Currency>) -> Self {
        Self {
            amount: amount.into(),
            currency: currency.into(),
            description: None,
            expand: None,
            metadata: None,
            source: None,
            statement_descriptor: None,
            transfer_group: None,
        }
    }
}
/// Top up the balance of an account
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTopup {
    inner: CreateTopupBuilder,
}
impl CreateTopup {
    /// Construct a new `CreateTopup`.
    pub fn new(amount: impl Into<i64>, currency: impl Into<stripe_types::Currency>) -> Self {
        Self { inner: CreateTopupBuilder::new(amount.into(), currency.into()) }
    }
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.inner.description = Some(description.into());
        self
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
    /// The ID of a source to transfer funds from.
    /// For most users, this should be left unspecified which will use the bank account that was set up in the dashboard for the specified currency.
    /// In test mode, this can be a test bank token (see [Testing Top-ups](https://stripe.com/docs/connect/testing#testing-top-ups)).
    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.inner.source = Some(source.into());
        self
    }
    /// Extra information about a top-up for the source's bank statement. Limited to 15 ASCII characters.
    pub fn statement_descriptor(mut self, statement_descriptor: impl Into<String>) -> Self {
        self.inner.statement_descriptor = Some(statement_descriptor.into());
        self
    }
    /// A string that identifies this top-up as part of a group.
    pub fn transfer_group(mut self, transfer_group: impl Into<String>) -> Self {
        self.inner.transfer_group = Some(transfer_group.into());
        self
    }
}
impl CreateTopup {
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

impl StripeRequest for CreateTopup {
    type Output = stripe_shared::Topup;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/topups").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct UpdateTopupBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
}
impl UpdateTopupBuilder {
    fn new() -> Self {
        Self { description: None, expand: None, metadata: None }
    }
}
/// Updates the metadata of a top-up. Other top-up details are not editable by design.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct UpdateTopup {
    inner: UpdateTopupBuilder,
    topup: stripe_shared::TopupId,
}
impl UpdateTopup {
    /// Construct a new `UpdateTopup`.
    pub fn new(topup: impl Into<stripe_shared::TopupId>) -> Self {
        Self { topup: topup.into(), inner: UpdateTopupBuilder::new() }
    }
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.inner.description = Some(description.into());
        self
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
impl UpdateTopup {
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

impl StripeRequest for UpdateTopup {
    type Output = stripe_shared::Topup;

    fn build(&self) -> RequestBuilder {
        let topup = &self.topup;
        RequestBuilder::new(StripeMethod::Post, format!("/topups/{topup}")).form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct CancelTopupBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl CancelTopupBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Cancels a top-up. Only pending top-ups can be canceled.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CancelTopup {
    inner: CancelTopupBuilder,
    topup: stripe_shared::TopupId,
}
impl CancelTopup {
    /// Construct a new `CancelTopup`.
    pub fn new(topup: impl Into<stripe_shared::TopupId>) -> Self {
        Self { topup: topup.into(), inner: CancelTopupBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl CancelTopup {
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

impl StripeRequest for CancelTopup {
    type Output = stripe_shared::Topup;

    fn build(&self) -> RequestBuilder {
        let topup = &self.topup;
        RequestBuilder::new(StripeMethod::Post, format!("/topups/{topup}/cancel")).form(&self.inner)
    }
}
