use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct ListTransferBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_group: Option<String>,
}
impl ListTransferBuilder {
    fn new() -> Self {
        Self {
            created: None,
            destination: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            transfer_group: None,
        }
    }
}
/// Returns a list of existing transfers sent to connected accounts.
/// The transfers are returned in sorted order, with the most recently created transfers appearing first.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListTransfer {
    inner: ListTransferBuilder,
}
impl ListTransfer {
    /// Construct a new `ListTransfer`.
    pub fn new() -> Self {
        Self { inner: ListTransferBuilder::new() }
    }
    /// Only return transfers that were created during the given date interval.
    pub fn created(mut self, created: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.created = Some(created.into());
        self
    }
    /// Only return transfers for the destination specified by this account ID.
    pub fn destination(mut self, destination: impl Into<String>) -> Self {
        self.inner.destination = Some(destination.into());
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
    /// Only return transfers with the specified transfer group.
    pub fn transfer_group(mut self, transfer_group: impl Into<String>) -> Self {
        self.inner.transfer_group = Some(transfer_group.into());
        self
    }
}
impl Default for ListTransfer {
    fn default() -> Self {
        Self::new()
    }
}
impl ListTransfer {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::Transfer>> {
        stripe_client_core::ListPaginator::new_list("/transfers", &self.inner)
    }
}

impl StripeRequest for ListTransfer {
    type Output = stripe_types::List<stripe_shared::Transfer>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/transfers").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveTransferBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveTransferBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of an existing transfer.
/// Supply the unique transfer ID from either a transfer creation request or the transfer list, and Stripe will return the corresponding transfer information.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveTransfer {
    inner: RetrieveTransferBuilder,
    transfer: stripe_shared::TransferId,
}
impl RetrieveTransfer {
    /// Construct a new `RetrieveTransfer`.
    pub fn new(transfer: impl Into<stripe_shared::TransferId>) -> Self {
        Self { transfer: transfer.into(), inner: RetrieveTransferBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveTransfer {
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

impl StripeRequest for RetrieveTransfer {
    type Output = stripe_shared::Transfer;

    fn build(&self) -> RequestBuilder {
        let transfer = &self.transfer;
        RequestBuilder::new(StripeMethod::Get, format!("/transfers/{transfer}")).query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreateTransferBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    currency: stripe_types::Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    destination: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_transaction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_type: Option<CreateTransferSourceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_group: Option<String>,
}
impl CreateTransferBuilder {
    fn new(currency: impl Into<stripe_types::Currency>, destination: impl Into<String>) -> Self {
        Self {
            amount: None,
            currency: currency.into(),
            description: None,
            destination: destination.into(),
            expand: None,
            metadata: None,
            source_transaction: None,
            source_type: None,
            transfer_group: None,
        }
    }
}
/// The source balance to use for this transfer.
/// One of `bank_account`, `card`, or `fpx`.
/// For most users, this will default to `card`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateTransferSourceType {
    BankAccount,
    Card,
    Fpx,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateTransferSourceType {
    pub fn as_str(&self) -> &str {
        use CreateTransferSourceType::*;
        match self {
            BankAccount => "bank_account",
            Card => "card",
            Fpx => "fpx",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateTransferSourceType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTransferSourceType::*;
        match s {
            "bank_account" => Ok(BankAccount),
            "card" => Ok(Card),
            "fpx" => Ok(Fpx),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "CreateTransferSourceType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateTransferSourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTransferSourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTransferSourceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTransferSourceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// To send funds from your Stripe account to a connected account, you create a new transfer object.
/// Your [Stripe balance](https://stripe.com/docs/api#balance) must be able to cover the transfer amount, or you’ll receive an “Insufficient Funds” error.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTransfer {
    inner: CreateTransferBuilder,
}
impl CreateTransfer {
    /// Construct a new `CreateTransfer`.
    pub fn new(
        currency: impl Into<stripe_types::Currency>,
        destination: impl Into<String>,
    ) -> Self {
        Self { inner: CreateTransferBuilder::new(currency.into(), destination.into()) }
    }
    /// A positive integer in cents (or local equivalent) representing how much to transfer.
    pub fn amount(mut self, amount: impl Into<i64>) -> Self {
        self.inner.amount = Some(amount.into());
        self
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
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
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
    /// You can use this parameter to transfer funds from a charge before they are added to your available balance.
    /// A pending balance will transfer immediately but the funds will not become available until the original charge becomes available.
    /// [See the Connect documentation](https://docs.stripe.com/connect/separate-charges-and-transfers#transfer-availability) for details.
    pub fn source_transaction(mut self, source_transaction: impl Into<String>) -> Self {
        self.inner.source_transaction = Some(source_transaction.into());
        self
    }
    /// The source balance to use for this transfer.
    /// One of `bank_account`, `card`, or `fpx`.
    /// For most users, this will default to `card`.
    pub fn source_type(mut self, source_type: impl Into<CreateTransferSourceType>) -> Self {
        self.inner.source_type = Some(source_type.into());
        self
    }
    /// A string that identifies this transaction as part of a group.
    /// See the [Connect documentation](https://docs.stripe.com/connect/separate-charges-and-transfers#transfer-options) for details.
    pub fn transfer_group(mut self, transfer_group: impl Into<String>) -> Self {
        self.inner.transfer_group = Some(transfer_group.into());
        self
    }
}
impl CreateTransfer {
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

impl StripeRequest for CreateTransfer {
    type Output = stripe_shared::Transfer;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/transfers").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdateTransferBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
}
impl UpdateTransferBuilder {
    fn new() -> Self {
        Self { description: None, expand: None, metadata: None }
    }
}
/// Updates the specified transfer by setting the values of the parameters passed.
/// Any parameters not provided will be left unchanged.
///
/// This request accepts only metadata as an argument.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateTransfer {
    inner: UpdateTransferBuilder,
    transfer: stripe_shared::TransferId,
}
impl UpdateTransfer {
    /// Construct a new `UpdateTransfer`.
    pub fn new(transfer: impl Into<stripe_shared::TransferId>) -> Self {
        Self { transfer: transfer.into(), inner: UpdateTransferBuilder::new() }
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
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
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
impl UpdateTransfer {
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

impl StripeRequest for UpdateTransfer {
    type Output = stripe_shared::Transfer;

    fn build(&self) -> RequestBuilder {
        let transfer = &self.transfer;
        RequestBuilder::new(StripeMethod::Post, format!("/transfers/{transfer}")).form(&self.inner)
    }
}
