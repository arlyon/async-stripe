use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct ListPayoutBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    arrival_date: Option<stripe_types::RangeQueryTs>,
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
    status: Option<String>,
}
impl ListPayoutBuilder {
    fn new() -> Self {
        Self {
            arrival_date: None,
            created: None,
            destination: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            status: None,
        }
    }
}
/// Returns a list of existing payouts sent to third-party bank accounts or payouts that Stripe sent to you.
/// The payouts return in sorted order, with the most recently created payouts appearing first.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListPayout {
    inner: ListPayoutBuilder,
}
impl ListPayout {
    /// Construct a new `ListPayout`.
    pub fn new() -> Self {
        Self { inner: ListPayoutBuilder::new() }
    }
    /// Only return payouts that are expected to arrive during the given date interval.
    pub fn arrival_date(mut self, arrival_date: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.arrival_date = Some(arrival_date.into());
        self
    }
    /// Only return payouts that were created during the given date interval.
    pub fn created(mut self, created: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.created = Some(created.into());
        self
    }
    /// The ID of an external account - only return payouts sent to this external account.
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
    /// Only return payouts that have the given status: `pending`, `paid`, `failed`, or `canceled`.
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.inner.status = Some(status.into());
        self
    }
}
impl Default for ListPayout {
    fn default() -> Self {
        Self::new()
    }
}
impl ListPayout {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::Payout>> {
        stripe_client_core::ListPaginator::new_list("/payouts", &self.inner)
    }
}

impl StripeRequest for ListPayout {
    type Output = stripe_types::List<stripe_shared::Payout>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/payouts").query(&self.inner)
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct RetrievePayoutBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrievePayoutBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of an existing payout.
/// Supply the unique payout ID from either a payout creation request or the payout list.
/// Stripe returns the corresponding payout information.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrievePayout {
    inner: RetrievePayoutBuilder,
    payout: stripe_shared::PayoutId,
}
impl RetrievePayout {
    /// Construct a new `RetrievePayout`.
    pub fn new(payout: impl Into<stripe_shared::PayoutId>) -> Self {
        Self { payout: payout.into(), inner: RetrievePayoutBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrievePayout {
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

impl StripeRequest for RetrievePayout {
    type Output = stripe_shared::Payout;

    fn build(&self) -> RequestBuilder {
        let payout = &self.payout;
        RequestBuilder::new(StripeMethod::Get, format!("/payouts/{payout}")).query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreatePayoutBuilder {
    amount: i64,
    currency: stripe_types::Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<CreatePayoutMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payout_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_type: Option<CreatePayoutSourceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statement_descriptor: Option<String>,
}
impl CreatePayoutBuilder {
    fn new(amount: impl Into<i64>, currency: impl Into<stripe_types::Currency>) -> Self {
        Self {
            amount: amount.into(),
            currency: currency.into(),
            description: None,
            destination: None,
            expand: None,
            metadata: None,
            method: None,
            payout_method: None,
            source_type: None,
            statement_descriptor: None,
        }
    }
}
/// The method used to send this payout, which is `standard` or `instant`.
/// We support `instant` for payouts to debit cards and bank accounts in certain countries.
/// Learn more about [bank support for Instant Payouts](https://stripe.com/docs/payouts/instant-payouts-banks).
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePayoutMethod {
    Instant,
    Standard,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePayoutMethod {
    pub fn as_str(&self) -> &str {
        use CreatePayoutMethod::*;
        match self {
            Instant => "instant",
            Standard => "standard",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePayoutMethod {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePayoutMethod::*;
        match s {
            "instant" => Ok(Instant),
            "standard" => Ok(Standard),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "CreatePayoutMethod");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreatePayoutMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePayoutMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePayoutMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePayoutMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The balance type of your Stripe balance to draw this payout from.
/// Balances for different payment sources are kept separately.
/// You can find the amounts with the Balances API.
/// One of `bank_account`, `card`, or `fpx`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePayoutSourceType {
    BankAccount,
    Card,
    Fpx,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePayoutSourceType {
    pub fn as_str(&self) -> &str {
        use CreatePayoutSourceType::*;
        match self {
            BankAccount => "bank_account",
            Card => "card",
            Fpx => "fpx",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePayoutSourceType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePayoutSourceType::*;
        match s {
            "bank_account" => Ok(BankAccount),
            "card" => Ok(Card),
            "fpx" => Ok(Fpx),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "CreatePayoutSourceType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreatePayoutSourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePayoutSourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePayoutSourceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePayoutSourceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// To send funds to your own bank account, create a new payout object.
/// Your [Stripe balance](https://stripe.com/docs/api#balance) must cover the payout amount.
/// If it doesn’t, you receive an “Insufficient Funds” error.
///
/// If your API key is in test mode, money won’t actually be sent, though every other action occurs as if you’re in live mode.
///
/// If you create a manual payout on a Stripe account that uses multiple payment source types, you need to specify the source type balance that the payout draws from.
/// The [balance object](https://stripe.com/docs/api#balance_object) details available and pending amounts by source type.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePayout {
    inner: CreatePayoutBuilder,
}
impl CreatePayout {
    /// Construct a new `CreatePayout`.
    pub fn new(amount: impl Into<i64>, currency: impl Into<stripe_types::Currency>) -> Self {
        Self { inner: CreatePayoutBuilder::new(amount.into(), currency.into()) }
    }
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.inner.description = Some(description.into());
        self
    }
    /// The ID of a bank account or a card to send the payout to.
    /// If you don't provide a destination, we use the default external account for the specified currency.
    pub fn destination(mut self, destination: impl Into<String>) -> Self {
        self.inner.destination = Some(destination.into());
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
    /// The method used to send this payout, which is `standard` or `instant`.
    /// We support `instant` for payouts to debit cards and bank accounts in certain countries.
    /// Learn more about [bank support for Instant Payouts](https://stripe.com/docs/payouts/instant-payouts-banks).
    pub fn method(mut self, method: impl Into<CreatePayoutMethod>) -> Self {
        self.inner.method = Some(method.into());
        self
    }
    /// The ID of a v2 FinancialAccount to send funds to.
    pub fn payout_method(mut self, payout_method: impl Into<String>) -> Self {
        self.inner.payout_method = Some(payout_method.into());
        self
    }
    /// The balance type of your Stripe balance to draw this payout from.
    /// Balances for different payment sources are kept separately.
    /// You can find the amounts with the Balances API.
    /// One of `bank_account`, `card`, or `fpx`.
    pub fn source_type(mut self, source_type: impl Into<CreatePayoutSourceType>) -> Self {
        self.inner.source_type = Some(source_type.into());
        self
    }
    /// A string that displays on the recipient's bank or card statement (up to 22 characters).
    /// A `statement_descriptor` that's longer than 22 characters return an error.
    /// Most banks truncate this information and display it inconsistently.
    /// Some banks might not display it at all.
    pub fn statement_descriptor(mut self, statement_descriptor: impl Into<String>) -> Self {
        self.inner.statement_descriptor = Some(statement_descriptor.into());
        self
    }
}
impl CreatePayout {
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

impl StripeRequest for CreatePayout {
    type Output = stripe_shared::Payout;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/payouts").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdatePayoutBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
}
impl UpdatePayoutBuilder {
    fn new() -> Self {
        Self { expand: None, metadata: None }
    }
}
/// Updates the specified payout by setting the values of the parameters you pass.
/// We don’t change parameters that you don’t provide.
/// This request only accepts the metadata as arguments.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdatePayout {
    inner: UpdatePayoutBuilder,
    payout: stripe_shared::PayoutId,
}
impl UpdatePayout {
    /// Construct a new `UpdatePayout`.
    pub fn new(payout: impl Into<stripe_shared::PayoutId>) -> Self {
        Self { payout: payout.into(), inner: UpdatePayoutBuilder::new() }
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
impl UpdatePayout {
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

impl StripeRequest for UpdatePayout {
    type Output = stripe_shared::Payout;

    fn build(&self) -> RequestBuilder {
        let payout = &self.payout;
        RequestBuilder::new(StripeMethod::Post, format!("/payouts/{payout}")).form(&self.inner)
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct CancelPayoutBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl CancelPayoutBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// You can cancel a previously created payout if its status is `pending`.
/// Stripe refunds the funds to your available balance.
/// You can’t cancel automatic Stripe payouts.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CancelPayout {
    inner: CancelPayoutBuilder,
    payout: stripe_shared::PayoutId,
}
impl CancelPayout {
    /// Construct a new `CancelPayout`.
    pub fn new(payout: impl Into<stripe_shared::PayoutId>) -> Self {
        Self { payout: payout.into(), inner: CancelPayoutBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl CancelPayout {
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

impl StripeRequest for CancelPayout {
    type Output = stripe_shared::Payout;

    fn build(&self) -> RequestBuilder {
        let payout = &self.payout;
        RequestBuilder::new(StripeMethod::Post, format!("/payouts/{payout}/cancel"))
            .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct ReversePayoutBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
}
impl ReversePayoutBuilder {
    fn new() -> Self {
        Self { expand: None, metadata: None }
    }
}
/// Reverses a payout by debiting the destination bank account.
/// At this time, you can only reverse payouts for connected accounts to US and Canadian bank accounts.
/// If the payout is manual and in the `pending` status, use `/v1/payouts/:id/cancel` instead.
///
/// By requesting a reversal through `/v1/payouts/:id/reverse`, you confirm that the authorized signatory of the selected bank account authorizes the debit on the bank account and that no other authorization is required.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ReversePayout {
    inner: ReversePayoutBuilder,
    payout: stripe_shared::PayoutId,
}
impl ReversePayout {
    /// Construct a new `ReversePayout`.
    pub fn new(payout: impl Into<stripe_shared::PayoutId>) -> Self {
        Self { payout: payout.into(), inner: ReversePayoutBuilder::new() }
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
impl ReversePayout {
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

impl StripeRequest for ReversePayout {
    type Output = stripe_shared::Payout;

    fn build(&self) -> RequestBuilder {
        let payout = &self.payout;
        RequestBuilder::new(StripeMethod::Post, format!("/payouts/{payout}/reverse"))
            .form(&self.inner)
    }
}
