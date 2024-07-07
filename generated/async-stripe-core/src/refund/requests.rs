use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListRefundBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    charge: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_intent: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListRefundBuilder<'a> {
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
/// We return the refunds in sorted order, with the most recent refunds appearing first The 10 most recent refunds are always available by default on the Charge object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListRefund<'a> {
    inner: ListRefundBuilder<'a>,
}
impl<'a> ListRefund<'a> {
    /// Construct a new `ListRefund`.
    pub fn new() -> Self {
        Self { inner: ListRefundBuilder::new() }
    }
    /// Only return refunds for the charge specified by this charge ID.
    pub fn charge(mut self, charge: &'a str) -> Self {
        self.inner.charge = Some(charge);
        self
    }
    /// Only return refunds that were created during the given date interval.
    pub fn created(mut self, created: stripe_types::RangeQueryTs) -> Self {
        self.inner.created = Some(created);
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
    /// Only return refunds for the PaymentIntent specified by this ID.
    pub fn payment_intent(mut self, payment_intent: &'a str) -> Self {
        self.inner.payment_intent = Some(payment_intent);
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
impl<'a> Default for ListRefund<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListRefund<'_> {
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
        stripe_client_core::ListPaginator::new_list("/refunds", self.inner)
    }
}

impl StripeRequest for ListRefund<'_> {
    type Output = stripe_types::List<stripe_shared::Refund>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/refunds").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveRefundBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveRefundBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of an existing refund.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveRefund<'a> {
    inner: RetrieveRefundBuilder<'a>,
    refund: &'a stripe_shared::RefundId,
}
impl<'a> RetrieveRefund<'a> {
    /// Construct a new `RetrieveRefund`.
    pub fn new(refund: &'a stripe_shared::RefundId) -> Self {
        Self { refund, inner: RetrieveRefundBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveRefund<'_> {
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

impl StripeRequest for RetrieveRefund<'_> {
    type Output = stripe_shared::Refund;

    fn build(&self) -> RequestBuilder {
        let refund = self.refund;
        RequestBuilder::new(StripeMethod::Get, format!("/refunds/{refund}")).query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateRefundBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    charge: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<stripe_types::Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instructions_email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<CreateRefundOrigin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_intent: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<CreateRefundReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refund_application_fee: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reverse_transfer: Option<bool>,
}
impl<'a> CreateRefundBuilder<'a> {
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateRefundOrigin {
    CustomerBalance,
}
impl CreateRefundOrigin {
    pub fn as_str(self) -> &'static str {
        use CreateRefundOrigin::*;
        match self {
            CustomerBalance => "customer_balance",
        }
    }
}

impl std::str::FromStr for CreateRefundOrigin {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateRefundOrigin::*;
        match s {
            "customer_balance" => Ok(CustomerBalance),
            _ => Err(stripe_types::StripeParseError),
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
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CreateRefundOrigin"))
    }
}
/// String indicating the reason for the refund.
/// If set, possible values are `duplicate`, `fraudulent`, and `requested_by_customer`.
/// If you believe the charge to be fraudulent, specifying `fraudulent` as the reason will add the associated card and email to your [block lists](https://stripe.com/docs/radar/lists), and will also help us improve our fraud detection algorithms.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateRefundReason {
    Duplicate,
    Fraudulent,
    RequestedByCustomer,
}
impl CreateRefundReason {
    pub fn as_str(self) -> &'static str {
        use CreateRefundReason::*;
        match self {
            Duplicate => "duplicate",
            Fraudulent => "fraudulent",
            RequestedByCustomer => "requested_by_customer",
        }
    }
}

impl std::str::FromStr for CreateRefundReason {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateRefundReason::*;
        match s {
            "duplicate" => Ok(Duplicate),
            "fraudulent" => Ok(Fraudulent),
            "requested_by_customer" => Ok(RequestedByCustomer),
            _ => Err(stripe_types::StripeParseError),
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
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CreateRefundReason"))
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
pub struct CreateRefund<'a> {
    inner: CreateRefundBuilder<'a>,
}
impl<'a> CreateRefund<'a> {
    /// Construct a new `CreateRefund`.
    pub fn new() -> Self {
        Self { inner: CreateRefundBuilder::new() }
    }
    pub fn amount(mut self, amount: i64) -> Self {
        self.inner.amount = Some(amount);
        self
    }
    /// The identifier of the charge to refund.
    pub fn charge(mut self, charge: &'a str) -> Self {
        self.inner.charge = Some(charge);
        self
    }
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub fn currency(mut self, currency: stripe_types::Currency) -> Self {
        self.inner.currency = Some(currency);
        self
    }
    /// Customer whose customer balance to refund from.
    pub fn customer(mut self, customer: &'a str) -> Self {
        self.inner.customer = Some(customer);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// For payment methods without native refund support (e.g., Konbini, PromptPay), use this email from the customer to receive refund instructions.
    pub fn instructions_email(mut self, instructions_email: &'a str) -> Self {
        self.inner.instructions_email = Some(instructions_email);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    /// Origin of the refund
    pub fn origin(mut self, origin: CreateRefundOrigin) -> Self {
        self.inner.origin = Some(origin);
        self
    }
    /// The identifier of the PaymentIntent to refund.
    pub fn payment_intent(mut self, payment_intent: &'a str) -> Self {
        self.inner.payment_intent = Some(payment_intent);
        self
    }
    /// String indicating the reason for the refund.
    /// If set, possible values are `duplicate`, `fraudulent`, and `requested_by_customer`.
    /// If you believe the charge to be fraudulent, specifying `fraudulent` as the reason will add the associated card and email to your [block lists](https://stripe.com/docs/radar/lists), and will also help us improve our fraud detection algorithms.
    pub fn reason(mut self, reason: CreateRefundReason) -> Self {
        self.inner.reason = Some(reason);
        self
    }
    /// Boolean indicating whether the application fee should be refunded when refunding this charge.
    /// If a full charge refund is given, the full application fee will be refunded.
    /// Otherwise, the application fee will be refunded in an amount proportional to the amount of the charge refunded.
    /// An application fee can be refunded only by the application that created the charge.
    pub fn refund_application_fee(mut self, refund_application_fee: bool) -> Self {
        self.inner.refund_application_fee = Some(refund_application_fee);
        self
    }
    /// Boolean indicating whether the transfer should be reversed when refunding this charge.
    /// The transfer will be reversed proportionally to the amount being refunded (either the entire or partial amount).
    ///
    /// A transfer can be reversed only by the application that created the charge.
    pub fn reverse_transfer(mut self, reverse_transfer: bool) -> Self {
        self.inner.reverse_transfer = Some(reverse_transfer);
        self
    }
}
impl<'a> Default for CreateRefund<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl CreateRefund<'_> {
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

impl StripeRequest for CreateRefund<'_> {
    type Output = stripe_shared::Refund;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/refunds").form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdateRefundBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> UpdateRefundBuilder<'a> {
    fn new() -> Self {
        Self { expand: None, metadata: None }
    }
}
/// Updates the refund that you specify by setting the values of the passed parameters.
/// Any parameters that you don’t provide remain unchanged.
///
/// This request only accepts `metadata` as an argument.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateRefund<'a> {
    inner: UpdateRefundBuilder<'a>,
    refund: &'a stripe_shared::RefundId,
}
impl<'a> UpdateRefund<'a> {
    /// Construct a new `UpdateRefund`.
    pub fn new(refund: &'a stripe_shared::RefundId) -> Self {
        Self { refund, inner: UpdateRefundBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
}
impl UpdateRefund<'_> {
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

impl StripeRequest for UpdateRefund<'_> {
    type Output = stripe_shared::Refund;

    fn build(&self) -> RequestBuilder {
        let refund = self.refund;
        RequestBuilder::new(StripeMethod::Post, format!("/refunds/{refund}")).form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CancelRefundBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> CancelRefundBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Cancels a refund with a status of `requires_action`.
///
/// You can’t cancel refunds in other states.
/// Only refunds for payment methods that require customer action can enter the `requires_action` state.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CancelRefund<'a> {
    inner: CancelRefundBuilder<'a>,
    refund: &'a stripe_shared::RefundId,
}
impl<'a> CancelRefund<'a> {
    /// Construct a new `CancelRefund`.
    pub fn new(refund: &'a stripe_shared::RefundId) -> Self {
        Self { refund, inner: CancelRefundBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl CancelRefund<'_> {
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

impl StripeRequest for CancelRefund<'_> {
    type Output = stripe_shared::Refund;

    fn build(&self) -> RequestBuilder {
        let refund = self.refund;
        RequestBuilder::new(StripeMethod::Post, format!("/refunds/{refund}/cancel"))
            .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ExpireRefundBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> ExpireRefundBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Expire a refund with a status of `requires_action`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ExpireRefund<'a> {
    inner: ExpireRefundBuilder<'a>,
    refund: &'a str,
}
impl<'a> ExpireRefund<'a> {
    /// Construct a new `ExpireRefund`.
    pub fn new(refund: &'a str) -> Self {
        Self { refund, inner: ExpireRefundBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl ExpireRefund<'_> {
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

impl StripeRequest for ExpireRefund<'_> {
    type Output = stripe_shared::Refund;

    fn build(&self) -> RequestBuilder {
        let refund = self.refund;
        RequestBuilder::new(StripeMethod::Post, format!("/test_helpers/refunds/{refund}/expire"))
            .form(&self.inner)
    }
}
