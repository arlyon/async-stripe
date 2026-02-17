use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct CreateRadarPaymentEvaluationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_device_metadata_details: Option<CreateRadarPaymentEvaluationClientDeviceMetadataDetails>,
    customer_details: CreateRadarPaymentEvaluationCustomerDetails,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    payment_details: CreateRadarPaymentEvaluationPaymentDetails,
}
impl CreateRadarPaymentEvaluationBuilder {
    fn new(
        customer_details: impl Into<CreateRadarPaymentEvaluationCustomerDetails>,
        payment_details: impl Into<CreateRadarPaymentEvaluationPaymentDetails>,
    ) -> Self {
        Self {
            client_device_metadata_details: None,
            customer_details: customer_details.into(),
            expand: None,
            metadata: None,
            payment_details: payment_details.into(),
        }
    }
}
/// Details about the Client Device Metadata to associate with the payment evaluation.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateRadarPaymentEvaluationClientDeviceMetadataDetails {
    /// ID for the Radar Session to associate with the payment evaluation.
    /// A [Radar Session](https://docs.stripe.com/radar/radar-session) is a snapshot of the browser metadata and device details that help Radar make more accurate predictions on your payments.
    pub radar_session: String,
}
impl CreateRadarPaymentEvaluationClientDeviceMetadataDetails {
    pub fn new(radar_session: impl Into<String>) -> Self {
        Self { radar_session: radar_session.into() }
    }
}
/// Details about the customer associated with the payment evaluation.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateRadarPaymentEvaluationCustomerDetails {
    /// The ID of the customer associated with the payment evaluation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    /// The ID of the Account representing the customer associated with the payment evaluation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_account: Option<String>,
    /// The customer's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The customer's full name or business name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The customer's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}
impl CreateRadarPaymentEvaluationCustomerDetails {
    pub fn new() -> Self {
        Self { customer: None, customer_account: None, email: None, name: None, phone: None }
    }
}
impl Default for CreateRadarPaymentEvaluationCustomerDetails {
    fn default() -> Self {
        Self::new()
    }
}
/// Details about the payment.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateRadarPaymentEvaluationPaymentDetails {
    /// The intended amount to collect with this payment.
    /// A positive integer representing how much to charge in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal) (for example, 100 cents to charge 1.00 USD or 100 to charge 100 Yen, a zero-decimal currency).
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Details about the payment's customer presence and type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub money_movement_details:
        Option<CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetails>,
    /// Details about the payment method to use for the payment.
    pub payment_method_details: CreateRadarPaymentEvaluationPaymentDetailsPaymentMethodDetails,
    /// Shipping details for the payment evaluation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_details: Option<CreateRadarPaymentEvaluationPaymentDetailsShippingDetails>,
    /// Payment statement descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}
impl CreateRadarPaymentEvaluationPaymentDetails {
    pub fn new(
        amount: impl Into<i64>,
        currency: impl Into<stripe_types::Currency>,
        payment_method_details: impl Into<
            CreateRadarPaymentEvaluationPaymentDetailsPaymentMethodDetails,
        >,
    ) -> Self {
        Self {
            amount: amount.into(),
            currency: currency.into(),
            description: None,
            money_movement_details: None,
            payment_method_details: payment_method_details.into(),
            shipping_details: None,
            statement_descriptor: None,
        }
    }
}
/// Details about the payment's customer presence and type.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetails {
    /// Describes card money movement details for the payment evaluation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsCard>,
    /// Describes the type of money movement. Currently only `card` is supported.
    pub money_movement_type:
        CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsMoneyMovementType,
}
impl CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetails {
    pub fn new(
        money_movement_type: impl Into<
            CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsMoneyMovementType,
        >,
    ) -> Self {
        Self { card: None, money_movement_type: money_movement_type.into() }
    }
}
/// Describes card money movement details for the payment evaluation.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsCard {
    /// Describes the presence of the customer during the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_presence:
        Option<CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsCardCustomerPresence>,
    /// Describes the type of payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type:
        Option<CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsCardPaymentType>,
}
impl CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsCard {
    pub fn new() -> Self {
        Self { customer_presence: None, payment_type: None }
    }
}
impl Default for CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsCard {
    fn default() -> Self {
        Self::new()
    }
}
/// Describes the presence of the customer during the payment.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsCardCustomerPresence {
    OffSession,
    OnSession,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsCardCustomerPresence {
    pub fn as_str(&self) -> &str {
        use CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsCardCustomerPresence::*;
        match self {
            OffSession => "off_session",
            OnSession => "on_session",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsCardCustomerPresence
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsCardCustomerPresence::*;
        match s {
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsCardCustomerPresence"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display
    for CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsCardCustomerPresence
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsCardCustomerPresence
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsCardCustomerPresence
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsCardCustomerPresence
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Describes the type of payment.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsCardPaymentType {
    OneOff,
    Recurring,
    SetupOneOff,
    SetupRecurring,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsCardPaymentType {
    pub fn as_str(&self) -> &str {
        use CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsCardPaymentType::*;
        match self {
            OneOff => "one_off",
            Recurring => "recurring",
            SetupOneOff => "setup_one_off",
            SetupRecurring => "setup_recurring",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsCardPaymentType
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsCardPaymentType::*;
        match s {
            "one_off" => Ok(OneOff),
            "recurring" => Ok(Recurring),
            "setup_one_off" => Ok(SetupOneOff),
            "setup_recurring" => Ok(SetupRecurring),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsCardPaymentType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display
    for CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsCardPaymentType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsCardPaymentType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsCardPaymentType
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsCardPaymentType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Describes the type of money movement. Currently only `card` is supported.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsMoneyMovementType {
    Card,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsMoneyMovementType {
    pub fn as_str(&self) -> &str {
        use CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsMoneyMovementType::*;
        match self {
            Card => "card",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsMoneyMovementType
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsMoneyMovementType::*;
        match s {
            "card" => Ok(Card),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsMoneyMovementType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display
    for CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsMoneyMovementType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsMoneyMovementType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsMoneyMovementType
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateRadarPaymentEvaluationPaymentDetailsMoneyMovementDetailsMoneyMovementType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Details about the payment method to use for the payment.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateRadarPaymentEvaluationPaymentDetailsPaymentMethodDetails {
    /// Billing information associated with the payment evaluation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details:
        Option<CreateRadarPaymentEvaluationPaymentDetailsPaymentMethodDetailsBillingDetails>,
    /// ID of the payment method used in this payment evaluation.
    pub payment_method: String,
}
impl CreateRadarPaymentEvaluationPaymentDetailsPaymentMethodDetails {
    pub fn new(payment_method: impl Into<String>) -> Self {
        Self { billing_details: None, payment_method: payment_method.into() }
    }
}
/// Billing information associated with the payment evaluation.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateRadarPaymentEvaluationPaymentDetailsPaymentMethodDetailsBillingDetails {
    /// Billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Billing phone number (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}
impl CreateRadarPaymentEvaluationPaymentDetailsPaymentMethodDetailsBillingDetails {
    pub fn new() -> Self {
        Self { address: None, email: None, name: None, phone: None }
    }
}
impl Default for CreateRadarPaymentEvaluationPaymentDetailsPaymentMethodDetailsBillingDetails {
    fn default() -> Self {
        Self::new()
    }
}
/// Shipping details for the payment evaluation.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateRadarPaymentEvaluationPaymentDetailsShippingDetails {
    /// Shipping address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    /// Shipping name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Shipping phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}
impl CreateRadarPaymentEvaluationPaymentDetailsShippingDetails {
    pub fn new() -> Self {
        Self { address: None, name: None, phone: None }
    }
}
impl Default for CreateRadarPaymentEvaluationPaymentDetailsShippingDetails {
    fn default() -> Self {
        Self::new()
    }
}
/// Request a Radar API fraud risk score from Stripe for a payment before sending it for external processor authorization.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateRadarPaymentEvaluation {
    inner: CreateRadarPaymentEvaluationBuilder,
}
impl CreateRadarPaymentEvaluation {
    /// Construct a new `CreateRadarPaymentEvaluation`.
    pub fn new(
        customer_details: impl Into<CreateRadarPaymentEvaluationCustomerDetails>,
        payment_details: impl Into<CreateRadarPaymentEvaluationPaymentDetails>,
    ) -> Self {
        Self {
            inner: CreateRadarPaymentEvaluationBuilder::new(
                customer_details.into(),
                payment_details.into(),
            ),
        }
    }
    /// Details about the Client Device Metadata to associate with the payment evaluation.
    pub fn client_device_metadata_details(
        mut self,
        client_device_metadata_details: impl Into<
            CreateRadarPaymentEvaluationClientDeviceMetadataDetails,
        >,
    ) -> Self {
        self.inner.client_device_metadata_details = Some(client_device_metadata_details.into());
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
impl CreateRadarPaymentEvaluation {
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

impl StripeRequest for CreateRadarPaymentEvaluation {
    type Output = stripe_fraud::RadarPaymentEvaluation;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/radar/payment_evaluations").form(&self.inner)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
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
    /// State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
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
