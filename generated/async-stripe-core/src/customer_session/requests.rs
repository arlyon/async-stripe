use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct CreateCustomerSessionBuilder {
    components: CreateCustomerSessionComponents,
    customer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl CreateCustomerSessionBuilder {
    fn new(
        components: impl Into<CreateCustomerSessionComponents>,
        customer: impl Into<String>,
    ) -> Self {
        Self { components: components.into(), customer: customer.into(), expand: None }
    }
}
/// Configuration for each component. Exactly 1 component must be enabled.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCustomerSessionComponents {
    /// Configuration for buy button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buy_button: Option<CreateCustomerSessionComponentsBuyButton>,
    /// Configuration for the Payment Element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_element: Option<CreateCustomerSessionComponentsPaymentElement>,
    /// Configuration for the pricing table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_table: Option<CreateCustomerSessionComponentsPricingTable>,
}
impl CreateCustomerSessionComponents {
    pub fn new() -> Self {
        Self { buy_button: None, payment_element: None, pricing_table: None }
    }
}
impl Default for CreateCustomerSessionComponents {
    fn default() -> Self {
        Self::new()
    }
}
/// Configuration for buy button.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCustomerSessionComponentsBuyButton {
    /// Whether the buy button is enabled.
    pub enabled: bool,
}
impl CreateCustomerSessionComponentsBuyButton {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into() }
    }
}
/// Configuration for the Payment Element.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCustomerSessionComponentsPaymentElement {
    /// Whether the Payment Element is enabled.
    pub enabled: bool,
    /// This hash defines whether the Payment Element supports certain features.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateCustomerSessionComponentsPaymentElementFeatures>,
}
impl CreateCustomerSessionComponentsPaymentElement {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), features: None }
    }
}
/// This hash defines whether the Payment Element supports certain features.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCustomerSessionComponentsPaymentElementFeatures {
    /// A list of [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) values that controls which saved payment methods the Payment Element displays by filtering to only show payment methods with an `allow_redisplay` value that is present in this list.
    ///
    /// If not specified, defaults to ["always"].
    /// In order to display all saved payment methods, specify ["always", "limited", "unspecified"].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_allow_redisplay_filters: Option<
        Vec<
            CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodAllowRedisplayFilters,
        >,
    >,
    /// Controls whether or not the Payment Element shows saved payment methods.
    /// This parameter defaults to `disabled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_redisplay:
        Option<CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRedisplay>,
    /// Determines the max number of saved payment methods for the Payment Element to display.
    /// This parameter defaults to `3`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_redisplay_limit: Option<i64>,
    /// Controls whether the Payment Element displays the option to remove a saved payment method.
    /// This parameter defaults to `disabled`.
    ///
    /// Allowing buyers to remove their saved payment methods impacts subscriptions that depend on that payment method.
    /// Removing the payment method detaches the [`customer` object](https://docs.stripe.com/api/payment_methods/object#payment_method_object-customer) from that [PaymentMethod](https://docs.stripe.com/api/payment_methods).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_remove:
        Option<CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRemove>,
    /// Controls whether the Payment Element displays a checkbox offering to save a new payment method.
    /// This parameter defaults to `disabled`.
    ///
    /// If a customer checks the box, the [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) value on the PaymentMethod is set to `'always'` at confirmation time.
    /// For PaymentIntents, the [`setup_future_usage`](https://docs.stripe.com/api/payment_intents/object#payment_intent_object-setup_future_usage) value is also set to the value defined in `payment_method_save_usage`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_save:
        Option<CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSave>,
    /// When using PaymentIntents and the customer checks the save checkbox, this field determines the [`setup_future_usage`](https://docs.stripe.com/api/payment_intents/object#payment_intent_object-setup_future_usage) value used to confirm the PaymentIntent.
    ///
    /// When using SetupIntents, directly configure the [`usage`](https://docs.stripe.com/api/setup_intents/object#setup_intent_object-usage) value on SetupIntent creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_save_usage:
        Option<CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSaveUsage>,
}
impl CreateCustomerSessionComponentsPaymentElementFeatures {
    pub fn new() -> Self {
        Self {
            payment_method_allow_redisplay_filters: None,
            payment_method_redisplay: None,
            payment_method_redisplay_limit: None,
            payment_method_remove: None,
            payment_method_save: None,
            payment_method_save_usage: None,
        }
    }
}
impl Default for CreateCustomerSessionComponentsPaymentElementFeatures {
    fn default() -> Self {
        Self::new()
    }
}
/// A list of [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) values that controls which saved payment methods the Payment Element displays by filtering to only show payment methods with an `allow_redisplay` value that is present in this list.
///
/// If not specified, defaults to ["always"].
/// In order to display all saved payment methods, specify ["always", "limited", "unspecified"].
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodAllowRedisplayFilters {
    Always,
    Limited,
    Unspecified,
}
impl CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodAllowRedisplayFilters {
    pub fn as_str(self) -> &'static str {
        use CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodAllowRedisplayFilters::*;
        match self {
            Always => "always",
            Limited => "limited",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr
    for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodAllowRedisplayFilters
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodAllowRedisplayFilters::*;
        match s {
            "always" => Ok(Always),
            "limited" => Ok(Limited),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodAllowRedisplayFilters
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodAllowRedisplayFilters
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodAllowRedisplayFilters
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
    for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodAllowRedisplayFilters
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodAllowRedisplayFilters"))
    }
}
/// Controls whether or not the Payment Element shows saved payment methods.
/// This parameter defaults to `disabled`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRedisplay {
    Disabled,
    Enabled,
}
impl CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRedisplay {
    pub fn as_str(self) -> &'static str {
        use CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRedisplay::*;
        match self {
            Disabled => "disabled",
            Enabled => "enabled",
        }
    }
}

impl std::str::FromStr
    for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRedisplay
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRedisplay::*;
        match s {
            "disabled" => Ok(Disabled),
            "enabled" => Ok(Enabled),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRedisplay
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRedisplay
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRedisplay
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
    for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRedisplay
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRedisplay"))
    }
}
/// Controls whether the Payment Element displays the option to remove a saved payment method.
/// This parameter defaults to `disabled`.
///
/// Allowing buyers to remove their saved payment methods impacts subscriptions that depend on that payment method.
/// Removing the payment method detaches the [`customer` object](https://docs.stripe.com/api/payment_methods/object#payment_method_object-customer) from that [PaymentMethod](https://docs.stripe.com/api/payment_methods).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRemove {
    Disabled,
    Enabled,
}
impl CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRemove {
    pub fn as_str(self) -> &'static str {
        use CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRemove::*;
        match self {
            Disabled => "disabled",
            Enabled => "enabled",
        }
    }
}

impl std::str::FromStr
    for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRemove
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRemove::*;
        match s {
            "disabled" => Ok(Disabled),
            "enabled" => Ok(Enabled),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRemove
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRemove {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRemove {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRemove
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRemove"))
    }
}
/// Controls whether the Payment Element displays a checkbox offering to save a new payment method.
/// This parameter defaults to `disabled`.
///
/// If a customer checks the box, the [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) value on the PaymentMethod is set to `'always'` at confirmation time.
/// For PaymentIntents, the [`setup_future_usage`](https://docs.stripe.com/api/payment_intents/object#payment_intent_object-setup_future_usage) value is also set to the value defined in `payment_method_save_usage`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSave {
    Disabled,
    Enabled,
}
impl CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSave {
    pub fn as_str(self) -> &'static str {
        use CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSave::*;
        match self {
            Disabled => "disabled",
            Enabled => "enabled",
        }
    }
}

impl std::str::FromStr for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSave {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSave::*;
        match s {
            "disabled" => Ok(Disabled),
            "enabled" => Ok(Enabled),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSave {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSave
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSave"))
    }
}
/// When using PaymentIntents and the customer checks the save checkbox, this field determines the [`setup_future_usage`](https://docs.stripe.com/api/payment_intents/object#payment_intent_object-setup_future_usage) value used to confirm the PaymentIntent.
///
/// When using SetupIntents, directly configure the [`usage`](https://docs.stripe.com/api/setup_intents/object#setup_intent_object-usage) value on SetupIntent creation.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSaveUsage {
    OffSession,
    OnSession,
}
impl CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSaveUsage {
    pub fn as_str(self) -> &'static str {
        use CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSaveUsage::*;
        match self {
            OffSession => "off_session",
            OnSession => "on_session",
        }
    }
}

impl std::str::FromStr
    for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSaveUsage
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSaveUsage::*;
        match s {
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSaveUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSaveUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSaveUsage
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
    for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSaveUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSaveUsage"))
    }
}
/// Configuration for the pricing table.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCustomerSessionComponentsPricingTable {
    /// Whether the pricing table is enabled.
    pub enabled: bool,
}
impl CreateCustomerSessionComponentsPricingTable {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into() }
    }
}
/// Creates a Customer Session object that includes a single-use client secret that you can use on your front-end to grant client-side API access for certain customer resources.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCustomerSession {
    inner: CreateCustomerSessionBuilder,
}
impl CreateCustomerSession {
    /// Construct a new `CreateCustomerSession`.
    pub fn new(
        components: impl Into<CreateCustomerSessionComponents>,
        customer: impl Into<String>,
    ) -> Self {
        Self { inner: CreateCustomerSessionBuilder::new(components.into(), customer.into()) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl CreateCustomerSession {
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

impl StripeRequest for CreateCustomerSession {
    type Output = stripe_core::CustomerSession;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/customer_sessions").form(&self.inner)
    }
}
