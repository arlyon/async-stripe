use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct ListRefundBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    charge: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_intent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListRefundBuilder {
    fn new() -> Self {
        Self {
            charge: None,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            payment_intent: None,
            starting_after: None,
        }
    }
}
/// Returns a list of all refunds you created.
/// We return the refunds in sorted order, with the most recent refunds appearing first.
/// The 10 most recent refunds are always available by default on the Charge object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListRefund {
    inner: ListRefundBuilder,
}
impl ListRefund {
    /// Construct a new `ListRefund`.
    pub fn new() -> Self {
        Self { inner: ListRefundBuilder::new() }
    }
    /// Only return refunds for the charge specified by this charge ID.
    pub fn charge(mut self, charge: impl Into<String>) -> Self {
        self.inner.charge = Some(charge.into());
        self
    }
    /// Only return refunds that were created during the given date interval.
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
    /// Only return refunds for the PaymentIntent specified by this ID.
    pub fn payment_intent(mut self, payment_intent: impl Into<String>) -> Self {
        self.inner.payment_intent = Some(payment_intent.into());
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
impl Default for ListRefund {
    fn default() -> Self {
        Self::new()
    }
}
impl ListRefund {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::Refund>> {
        stripe_client_core::ListPaginator::new_list("/refunds", &self.inner)
    }
}

impl StripeRequest for ListRefund {
    type Output = stripe_types::List<stripe_shared::Refund>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/refunds").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveRefundBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveRefundBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of an existing refund.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveRefund {
    inner: RetrieveRefundBuilder,
    refund: stripe_shared::RefundId,
}
impl RetrieveRefund {
    /// Construct a new `RetrieveRefund`.
    pub fn new(refund: impl Into<stripe_shared::RefundId>) -> Self {
        Self { refund: refund.into(), inner: RetrieveRefundBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveRefund {
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

impl StripeRequest for RetrieveRefund {
    type Output = stripe_shared::Refund;

    fn build(&self) -> RequestBuilder {
        let refund = &self.refund;
        RequestBuilder::new(StripeMethod::Get, format!("/refunds/{refund}")).query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreateRefundBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    charge: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<stripe_types::Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instructions_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<CreateRefundOrigin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_intent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<CreateRefundReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refund_application_fee: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reverse_transfer: Option<bool>,
}
impl CreateRefundBuilder {
    fn new() -> Self {
        Self {
            amount: None,
            charge: None,
            currency: None,
            customer: None,
            expand: None,
            instructions_email: None,
            metadata: None,
            origin: None,
            payment_intent: None,
            reason: None,
            refund_application_fee: None,
            reverse_transfer: None,
        }
    }
}
/// Origin of the refund
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateRefundOrigin {
    CustomerBalance,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateRefundOrigin {
    pub fn as_str(&self) -> &str {
        use CreateRefundOrigin::*;
        match self {
            CustomerBalance => "customer_balance",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateRefundOrigin {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateRefundOrigin::*;
        match s {
            "customer_balance" => Ok(CustomerBalance),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "CreateRefundOrigin");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateRefundOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateRefundOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateRefundOrigin {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateRefundOrigin {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// String indicating the reason for the refund.
/// If set, possible values are `duplicate`, `fraudulent`, and `requested_by_customer`.
/// If you believe the charge to be fraudulent, specifying `fraudulent` as the reason will add the associated card and email to your [block lists](https://docs.stripe.com/radar/lists), and will also help us improve our fraud detection algorithms.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateRefundReason {
    Duplicate,
    Fraudulent,
    RequestedByCustomer,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateRefundReason {
    pub fn as_str(&self) -> &str {
        use CreateRefundReason::*;
        match self {
            Duplicate => "duplicate",
            Fraudulent => "fraudulent",
            RequestedByCustomer => "requested_by_customer",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateRefundReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateRefundReason::*;
        match s {
            "duplicate" => Ok(Duplicate),
            "fraudulent" => Ok(Fraudulent),
            "requested_by_customer" => Ok(RequestedByCustomer),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "CreateRefundReason");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateRefundReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateRefundReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateRefundReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateRefundReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// When you create a new refund, you must specify a Charge or a PaymentIntent object on which to create it.
///
/// Creating a new refund will refund a charge that has previously been created but not yet refunded.
/// Funds will be refunded to the credit or debit card that was originally charged.
///
/// You can optionally refund only part of a charge.
/// You can do so multiple times, until the entire charge has been refunded.
///
/// Once entirely refunded, a charge can’t be refunded again.
/// This method will raise an error when called on an already-refunded charge,
/// or when trying to refund more money than is left on a charge.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateRefund {
    inner: CreateRefundBuilder,
}
impl CreateRefund {
    /// Construct a new `CreateRefund`.
    pub fn new() -> Self {
        Self { inner: CreateRefundBuilder::new() }
    }
    pub fn amount(mut self, amount: impl Into<i64>) -> Self {
        self.inner.amount = Some(amount.into());
        self
    }
    /// The identifier of the charge to refund.
    pub fn charge(mut self, charge: impl Into<String>) -> Self {
        self.inner.charge = Some(charge.into());
        self
    }
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub fn currency(mut self, currency: impl Into<stripe_types::Currency>) -> Self {
        self.inner.currency = Some(currency.into());
        self
    }
    /// Customer whose customer balance to refund from.
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.inner.customer = Some(customer.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// For payment methods without native refund support (e.g., Konbini, PromptPay), use this email from the customer to receive refund instructions.
    pub fn instructions_email(mut self, instructions_email: impl Into<String>) -> Self {
        self.inner.instructions_email = Some(instructions_email.into());
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
    /// Origin of the refund
    pub fn origin(mut self, origin: impl Into<CreateRefundOrigin>) -> Self {
        self.inner.origin = Some(origin.into());
        self
    }
    /// The identifier of the PaymentIntent to refund.
    pub fn payment_intent(mut self, payment_intent: impl Into<String>) -> Self {
        self.inner.payment_intent = Some(payment_intent.into());
        self
    }
    /// String indicating the reason for the refund.
    /// If set, possible values are `duplicate`, `fraudulent`, and `requested_by_customer`.
    /// If you believe the charge to be fraudulent, specifying `fraudulent` as the reason will add the associated card and email to your [block lists](https://docs.stripe.com/radar/lists), and will also help us improve our fraud detection algorithms.
    pub fn reason(mut self, reason: impl Into<CreateRefundReason>) -> Self {
        self.inner.reason = Some(reason.into());
        self
    }
    /// Boolean indicating whether the application fee should be refunded when refunding this charge.
    /// If a full charge refund is given, the full application fee will be refunded.
    /// Otherwise, the application fee will be refunded in an amount proportional to the amount of the charge refunded.
    /// An application fee can be refunded only by the application that created the charge.
    pub fn refund_application_fee(mut self, refund_application_fee: impl Into<bool>) -> Self {
        self.inner.refund_application_fee = Some(refund_application_fee.into());
        self
    }
    /// Boolean indicating whether the transfer should be reversed when refunding this charge.
    /// The transfer will be reversed proportionally to the amount being refunded (either the entire or partial amount).
    ///
    /// A transfer can be reversed only by the application that created the charge.
    pub fn reverse_transfer(mut self, reverse_transfer: impl Into<bool>) -> Self {
        self.inner.reverse_transfer = Some(reverse_transfer.into());
        self
    }
}
impl Default for CreateRefund {
    fn default() -> Self {
        Self::new()
    }
}
impl CreateRefund {
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

impl StripeRequest for CreateRefund {
    type Output = stripe_shared::Refund;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/refunds").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdateRefundBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
}
impl UpdateRefundBuilder {
    fn new() -> Self {
        Self { expand: None, metadata: None }
    }
}
/// Updates the refund that you specify by setting the values of the passed parameters.
/// Any parameters that you don’t provide remain unchanged.
///
/// This request only accepts `metadata` as an argument.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateRefund {
    inner: UpdateRefundBuilder,
    refund: stripe_shared::RefundId,
}
impl UpdateRefund {
    /// Construct a new `UpdateRefund`.
    pub fn new(refund: impl Into<stripe_shared::RefundId>) -> Self {
        Self { refund: refund.into(), inner: UpdateRefundBuilder::new() }
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
impl UpdateRefund {
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

impl StripeRequest for UpdateRefund {
    type Output = stripe_shared::Refund;

    fn build(&self) -> RequestBuilder {
        let refund = &self.refund;
        RequestBuilder::new(StripeMethod::Post, format!("/refunds/{refund}")).form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CancelRefundBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl CancelRefundBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Cancels a refund with a status of `requires_action`.
///
/// You can’t cancel refunds in other states.
/// Only refunds for payment methods that require customer action can enter the `requires_action` state.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CancelRefund {
    inner: CancelRefundBuilder,
    refund: stripe_shared::RefundId,
}
impl CancelRefund {
    /// Construct a new `CancelRefund`.
    pub fn new(refund: impl Into<stripe_shared::RefundId>) -> Self {
        Self { refund: refund.into(), inner: CancelRefundBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl CancelRefund {
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

impl StripeRequest for CancelRefund {
    type Output = stripe_shared::Refund;

    fn build(&self) -> RequestBuilder {
        let refund = &self.refund;
        RequestBuilder::new(StripeMethod::Post, format!("/refunds/{refund}/cancel"))
            .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct ExpireRefundBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl ExpireRefundBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Expire a refund with a status of `requires_action`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ExpireRefund {
    inner: ExpireRefundBuilder,
    refund: String,
}
impl ExpireRefund {
    /// Construct a new `ExpireRefund`.
    pub fn new(refund: impl Into<String>) -> Self {
        Self { refund: refund.into(), inner: ExpireRefundBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl ExpireRefund {
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

impl StripeRequest for ExpireRefund {
    type Output = stripe_shared::Refund;

    fn build(&self) -> RequestBuilder {
        let refund = &self.refund;
        RequestBuilder::new(StripeMethod::Post, format!("/test_helpers/refunds/{refund}/expire"))
            .form(&self.inner)
    }
}
