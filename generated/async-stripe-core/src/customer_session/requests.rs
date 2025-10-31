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
    /// Configuration for the customer sheet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_sheet: Option<CreateCustomerSessionComponentsCustomerSheet>,
    /// Configuration for the mobile payment element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_payment_element: Option<CreateCustomerSessionComponentsMobilePaymentElement>,
    /// Configuration for the Payment Element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_element: Option<CreateCustomerSessionComponentsPaymentElement>,
    /// Configuration for the pricing table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_table: Option<CreateCustomerSessionComponentsPricingTable>,
}
impl CreateCustomerSessionComponents {
    pub fn new() -> Self {
        Self {
            buy_button: None,
            customer_sheet: None,
            mobile_payment_element: None,
            payment_element: None,
            pricing_table: None,
        }
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
/// Configuration for the customer sheet.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCustomerSessionComponentsCustomerSheet {
    /// Whether the customer sheet is enabled.
    pub enabled: bool,
    /// This hash defines whether the customer sheet supports certain features.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateCustomerSessionComponentsCustomerSheetFeatures>,
}
impl CreateCustomerSessionComponentsCustomerSheet {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), features: None }
    }
}
/// This hash defines whether the customer sheet supports certain features.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCustomerSessionComponentsCustomerSheetFeatures {
    /// A list of [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) values that controls which saved payment methods the customer sheet displays by filtering to only show payment methods with an `allow_redisplay` value that is present in this list.
    ///
    /// If not specified, defaults to ["always"].
    /// In order to display all saved payment methods, specify ["always", "limited", "unspecified"].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_allow_redisplay_filters: Option<
        Vec<CreateCustomerSessionComponentsCustomerSheetFeaturesPaymentMethodAllowRedisplayFilters>,
    >,
    /// Controls whether the customer sheet displays the option to remove a saved payment method."
    ///
    /// Allowing buyers to remove their saved payment methods impacts subscriptions that depend on that payment method.
    /// Removing the payment method detaches the [`customer` object](https://docs.stripe.com/api/payment_methods/object#payment_method_object-customer) from that [PaymentMethod](https://docs.stripe.com/api/payment_methods).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_remove:
        Option<CreateCustomerSessionComponentsCustomerSheetFeaturesPaymentMethodRemove>,
}
impl CreateCustomerSessionComponentsCustomerSheetFeatures {
    pub fn new() -> Self {
        Self { payment_method_allow_redisplay_filters: None, payment_method_remove: None }
    }
}
impl Default for CreateCustomerSessionComponentsCustomerSheetFeatures {
    fn default() -> Self {
        Self::new()
    }
}
/// A list of [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) values that controls which saved payment methods the customer sheet displays by filtering to only show payment methods with an `allow_redisplay` value that is present in this list.
///
/// If not specified, defaults to ["always"].
/// In order to display all saved payment methods, specify ["always", "limited", "unspecified"].
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCustomerSessionComponentsCustomerSheetFeaturesPaymentMethodAllowRedisplayFilters {
    Always,
    Limited,
    Unspecified,
}
impl CreateCustomerSessionComponentsCustomerSheetFeaturesPaymentMethodAllowRedisplayFilters {
    pub fn as_str(self) -> &'static str {
        use CreateCustomerSessionComponentsCustomerSheetFeaturesPaymentMethodAllowRedisplayFilters::*;
        match self {
            Always => "always",
            Limited => "limited",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr
    for CreateCustomerSessionComponentsCustomerSheetFeaturesPaymentMethodAllowRedisplayFilters
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCustomerSessionComponentsCustomerSheetFeaturesPaymentMethodAllowRedisplayFilters::*;
        match s {
            "always" => Ok(Always),
            "limited" => Ok(Limited),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateCustomerSessionComponentsCustomerSheetFeaturesPaymentMethodAllowRedisplayFilters
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateCustomerSessionComponentsCustomerSheetFeaturesPaymentMethodAllowRedisplayFilters
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateCustomerSessionComponentsCustomerSheetFeaturesPaymentMethodAllowRedisplayFilters
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
    for CreateCustomerSessionComponentsCustomerSheetFeaturesPaymentMethodAllowRedisplayFilters
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCustomerSessionComponentsCustomerSheetFeaturesPaymentMethodAllowRedisplayFilters"))
    }
}
/// Controls whether the customer sheet displays the option to remove a saved payment method."
///
/// Allowing buyers to remove their saved payment methods impacts subscriptions that depend on that payment method.
/// Removing the payment method detaches the [`customer` object](https://docs.stripe.com/api/payment_methods/object#payment_method_object-customer) from that [PaymentMethod](https://docs.stripe.com/api/payment_methods).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCustomerSessionComponentsCustomerSheetFeaturesPaymentMethodRemove {
    Disabled,
    Enabled,
}
impl CreateCustomerSessionComponentsCustomerSheetFeaturesPaymentMethodRemove {
    pub fn as_str(self) -> &'static str {
        use CreateCustomerSessionComponentsCustomerSheetFeaturesPaymentMethodRemove::*;
        match self {
            Disabled => "disabled",
            Enabled => "enabled",
        }
    }
}

impl std::str::FromStr for CreateCustomerSessionComponentsCustomerSheetFeaturesPaymentMethodRemove {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCustomerSessionComponentsCustomerSheetFeaturesPaymentMethodRemove::*;
        match s {
            "disabled" => Ok(Disabled),
            "enabled" => Ok(Enabled),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateCustomerSessionComponentsCustomerSheetFeaturesPaymentMethodRemove {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCustomerSessionComponentsCustomerSheetFeaturesPaymentMethodRemove {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCustomerSessionComponentsCustomerSheetFeaturesPaymentMethodRemove {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCustomerSessionComponentsCustomerSheetFeaturesPaymentMethodRemove
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCustomerSessionComponentsCustomerSheetFeaturesPaymentMethodRemove"))
    }
}
/// Configuration for the mobile payment element.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCustomerSessionComponentsMobilePaymentElement {
    /// Whether the mobile payment element is enabled.
    pub enabled: bool,
    /// This hash defines whether the mobile payment element supports certain features.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateCustomerSessionComponentsMobilePaymentElementFeatures>,
}
impl CreateCustomerSessionComponentsMobilePaymentElement {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), features: None }
    }
}
/// This hash defines whether the mobile payment element supports certain features.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCustomerSessionComponentsMobilePaymentElementFeatures {
        /// A list of [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) values that controls which saved payment methods the mobile payment element displays by filtering to only show payment methods with an `allow_redisplay` value that is present in this list.
    ///
        /// If not specified, defaults to ["always"].
    /// In order to display all saved payment methods, specify ["always", "limited", "unspecified"].
#[serde(skip_serializing_if = "Option::is_none")]
pub payment_method_allow_redisplay_filters: Option<Vec<CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodAllowRedisplayFilters>>,
    /// Controls whether or not the mobile payment element shows saved payment methods.
#[serde(skip_serializing_if = "Option::is_none")]
pub payment_method_redisplay: Option<CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodRedisplay>,
    /// Controls whether the mobile payment element displays the option to remove a saved payment method."
    ///
        /// Allowing buyers to remove their saved payment methods impacts subscriptions that depend on that payment method.
    /// Removing the payment method detaches the [`customer` object](https://docs.stripe.com/api/payment_methods/object#payment_method_object-customer) from that [PaymentMethod](https://docs.stripe.com/api/payment_methods).
#[serde(skip_serializing_if = "Option::is_none")]
pub payment_method_remove: Option<CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodRemove>,
        /// Controls whether the mobile payment element displays a checkbox offering to save a new payment method.
    ///
        /// If a customer checks the box, the [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) value on the PaymentMethod is set to `'always'` at confirmation time.
    /// For PaymentIntents, the [`setup_future_usage`](https://docs.stripe.com/api/payment_intents/object#payment_intent_object-setup_future_usage) value is also set to the value defined in `payment_method_save_usage`.
#[serde(skip_serializing_if = "Option::is_none")]
pub payment_method_save: Option<CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodSave>,
        /// Allows overriding the value of allow_override when saving a new payment method when payment_method_save is set to disabled.
    /// Use values: "always", "limited", or "unspecified".
    ///
    /// If not specified, defaults to `nil` (no override value).
#[serde(skip_serializing_if = "Option::is_none")]
pub payment_method_save_allow_redisplay_override: Option<CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodSaveAllowRedisplayOverride>,

}
impl CreateCustomerSessionComponentsMobilePaymentElementFeatures {
    pub fn new() -> Self {
        Self {
            payment_method_allow_redisplay_filters: None,
            payment_method_redisplay: None,
            payment_method_remove: None,
            payment_method_save: None,
            payment_method_save_allow_redisplay_override: None,
        }
    }
}
impl Default for CreateCustomerSessionComponentsMobilePaymentElementFeatures {
    fn default() -> Self {
        Self::new()
    }
}
/// A list of [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) values that controls which saved payment methods the mobile payment element displays by filtering to only show payment methods with an `allow_redisplay` value that is present in this list.
///
/// If not specified, defaults to ["always"].
/// In order to display all saved payment methods, specify ["always", "limited", "unspecified"].
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodAllowRedisplayFilters
{
    Always,
    Limited,
    Unspecified,
}
impl CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodAllowRedisplayFilters {
    pub fn as_str(self) -> &'static str {
        use CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodAllowRedisplayFilters::*;
        match self {
            Always => "always",
            Limited => "limited",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodAllowRedisplayFilters {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodAllowRedisplayFilters::*;
        match s {
    "always" => Ok(Always),
"limited" => Ok(Limited),
"unspecified" => Ok(Unspecified),
_ => Err(stripe_types::StripeParseError)

        }
    }
}
impl std::fmt::Display for CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodAllowRedisplayFilters {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodAllowRedisplayFilters {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodAllowRedisplayFilters {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodAllowRedisplayFilters {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodAllowRedisplayFilters"))
    }
}
/// Controls whether or not the mobile payment element shows saved payment methods.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodRedisplay {
    Disabled,
    Enabled,
}
impl CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodRedisplay {
    pub fn as_str(self) -> &'static str {
        use CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodRedisplay::*;
        match self {
            Disabled => "disabled",
            Enabled => "enabled",
        }
    }
}

impl std::str::FromStr
    for CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodRedisplay
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodRedisplay::*;
        match s {
            "disabled" => Ok(Disabled),
            "enabled" => Ok(Enabled),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodRedisplay
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodRedisplay
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodRedisplay
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
    for CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodRedisplay
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodRedisplay"))
    }
}
/// Controls whether the mobile payment element displays the option to remove a saved payment method."
///
/// Allowing buyers to remove their saved payment methods impacts subscriptions that depend on that payment method.
/// Removing the payment method detaches the [`customer` object](https://docs.stripe.com/api/payment_methods/object#payment_method_object-customer) from that [PaymentMethod](https://docs.stripe.com/api/payment_methods).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodRemove {
    Disabled,
    Enabled,
}
impl CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodRemove {
    pub fn as_str(self) -> &'static str {
        use CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodRemove::*;
        match self {
            Disabled => "disabled",
            Enabled => "enabled",
        }
    }
}

impl std::str::FromStr
    for CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodRemove
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodRemove::*;
        match s {
            "disabled" => Ok(Disabled),
            "enabled" => Ok(Enabled),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodRemove
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodRemove
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodRemove
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
    for CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodRemove
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodRemove"))
    }
}
/// Controls whether the mobile payment element displays a checkbox offering to save a new payment method.
///
/// If a customer checks the box, the [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) value on the PaymentMethod is set to `'always'` at confirmation time.
/// For PaymentIntents, the [`setup_future_usage`](https://docs.stripe.com/api/payment_intents/object#payment_intent_object-setup_future_usage) value is also set to the value defined in `payment_method_save_usage`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodSave {
    Disabled,
    Enabled,
}
impl CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodSave {
    pub fn as_str(self) -> &'static str {
        use CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodSave::*;
        match self {
            Disabled => "disabled",
            Enabled => "enabled",
        }
    }
}

impl std::str::FromStr
    for CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodSave
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodSave::*;
        match s {
            "disabled" => Ok(Disabled),
            "enabled" => Ok(Enabled),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodSave
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodSave
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodSave
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
    for CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodSave
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodSave"))
    }
}
/// Allows overriding the value of allow_override when saving a new payment method when payment_method_save is set to disabled.
/// Use values: "always", "limited", or "unspecified".
///
/// If not specified, defaults to `nil` (no override value).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodSaveAllowRedisplayOverride
{
    Always,
    Limited,
    Unspecified,
}
impl CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodSaveAllowRedisplayOverride {
    pub fn as_str(self) -> &'static str {
        use CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodSaveAllowRedisplayOverride::*;
        match self {
Always => "always",
Limited => "limited",
Unspecified => "unspecified",

        }
    }
}

impl std::str::FromStr for CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodSaveAllowRedisplayOverride {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodSaveAllowRedisplayOverride::*;
        match s {
    "always" => Ok(Always),
"limited" => Ok(Limited),
"unspecified" => Ok(Unspecified),
_ => Err(stripe_types::StripeParseError)

        }
    }
}
impl std::fmt::Display for CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodSaveAllowRedisplayOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodSaveAllowRedisplayOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodSaveAllowRedisplayOverride {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodSaveAllowRedisplayOverride {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodSaveAllowRedisplayOverride"))
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
    /// The maximum redisplay limit is `10`.
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
