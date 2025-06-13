// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::{CustomerId};
use crate::params::{Expand, Expandable, Object, Timestamp};
use crate::resources::{Customer};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "CustomerSessionResourceCustomerSession".
///
/// For more details see <https://stripe.com/docs/api/customer_sessions/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CustomerSession {

    /// The client secret of this Customer Session.
    ///
    /// Used on the client to set up secure access to the given `customer`.  The client secret can be used to provide access to `customer` from your frontend.
    /// It should not be stored, logged, or exposed to anyone other than the relevant customer.
    /// Make sure that you have TLS enabled on any page that includes the client secret.
    pub client_secret: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<CustomerSessionResourceComponents>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The Customer the Customer Session was created for.
    pub customer: Expandable<Customer>,

    /// The timestamp at which this Customer Session will expire.
    pub expires_at: Timestamp,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
}

impl CustomerSession {

    /// Creates a Customer Session object that includes a single-use client secret that you can use on your front-end to grant client-side API access for certain customer resources.
    pub fn create(client: &Client, params: CreateCustomerSession<'_>) -> Response<CustomerSession> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form("/customer_sessions", &params)
    }
}

impl Object for CustomerSession {
    type Id = ();
    fn id(&self) -> Self::Id {}
    fn object(&self) -> &'static str {
        "customer_session"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CustomerSessionResourceComponents {

    pub buy_button: CustomerSessionResourceComponentsResourceBuyButton,

    pub payment_element: CustomerSessionResourceComponentsResourcePaymentElement,

    pub pricing_table: CustomerSessionResourceComponentsResourcePricingTable,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CustomerSessionResourceComponentsResourceBuyButton {

    /// Whether the buy button is enabled.
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CustomerSessionResourceComponentsResourcePaymentElement {

    /// Whether the Payment Element is enabled.
    pub enabled: bool,

    /// This hash defines whether the Payment Element supports certain features.
    pub features: Option<CustomerSessionResourceComponentsResourcePaymentElementResourceFeatures>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CustomerSessionResourceComponentsResourcePaymentElementResourceFeatures {

    /// A list of [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) values that controls which saved payment methods the Payment Element displays by filtering to only show payment methods with an `allow_redisplay` value that is present in this list.
    ///
    /// If not specified, defaults to ["always"].
    ///
    /// In order to display all saved payment methods, specify ["always", "limited", "unspecified"].
    pub payment_method_allow_redisplay_filters: Vec<CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters>,

    /// Controls whether or not the Payment Element shows saved payment methods.
    ///
    /// This parameter defaults to `disabled`.
    pub payment_method_redisplay: CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRedisplay,

    /// Determines the max number of saved payment methods for the Payment Element to display.
    ///
    /// This parameter defaults to `3`.
    pub payment_method_redisplay_limit: Option<i64>,

    /// Controls whether the Payment Element displays the option to remove a saved payment method.
    ///
    /// This parameter defaults to `disabled`.  Allowing buyers to remove their saved payment methods impacts subscriptions that depend on that payment method.
    /// Removing the payment method detaches the [`customer` object](https://docs.stripe.com/api/payment_methods/object#payment_method_object-customer) from that [PaymentMethod](https://docs.stripe.com/api/payment_methods).
    pub payment_method_remove: CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRemove,

    /// Controls whether the Payment Element displays a checkbox offering to save a new payment method.
    ///
    /// This parameter defaults to `disabled`.  If a customer checks the box, the [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) value on the PaymentMethod is set to `'always'` at confirmation time.
    /// For PaymentIntents, the [`setup_future_usage`](https://docs.stripe.com/api/payment_intents/object#payment_intent_object-setup_future_usage) value is also set to the value defined in `payment_method_save_usage`.
    pub payment_method_save: CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSave,

    /// When using PaymentIntents and the customer checks the save checkbox, this field determines the [`setup_future_usage`](https://docs.stripe.com/api/payment_intents/object#payment_intent_object-setup_future_usage) value used to confirm the PaymentIntent.
    ///
    /// When using SetupIntents, directly configure the [`usage`](https://docs.stripe.com/api/setup_intents/object#setup_intent_object-usage) value on SetupIntent creation.
    pub payment_method_save_usage: Option<CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CustomerSessionResourceComponentsResourcePricingTable {

    /// Whether the pricing table is enabled.
    pub enabled: bool,
}

/// The parameters for `CustomerSession::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreateCustomerSession<'a> {

    /// Configuration for each component.
    ///
    /// Exactly 1 component must be enabled.
    pub components: CreateCustomerSessionComponents,

    /// The ID of an existing customer for which to create the Customer Session.
    pub customer: CustomerId,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],
}

impl<'a> CreateCustomerSession<'a> {
    pub fn new(components: CreateCustomerSessionComponents, customer: CustomerId) -> Self {
        CreateCustomerSession {
            components,
            customer,
            expand: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCustomerSessionComponentsBuyButton {

    /// Whether the buy button is enabled.
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCustomerSessionComponentsPaymentElement {

    /// Whether the Payment Element is enabled.
    pub enabled: bool,

    /// This hash defines whether the Payment Element supports certain features.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateCustomerSessionComponentsPaymentElementFeatures>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCustomerSessionComponentsPricingTable {

    /// Whether the pricing table is enabled.
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCustomerSessionComponentsPaymentElementFeatures {

    /// A list of [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) values that controls which saved payment methods the Payment Element displays by filtering to only show payment methods with an `allow_redisplay` value that is present in this list.
    ///
    /// If not specified, defaults to ["always"].
    ///
    /// In order to display all saved payment methods, specify ["always", "limited", "unspecified"].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_allow_redisplay_filters: Option<Vec<CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodAllowRedisplayFilters>>,

    /// Controls whether or not the Payment Element shows saved payment methods.
    ///
    /// This parameter defaults to `disabled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_redisplay: Option<CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRedisplay>,

    /// Determines the max number of saved payment methods for the Payment Element to display.
    ///
    /// This parameter defaults to `3`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_redisplay_limit: Option<i64>,

    /// Controls whether the Payment Element displays the option to remove a saved payment method.
    ///
    /// This parameter defaults to `disabled`.  Allowing buyers to remove their saved payment methods impacts subscriptions that depend on that payment method.
    /// Removing the payment method detaches the [`customer` object](https://docs.stripe.com/api/payment_methods/object#payment_method_object-customer) from that [PaymentMethod](https://docs.stripe.com/api/payment_methods).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_remove: Option<CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRemove>,

    /// Controls whether the Payment Element displays a checkbox offering to save a new payment method.
    ///
    /// This parameter defaults to `disabled`.  If a customer checks the box, the [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) value on the PaymentMethod is set to `'always'` at confirmation time.
    /// For PaymentIntents, the [`setup_future_usage`](https://docs.stripe.com/api/payment_intents/object#payment_intent_object-setup_future_usage) value is also set to the value defined in `payment_method_save_usage`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_save: Option<CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSave>,

    /// When using PaymentIntents and the customer checks the save checkbox, this field determines the [`setup_future_usage`](https://docs.stripe.com/api/payment_intents/object#payment_intent_object-setup_future_usage) value used to confirm the PaymentIntent.
    ///
    /// When using SetupIntents, directly configure the [`usage`](https://docs.stripe.com/api/setup_intents/object#setup_intent_object-usage) value on SetupIntent creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_save_usage: Option<CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSaveUsage>,
}

/// An enum representing the possible values of an `CreateCustomerSessionComponentsPaymentElementFeatures`'s `payment_method_allow_redisplay_filters` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodAllowRedisplayFilters {
    Always,
    Limited,
    Unspecified,
}

impl CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodAllowRedisplayFilters {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodAllowRedisplayFilters::Always => "always",
            CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodAllowRedisplayFilters::Limited => "limited",
            CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodAllowRedisplayFilters::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodAllowRedisplayFilters {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodAllowRedisplayFilters {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodAllowRedisplayFilters {
    fn default() -> Self {
        Self::Always
    }
}

/// An enum representing the possible values of an `CreateCustomerSessionComponentsPaymentElementFeatures`'s `payment_method_redisplay` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRedisplay {
    Disabled,
    Enabled,
}

impl CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRedisplay {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRedisplay::Disabled => "disabled",
            CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRedisplay::Enabled => "enabled",
        }
    }
}

impl AsRef<str> for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRedisplay {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRedisplay {
    fn default() -> Self {
        Self::Disabled
    }
}

/// An enum representing the possible values of an `CreateCustomerSessionComponentsPaymentElementFeatures`'s `payment_method_remove` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRemove {
    Disabled,
    Enabled,
}

impl CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRemove {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRemove::Disabled => "disabled",
            CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRemove::Enabled => "enabled",
        }
    }
}

impl AsRef<str> for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRemove {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRemove {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRemove {
    fn default() -> Self {
        Self::Disabled
    }
}

/// An enum representing the possible values of an `CreateCustomerSessionComponentsPaymentElementFeatures`'s `payment_method_save` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSave {
    Disabled,
    Enabled,
}

impl CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSave {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSave::Disabled => "disabled",
            CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSave::Enabled => "enabled",
        }
    }
}

impl AsRef<str> for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSave {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSave {
    fn default() -> Self {
        Self::Disabled
    }
}

/// An enum representing the possible values of an `CreateCustomerSessionComponentsPaymentElementFeatures`'s `payment_method_save_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSaveUsage {
    OffSession,
    OnSession,
}

impl CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSaveUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSaveUsage::OffSession => "off_session",
            CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSaveUsage::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSaveUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSaveUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSaveUsage {
    fn default() -> Self {
        Self::OffSession
    }
}

/// An enum representing the possible values of an `CustomerSessionResourceComponentsResourcePaymentElementResourceFeatures`'s `payment_method_allow_redisplay_filters` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters {
    Always,
    Limited,
    Unspecified,
}

impl CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters {
    pub fn as_str(self) -> &'static str {
        match self {
            CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters::Always => "always",
            CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters::Limited => "limited",
            CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters {
    fn default() -> Self {
        Self::Always
    }
}

/// An enum representing the possible values of an `CustomerSessionResourceComponentsResourcePaymentElementResourceFeatures`'s `payment_method_redisplay` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRedisplay {
    Disabled,
    Enabled,
}

impl CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRedisplay {
    pub fn as_str(self) -> &'static str {
        match self {
            CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRedisplay::Disabled => "disabled",
            CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRedisplay::Enabled => "enabled",
        }
    }
}

impl AsRef<str> for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRedisplay {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRedisplay {
    fn default() -> Self {
        Self::Disabled
    }
}

/// An enum representing the possible values of an `CustomerSessionResourceComponentsResourcePaymentElementResourceFeatures`'s `payment_method_remove` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRemove {
    Disabled,
    Enabled,
}

impl CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRemove {
    pub fn as_str(self) -> &'static str {
        match self {
            CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRemove::Disabled => "disabled",
            CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRemove::Enabled => "enabled",
        }
    }
}

impl AsRef<str> for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRemove {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRemove {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRemove {
    fn default() -> Self {
        Self::Disabled
    }
}

/// An enum representing the possible values of an `CustomerSessionResourceComponentsResourcePaymentElementResourceFeatures`'s `payment_method_save` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSave {
    Disabled,
    Enabled,
}

impl CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSave {
    pub fn as_str(self) -> &'static str {
        match self {
            CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSave::Disabled => "disabled",
            CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSave::Enabled => "enabled",
        }
    }
}

impl AsRef<str> for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSave {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSave {
    fn default() -> Self {
        Self::Disabled
    }
}

/// An enum representing the possible values of an `CustomerSessionResourceComponentsResourcePaymentElementResourceFeatures`'s `payment_method_save_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage {
    OffSession,
    OnSession,
}

impl CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage::OffSession => "off_session",
            CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage {
    fn default() -> Self {
        Self::OffSession
    }
}
