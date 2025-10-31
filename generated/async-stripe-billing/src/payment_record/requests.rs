use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct RetrievePaymentRecordBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrievePaymentRecordBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a Payment Record with the given ID
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrievePaymentRecord {
    inner: RetrievePaymentRecordBuilder,
    id: stripe_shared::PaymentRecordId,
}
impl RetrievePaymentRecord {
    /// Construct a new `RetrievePaymentRecord`.
    pub fn new(id: impl Into<stripe_shared::PaymentRecordId>) -> Self {
        Self { id: id.into(), inner: RetrievePaymentRecordBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrievePaymentRecord {
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

impl StripeRequest for RetrievePaymentRecord {
    type Output = stripe_shared::PaymentRecord;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/payment_records/{id}")).query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct ReportPaymentAttemptPaymentRecordBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failed: Option<Failed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guaranteed: Option<Guaranteed>,
    initiated_at: stripe_types::Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outcome: Option<ReportPaymentAttemptPaymentRecordOutcome>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_details: Option<ReportPaymentAttemptPaymentRecordPaymentMethodDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_details: Option<ShippingDetails>,
}
impl ReportPaymentAttemptPaymentRecordBuilder {
    fn new(initiated_at: impl Into<stripe_types::Timestamp>) -> Self {
        Self {
            description: None,
            expand: None,
            failed: None,
            guaranteed: None,
            initiated_at: initiated_at.into(),
            metadata: None,
            outcome: None,
            payment_method_details: None,
            shipping_details: None,
        }
    }
}
/// The outcome of the reported payment.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ReportPaymentAttemptPaymentRecordOutcome {
    Failed,
    Guaranteed,
}
impl ReportPaymentAttemptPaymentRecordOutcome {
    pub fn as_str(self) -> &'static str {
        use ReportPaymentAttemptPaymentRecordOutcome::*;
        match self {
            Failed => "failed",
            Guaranteed => "guaranteed",
        }
    }
}

impl std::str::FromStr for ReportPaymentAttemptPaymentRecordOutcome {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ReportPaymentAttemptPaymentRecordOutcome::*;
        match s {
            "failed" => Ok(Failed),
            "guaranteed" => Ok(Guaranteed),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ReportPaymentAttemptPaymentRecordOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ReportPaymentAttemptPaymentRecordOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ReportPaymentAttemptPaymentRecordOutcome {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ReportPaymentAttemptPaymentRecordOutcome {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for ReportPaymentAttemptPaymentRecordOutcome")
        })
    }
}
/// Information about the Payment Method debited for this payment.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ReportPaymentAttemptPaymentRecordPaymentMethodDetails {
    /// The billing details associated with the method of payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<BillingDetails>,
    /// Information about the custom (user-defined) payment method used to make this payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<Custom>,
    /// ID of the Stripe Payment Method used to make this payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    /// The type of the payment method details.
    /// An additional hash is included on the payment_method_details with a name matching this value.
    /// It contains additional information specific to the type.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<ReportPaymentAttemptPaymentRecordPaymentMethodDetailsType>,
}
impl ReportPaymentAttemptPaymentRecordPaymentMethodDetails {
    pub fn new() -> Self {
        Self { billing_details: None, custom: None, payment_method: None, type_: None }
    }
}
impl Default for ReportPaymentAttemptPaymentRecordPaymentMethodDetails {
    fn default() -> Self {
        Self::new()
    }
}
/// The type of the payment method details.
/// An additional hash is included on the payment_method_details with a name matching this value.
/// It contains additional information specific to the type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ReportPaymentAttemptPaymentRecordPaymentMethodDetailsType {
    Custom,
}
impl ReportPaymentAttemptPaymentRecordPaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        use ReportPaymentAttemptPaymentRecordPaymentMethodDetailsType::*;
        match self {
            Custom => "custom",
        }
    }
}

impl std::str::FromStr for ReportPaymentAttemptPaymentRecordPaymentMethodDetailsType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ReportPaymentAttemptPaymentRecordPaymentMethodDetailsType::*;
        match s {
            "custom" => Ok(Custom),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ReportPaymentAttemptPaymentRecordPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ReportPaymentAttemptPaymentRecordPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ReportPaymentAttemptPaymentRecordPaymentMethodDetailsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ReportPaymentAttemptPaymentRecordPaymentMethodDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for ReportPaymentAttemptPaymentRecordPaymentMethodDetailsType",
            )
        })
    }
}
/// Report a new payment attempt on the specified Payment Record. A new payment
///  attempt can only be specified if all other payment attempts are canceled or failed.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ReportPaymentAttemptPaymentRecord {
    inner: ReportPaymentAttemptPaymentRecordBuilder,
    id: stripe_shared::PaymentRecordId,
}
impl ReportPaymentAttemptPaymentRecord {
    /// Construct a new `ReportPaymentAttemptPaymentRecord`.
    pub fn new(
        id: impl Into<stripe_shared::PaymentRecordId>,
        initiated_at: impl Into<stripe_types::Timestamp>,
    ) -> Self {
        Self {
            id: id.into(),
            inner: ReportPaymentAttemptPaymentRecordBuilder::new(initiated_at.into()),
        }
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
    /// Information about the payment attempt failure.
    pub fn failed(mut self, failed: impl Into<Failed>) -> Self {
        self.inner.failed = Some(failed.into());
        self
    }
    /// Information about the payment attempt guarantee.
    pub fn guaranteed(mut self, guaranteed: impl Into<Guaranteed>) -> Self {
        self.inner.guaranteed = Some(guaranteed.into());
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
    /// The outcome of the reported payment.
    pub fn outcome(mut self, outcome: impl Into<ReportPaymentAttemptPaymentRecordOutcome>) -> Self {
        self.inner.outcome = Some(outcome.into());
        self
    }
    /// Information about the Payment Method debited for this payment.
    pub fn payment_method_details(
        mut self,
        payment_method_details: impl Into<ReportPaymentAttemptPaymentRecordPaymentMethodDetails>,
    ) -> Self {
        self.inner.payment_method_details = Some(payment_method_details.into());
        self
    }
    /// Shipping information for this payment.
    pub fn shipping_details(mut self, shipping_details: impl Into<ShippingDetails>) -> Self {
        self.inner.shipping_details = Some(shipping_details.into());
        self
    }
}
impl ReportPaymentAttemptPaymentRecord {
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

impl StripeRequest for ReportPaymentAttemptPaymentRecord {
    type Output = stripe_shared::PaymentRecord;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/payment_records/{id}/report_payment_attempt"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct ReportPaymentAttemptCanceledPaymentRecordBuilder {
    canceled_at: stripe_types::Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
}
impl ReportPaymentAttemptCanceledPaymentRecordBuilder {
    fn new(canceled_at: impl Into<stripe_types::Timestamp>) -> Self {
        Self { canceled_at: canceled_at.into(), expand: None, metadata: None }
    }
}
/// Report that the most recent payment attempt on the specified Payment Record
///  was canceled.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ReportPaymentAttemptCanceledPaymentRecord {
    inner: ReportPaymentAttemptCanceledPaymentRecordBuilder,
    id: stripe_shared::PaymentRecordId,
}
impl ReportPaymentAttemptCanceledPaymentRecord {
    /// Construct a new `ReportPaymentAttemptCanceledPaymentRecord`.
    pub fn new(
        id: impl Into<stripe_shared::PaymentRecordId>,
        canceled_at: impl Into<stripe_types::Timestamp>,
    ) -> Self {
        Self {
            id: id.into(),
            inner: ReportPaymentAttemptCanceledPaymentRecordBuilder::new(canceled_at.into()),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
}
impl ReportPaymentAttemptCanceledPaymentRecord {
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

impl StripeRequest for ReportPaymentAttemptCanceledPaymentRecord {
    type Output = stripe_shared::PaymentRecord;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/payment_records/{id}/report_payment_attempt_canceled"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct ReportPaymentAttemptFailedPaymentRecordBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    failed_at: stripe_types::Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
}
impl ReportPaymentAttemptFailedPaymentRecordBuilder {
    fn new(failed_at: impl Into<stripe_types::Timestamp>) -> Self {
        Self { expand: None, failed_at: failed_at.into(), metadata: None }
    }
}
/// Report that the most recent payment attempt on the specified Payment Record
///  failed or errored.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ReportPaymentAttemptFailedPaymentRecord {
    inner: ReportPaymentAttemptFailedPaymentRecordBuilder,
    id: stripe_shared::PaymentRecordId,
}
impl ReportPaymentAttemptFailedPaymentRecord {
    /// Construct a new `ReportPaymentAttemptFailedPaymentRecord`.
    pub fn new(
        id: impl Into<stripe_shared::PaymentRecordId>,
        failed_at: impl Into<stripe_types::Timestamp>,
    ) -> Self {
        Self {
            id: id.into(),
            inner: ReportPaymentAttemptFailedPaymentRecordBuilder::new(failed_at.into()),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
}
impl ReportPaymentAttemptFailedPaymentRecord {
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

impl StripeRequest for ReportPaymentAttemptFailedPaymentRecord {
    type Output = stripe_shared::PaymentRecord;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/payment_records/{id}/report_payment_attempt_failed"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct ReportPaymentAttemptGuaranteedPaymentRecordBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    guaranteed_at: stripe_types::Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
}
impl ReportPaymentAttemptGuaranteedPaymentRecordBuilder {
    fn new(guaranteed_at: impl Into<stripe_types::Timestamp>) -> Self {
        Self { expand: None, guaranteed_at: guaranteed_at.into(), metadata: None }
    }
}
/// Report that the most recent payment attempt on the specified Payment Record
///  was guaranteed.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ReportPaymentAttemptGuaranteedPaymentRecord {
    inner: ReportPaymentAttemptGuaranteedPaymentRecordBuilder,
    id: stripe_shared::PaymentRecordId,
}
impl ReportPaymentAttemptGuaranteedPaymentRecord {
    /// Construct a new `ReportPaymentAttemptGuaranteedPaymentRecord`.
    pub fn new(
        id: impl Into<stripe_shared::PaymentRecordId>,
        guaranteed_at: impl Into<stripe_types::Timestamp>,
    ) -> Self {
        Self {
            id: id.into(),
            inner: ReportPaymentAttemptGuaranteedPaymentRecordBuilder::new(guaranteed_at.into()),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
}
impl ReportPaymentAttemptGuaranteedPaymentRecord {
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

impl StripeRequest for ReportPaymentAttemptGuaranteedPaymentRecord {
    type Output = stripe_shared::PaymentRecord;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/payment_records/{id}/report_payment_attempt_guaranteed"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct ReportPaymentAttemptInformationalPaymentRecordBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_details: Option<CustomerDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_details: Option<ShippingDetails>,
}
impl ReportPaymentAttemptInformationalPaymentRecordBuilder {
    fn new() -> Self {
        Self {
            customer_details: None,
            description: None,
            expand: None,
            metadata: None,
            shipping_details: None,
        }
    }
}
/// Report informational updates on the specified Payment Record.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ReportPaymentAttemptInformationalPaymentRecord {
    inner: ReportPaymentAttemptInformationalPaymentRecordBuilder,
    id: stripe_shared::PaymentRecordId,
}
impl ReportPaymentAttemptInformationalPaymentRecord {
    /// Construct a new `ReportPaymentAttemptInformationalPaymentRecord`.
    pub fn new(id: impl Into<stripe_shared::PaymentRecordId>) -> Self {
        Self { id: id.into(), inner: ReportPaymentAttemptInformationalPaymentRecordBuilder::new() }
    }
    /// Customer information for this payment.
    pub fn customer_details(mut self, customer_details: impl Into<CustomerDetails>) -> Self {
        self.inner.customer_details = Some(customer_details.into());
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
    /// Shipping information for this payment.
    pub fn shipping_details(mut self, shipping_details: impl Into<ShippingDetails>) -> Self {
        self.inner.shipping_details = Some(shipping_details.into());
        self
    }
}
impl ReportPaymentAttemptInformationalPaymentRecord {
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

impl StripeRequest for ReportPaymentAttemptInformationalPaymentRecord {
    type Output = stripe_shared::PaymentRecord;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/payment_records/{id}/report_payment_attempt_informational"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct ReportRefundPaymentRecordBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<Amount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initiated_at: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    outcome: ReportRefundPaymentRecordOutcome,
    processor_details: ReportRefundPaymentRecordProcessorDetails,
    refunded: ReportRefundPaymentRecordRefunded,
}
impl ReportRefundPaymentRecordBuilder {
    fn new(
        outcome: impl Into<ReportRefundPaymentRecordOutcome>,
        processor_details: impl Into<ReportRefundPaymentRecordProcessorDetails>,
        refunded: impl Into<ReportRefundPaymentRecordRefunded>,
    ) -> Self {
        Self {
            amount: None,
            expand: None,
            initiated_at: None,
            metadata: None,
            outcome: outcome.into(),
            processor_details: processor_details.into(),
            refunded: refunded.into(),
        }
    }
}
/// The outcome of the reported refund.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ReportRefundPaymentRecordOutcome {
    Refunded,
}
impl ReportRefundPaymentRecordOutcome {
    pub fn as_str(self) -> &'static str {
        use ReportRefundPaymentRecordOutcome::*;
        match self {
            Refunded => "refunded",
        }
    }
}

impl std::str::FromStr for ReportRefundPaymentRecordOutcome {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ReportRefundPaymentRecordOutcome::*;
        match s {
            "refunded" => Ok(Refunded),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ReportRefundPaymentRecordOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ReportRefundPaymentRecordOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ReportRefundPaymentRecordOutcome {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ReportRefundPaymentRecordOutcome {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for ReportRefundPaymentRecordOutcome")
        })
    }
}
/// Processor information for this refund.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ReportRefundPaymentRecordProcessorDetails {
    /// Information about the custom processor used to make this refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<ReportRefundPaymentRecordProcessorDetailsCustom>,
    /// The type of the processor details.
    /// An additional hash is included on processor_details with a name matching this value.
    /// It contains additional information specific to the processor.
    #[serde(rename = "type")]
    pub type_: ReportRefundPaymentRecordProcessorDetailsType,
}
impl ReportRefundPaymentRecordProcessorDetails {
    pub fn new(type_: impl Into<ReportRefundPaymentRecordProcessorDetailsType>) -> Self {
        Self { custom: None, type_: type_.into() }
    }
}
/// Information about the custom processor used to make this refund.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ReportRefundPaymentRecordProcessorDetailsCustom {
    /// A reference to the external refund. This field must be unique across all refunds.
    pub refund_reference: String,
}
impl ReportRefundPaymentRecordProcessorDetailsCustom {
    pub fn new(refund_reference: impl Into<String>) -> Self {
        Self { refund_reference: refund_reference.into() }
    }
}
/// The type of the processor details.
/// An additional hash is included on processor_details with a name matching this value.
/// It contains additional information specific to the processor.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ReportRefundPaymentRecordProcessorDetailsType {
    Custom,
}
impl ReportRefundPaymentRecordProcessorDetailsType {
    pub fn as_str(self) -> &'static str {
        use ReportRefundPaymentRecordProcessorDetailsType::*;
        match self {
            Custom => "custom",
        }
    }
}

impl std::str::FromStr for ReportRefundPaymentRecordProcessorDetailsType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ReportRefundPaymentRecordProcessorDetailsType::*;
        match s {
            "custom" => Ok(Custom),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ReportRefundPaymentRecordProcessorDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ReportRefundPaymentRecordProcessorDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ReportRefundPaymentRecordProcessorDetailsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ReportRefundPaymentRecordProcessorDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for ReportRefundPaymentRecordProcessorDetailsType",
            )
        })
    }
}
/// Information about the payment attempt refund.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ReportRefundPaymentRecordRefunded {
    /// When the reported refund completed. Measured in seconds since the Unix epoch.
    pub refunded_at: stripe_types::Timestamp,
}
impl ReportRefundPaymentRecordRefunded {
    pub fn new(refunded_at: impl Into<stripe_types::Timestamp>) -> Self {
        Self { refunded_at: refunded_at.into() }
    }
}
/// Report that the most recent payment attempt on the specified Payment Record
///  was refunded.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ReportRefundPaymentRecord {
    inner: ReportRefundPaymentRecordBuilder,
    id: stripe_shared::PaymentRecordId,
}
impl ReportRefundPaymentRecord {
    /// Construct a new `ReportRefundPaymentRecord`.
    pub fn new(
        id: impl Into<stripe_shared::PaymentRecordId>,
        outcome: impl Into<ReportRefundPaymentRecordOutcome>,
        processor_details: impl Into<ReportRefundPaymentRecordProcessorDetails>,
        refunded: impl Into<ReportRefundPaymentRecordRefunded>,
    ) -> Self {
        Self {
            id: id.into(),
            inner: ReportRefundPaymentRecordBuilder::new(
                outcome.into(),
                processor_details.into(),
                refunded.into(),
            ),
        }
    }
    /// A positive integer in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) representing how much of this payment to refund.
    /// Can refund only up to the remaining, unrefunded amount of the payment.
    pub fn amount(mut self, amount: impl Into<Amount>) -> Self {
        self.inner.amount = Some(amount.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// When the reported refund was initiated. Measured in seconds since the Unix epoch.
    pub fn initiated_at(mut self, initiated_at: impl Into<stripe_types::Timestamp>) -> Self {
        self.inner.initiated_at = Some(initiated_at.into());
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
impl ReportRefundPaymentRecord {
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

impl StripeRequest for ReportRefundPaymentRecord {
    type Output = stripe_shared::PaymentRecord;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Post, format!("/payment_records/{id}/report_refund"))
            .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct ReportPaymentPaymentRecordBuilder {
    amount_requested: Amount,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_details: Option<CustomerDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_presence: Option<stripe_shared::PaymentRecordCustomerPresence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failed: Option<Failed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guaranteed: Option<Guaranteed>,
    initiated_at: stripe_types::Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outcome: Option<ReportPaymentPaymentRecordOutcome>,
    payment_method_details: ReportPaymentPaymentRecordPaymentMethodDetails,
    #[serde(skip_serializing_if = "Option::is_none")]
    processor_details: Option<ReportPaymentPaymentRecordProcessorDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_details: Option<ShippingDetails>,
}
impl ReportPaymentPaymentRecordBuilder {
    fn new(
        amount_requested: impl Into<Amount>,
        initiated_at: impl Into<stripe_types::Timestamp>,
        payment_method_details: impl Into<ReportPaymentPaymentRecordPaymentMethodDetails>,
    ) -> Self {
        Self {
            amount_requested: amount_requested.into(),
            customer_details: None,
            customer_presence: None,
            description: None,
            expand: None,
            failed: None,
            guaranteed: None,
            initiated_at: initiated_at.into(),
            metadata: None,
            outcome: None,
            payment_method_details: payment_method_details.into(),
            processor_details: None,
            shipping_details: None,
        }
    }
}
/// The outcome of the reported payment.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ReportPaymentPaymentRecordOutcome {
    Failed,
    Guaranteed,
}
impl ReportPaymentPaymentRecordOutcome {
    pub fn as_str(self) -> &'static str {
        use ReportPaymentPaymentRecordOutcome::*;
        match self {
            Failed => "failed",
            Guaranteed => "guaranteed",
        }
    }
}

impl std::str::FromStr for ReportPaymentPaymentRecordOutcome {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ReportPaymentPaymentRecordOutcome::*;
        match s {
            "failed" => Ok(Failed),
            "guaranteed" => Ok(Guaranteed),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ReportPaymentPaymentRecordOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ReportPaymentPaymentRecordOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ReportPaymentPaymentRecordOutcome {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ReportPaymentPaymentRecordOutcome {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for ReportPaymentPaymentRecordOutcome")
        })
    }
}
/// Information about the Payment Method debited for this payment.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ReportPaymentPaymentRecordPaymentMethodDetails {
    /// The billing details associated with the method of payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<BillingDetails>,
    /// Information about the custom (user-defined) payment method used to make this payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<Custom>,
    /// ID of the Stripe Payment Method used to make this payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    /// The type of the payment method details.
    /// An additional hash is included on the payment_method_details with a name matching this value.
    /// It contains additional information specific to the type.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<ReportPaymentPaymentRecordPaymentMethodDetailsType>,
}
impl ReportPaymentPaymentRecordPaymentMethodDetails {
    pub fn new() -> Self {
        Self { billing_details: None, custom: None, payment_method: None, type_: None }
    }
}
impl Default for ReportPaymentPaymentRecordPaymentMethodDetails {
    fn default() -> Self {
        Self::new()
    }
}
/// The type of the payment method details.
/// An additional hash is included on the payment_method_details with a name matching this value.
/// It contains additional information specific to the type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ReportPaymentPaymentRecordPaymentMethodDetailsType {
    Custom,
}
impl ReportPaymentPaymentRecordPaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        use ReportPaymentPaymentRecordPaymentMethodDetailsType::*;
        match self {
            Custom => "custom",
        }
    }
}

impl std::str::FromStr for ReportPaymentPaymentRecordPaymentMethodDetailsType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ReportPaymentPaymentRecordPaymentMethodDetailsType::*;
        match s {
            "custom" => Ok(Custom),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ReportPaymentPaymentRecordPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ReportPaymentPaymentRecordPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ReportPaymentPaymentRecordPaymentMethodDetailsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ReportPaymentPaymentRecordPaymentMethodDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for ReportPaymentPaymentRecordPaymentMethodDetailsType",
            )
        })
    }
}
/// Processor information for this payment.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ReportPaymentPaymentRecordProcessorDetails {
    /// Information about the custom processor used to make this payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<ReportPaymentPaymentRecordProcessorDetailsCustom>,
    /// The type of the processor details.
    /// An additional hash is included on processor_details with a name matching this value.
    /// It contains additional information specific to the processor.
    #[serde(rename = "type")]
    pub type_: ReportPaymentPaymentRecordProcessorDetailsType,
}
impl ReportPaymentPaymentRecordProcessorDetails {
    pub fn new(type_: impl Into<ReportPaymentPaymentRecordProcessorDetailsType>) -> Self {
        Self { custom: None, type_: type_.into() }
    }
}
/// Information about the custom processor used to make this payment.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ReportPaymentPaymentRecordProcessorDetailsCustom {
    /// An opaque string for manual reconciliation of this payment, for example a check number or a payment processor ID.
    pub payment_reference: String,
}
impl ReportPaymentPaymentRecordProcessorDetailsCustom {
    pub fn new(payment_reference: impl Into<String>) -> Self {
        Self { payment_reference: payment_reference.into() }
    }
}
/// The type of the processor details.
/// An additional hash is included on processor_details with a name matching this value.
/// It contains additional information specific to the processor.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ReportPaymentPaymentRecordProcessorDetailsType {
    Custom,
}
impl ReportPaymentPaymentRecordProcessorDetailsType {
    pub fn as_str(self) -> &'static str {
        use ReportPaymentPaymentRecordProcessorDetailsType::*;
        match self {
            Custom => "custom",
        }
    }
}

impl std::str::FromStr for ReportPaymentPaymentRecordProcessorDetailsType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ReportPaymentPaymentRecordProcessorDetailsType::*;
        match s {
            "custom" => Ok(Custom),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ReportPaymentPaymentRecordProcessorDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ReportPaymentPaymentRecordProcessorDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ReportPaymentPaymentRecordProcessorDetailsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ReportPaymentPaymentRecordProcessorDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for ReportPaymentPaymentRecordProcessorDetailsType",
            )
        })
    }
}
/// Report a new Payment Record. You may report a Payment Record as it is
///  initialized and later report updates through the other report_* methods, or report Payment
///  Records in a terminal state directly, through this method.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ReportPaymentPaymentRecord {
    inner: ReportPaymentPaymentRecordBuilder,
}
impl ReportPaymentPaymentRecord {
    /// Construct a new `ReportPaymentPaymentRecord`.
    pub fn new(
        amount_requested: impl Into<Amount>,
        initiated_at: impl Into<stripe_types::Timestamp>,
        payment_method_details: impl Into<ReportPaymentPaymentRecordPaymentMethodDetails>,
    ) -> Self {
        Self {
            inner: ReportPaymentPaymentRecordBuilder::new(
                amount_requested.into(),
                initiated_at.into(),
                payment_method_details.into(),
            ),
        }
    }
    /// Customer information for this payment.
    pub fn customer_details(mut self, customer_details: impl Into<CustomerDetails>) -> Self {
        self.inner.customer_details = Some(customer_details.into());
        self
    }
    /// Indicates whether the customer was present in your checkout flow during this payment.
    pub fn customer_presence(
        mut self,
        customer_presence: impl Into<stripe_shared::PaymentRecordCustomerPresence>,
    ) -> Self {
        self.inner.customer_presence = Some(customer_presence.into());
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
    /// Information about the payment attempt failure.
    pub fn failed(mut self, failed: impl Into<Failed>) -> Self {
        self.inner.failed = Some(failed.into());
        self
    }
    /// Information about the payment attempt guarantee.
    pub fn guaranteed(mut self, guaranteed: impl Into<Guaranteed>) -> Self {
        self.inner.guaranteed = Some(guaranteed.into());
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
    /// The outcome of the reported payment.
    pub fn outcome(mut self, outcome: impl Into<ReportPaymentPaymentRecordOutcome>) -> Self {
        self.inner.outcome = Some(outcome.into());
        self
    }
    /// Processor information for this payment.
    pub fn processor_details(
        mut self,
        processor_details: impl Into<ReportPaymentPaymentRecordProcessorDetails>,
    ) -> Self {
        self.inner.processor_details = Some(processor_details.into());
        self
    }
    /// Shipping information for this payment.
    pub fn shipping_details(mut self, shipping_details: impl Into<ShippingDetails>) -> Self {
        self.inner.shipping_details = Some(shipping_details.into());
        self
    }
}
impl ReportPaymentPaymentRecord {
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

impl StripeRequest for ReportPaymentPaymentRecord {
    type Output = stripe_shared::PaymentRecord;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/payment_records/report_payment").form(&self.inner)
    }
}

#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct Failed {
    /// When the reported payment failed. Measured in seconds since the Unix epoch.
    pub failed_at: stripe_types::Timestamp,
}
impl Failed {
    pub fn new(failed_at: impl Into<stripe_types::Timestamp>) -> Self {
        Self { failed_at: failed_at.into() }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct Guaranteed {
    /// When the reported payment was guaranteed. Measured in seconds since the Unix epoch.
    pub guaranteed_at: stripe_types::Timestamp,
}
impl Guaranteed {
    pub fn new(guaranteed_at: impl Into<stripe_types::Timestamp>) -> Self {
        Self { guaranteed_at: guaranteed_at.into() }
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct Address {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1, such as the street, PO Box, or company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2, such as the apartment, suite, unit, or building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl Address {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl Default for Address {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct Custom {
    /// Display name for the custom (user-defined) payment method type used to make this payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The custom payment method type associated with this payment.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Custom {
    pub fn new() -> Self {
        Self { display_name: None, type_: None }
    }
}
impl Default for Custom {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct CustomerDetails {
    /// The customer who made the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    /// The customer's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The customer's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The customer's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}
impl CustomerDetails {
    pub fn new() -> Self {
        Self { customer: None, email: None, name: None, phone: None }
    }
}
impl Default for CustomerDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct Amount {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// A positive integer representing the amount in the currency's [minor unit](https://stripe.com/docs/currencies#zero-decimal).
    /// For example, `100` can represent 1 USD or 100 JPY.
    pub value: i64,
}
impl Amount {
    pub fn new(currency: impl Into<stripe_types::Currency>, value: impl Into<i64>) -> Self {
        Self { currency: currency.into(), value: value.into() }
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct BillingDetails {
    /// The billing address associated with the method of payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    /// The billing email associated with the method of payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The billing name associated with the method of payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The billing phone number associated with the method of payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}
impl BillingDetails {
    pub fn new() -> Self {
        Self { address: None, email: None, name: None, phone: None }
    }
}
impl Default for BillingDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct ShippingDetails {
    /// The physical shipping address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    /// The shipping recipient's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The shipping recipient's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}
impl ShippingDetails {
    pub fn new() -> Self {
        Self { address: None, name: None, phone: None }
    }
}
impl Default for ShippingDetails {
    fn default() -> Self {
        Self::new()
    }
}
