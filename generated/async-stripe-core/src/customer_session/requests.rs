use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CreateCustomerSessionBuilder {
    components: CreateCustomerSessionComponents,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCustomerSessionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCustomerSessionBuilder").finish_non_exhaustive()
    }
}
impl CreateCustomerSessionBuilder {
    fn new(components: impl Into<CreateCustomerSessionComponents>) -> Self {
        Self { components: components.into(), customer: None, customer_account: None, expand: None }
    }
}
/// Configuration for each component. At least 1 component must be enabled.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCustomerSessionComponents {
    /// Configuration for active entitlements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_entitlements: Option<CreateCustomerSessionComponentsActiveEntitlements>,
    /// Configuration for buy button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buy_button: Option<CreateCustomerSessionComponentsBuyButton>,
    /// Configuration for customer portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_portal: Option<CreateCustomerSessionComponentsCustomerPortal>,
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCustomerSessionComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCustomerSessionComponents").finish_non_exhaustive()
    }
}
impl CreateCustomerSessionComponents {
    pub fn new() -> Self {
        Self {
            active_entitlements: None,
            buy_button: None,
            customer_portal: None,
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
/// Configuration for active entitlements.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCustomerSessionComponentsActiveEntitlements {
    /// Whether the active entitlements is enabled.
    pub enabled: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCustomerSessionComponentsActiveEntitlements {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCustomerSessionComponentsActiveEntitlements").finish_non_exhaustive()
    }
}
impl CreateCustomerSessionComponentsActiveEntitlements {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into() }
    }
}
/// Configuration for buy button.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCustomerSessionComponentsBuyButton {
    /// Whether the buy button is enabled.
    pub enabled: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCustomerSessionComponentsBuyButton {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCustomerSessionComponentsBuyButton").finish_non_exhaustive()
    }
}
impl CreateCustomerSessionComponentsBuyButton {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into() }
    }
}
/// Configuration for customer portal.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCustomerSessionComponentsCustomerPortal {
    /// Whether the customer portal is enabled.
    pub enabled: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCustomerSessionComponentsCustomerPortal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCustomerSessionComponentsCustomerPortal").finish_non_exhaustive()
    }
}
impl CreateCustomerSessionComponentsCustomerPortal {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into() }
    }
}
/// Configuration for the customer sheet.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCustomerSessionComponentsCustomerSheet {
    /// Whether the customer sheet is enabled.
    pub enabled: bool,
    /// This hash defines whether the customer sheet supports certain features.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateCustomerSessionComponentsCustomerSheetFeatures>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCustomerSessionComponentsCustomerSheet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCustomerSessionComponentsCustomerSheet").finish_non_exhaustive()
    }
}
impl CreateCustomerSessionComponentsCustomerSheet {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), features: None }
    }
}
/// This hash defines whether the customer sheet supports certain features.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCustomerSessionComponentsCustomerSheetFeatures {
    /// A list of [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) values that controls which saved payment methods the customer sheet displays by filtering to only show payment methods with an `allow_redisplay` value that is present in this list.
    ///
    /// If not specified, defaults to ["always"].
    /// In order to display all saved payment methods, specify ["always", "limited", "unspecified"].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_allow_redisplay_filters: Option<Vec<stripe_shared::CardAllowRedisplay>>,
    /// Controls whether the customer sheet displays the option to remove a saved payment method."
    ///
    /// Allowing buyers to remove their saved payment methods impacts subscriptions that depend on that payment method.
    /// Removing the payment method detaches the [`customer` object](https://docs.stripe.com/api/payment_methods/object#payment_method_object-customer) from that [PaymentMethod](https://docs.stripe.com/api/payment_methods).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_remove: Option<stripe_shared::BalanceSettingsResourcePayoutsStatus>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCustomerSessionComponentsCustomerSheetFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCustomerSessionComponentsCustomerSheetFeatures")
            .finish_non_exhaustive()
    }
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
/// Configuration for the mobile payment element.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCustomerSessionComponentsMobilePaymentElement {
    /// Whether the mobile payment element is enabled.
    pub enabled: bool,
    /// This hash defines whether the mobile payment element supports certain features.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateCustomerSessionComponentsMobilePaymentElementFeatures>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCustomerSessionComponentsMobilePaymentElement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCustomerSessionComponentsMobilePaymentElement")
            .finish_non_exhaustive()
    }
}
impl CreateCustomerSessionComponentsMobilePaymentElement {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), features: None }
    }
}
/// This hash defines whether the mobile payment element supports certain features.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCustomerSessionComponentsMobilePaymentElementFeatures {
    /// A list of [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) values that controls which saved payment methods the mobile payment element displays by filtering to only show payment methods with an `allow_redisplay` value that is present in this list.
    ///
    /// If not specified, defaults to ["always"].
    /// In order to display all saved payment methods, specify ["always", "limited", "unspecified"].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_allow_redisplay_filters: Option<Vec<stripe_shared::CardAllowRedisplay>>,
    /// Controls whether or not the mobile payment element shows saved payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_redisplay: Option<stripe_shared::BalanceSettingsResourcePayoutsStatus>,
    /// Controls whether the mobile payment element displays the option to remove a saved payment method."
    ///
    /// Allowing buyers to remove their saved payment methods impacts subscriptions that depend on that payment method.
    /// Removing the payment method detaches the [`customer` object](https://docs.stripe.com/api/payment_methods/object#payment_method_object-customer) from that [PaymentMethod](https://docs.stripe.com/api/payment_methods).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_remove: Option<stripe_shared::BalanceSettingsResourcePayoutsStatus>,
    /// Controls whether the mobile payment element displays a checkbox offering to save a new payment method.
    ///
    /// If a customer checks the box, the [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) value on the PaymentMethod is set to `'always'` at confirmation time.
    /// For PaymentIntents, the [`setup_future_usage`](https://docs.stripe.com/api/payment_intents/object#payment_intent_object-setup_future_usage) value is also set to the value defined in `payment_method_save_usage`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_save: Option<stripe_shared::BalanceSettingsResourcePayoutsStatus>,
    /// Allows overriding the value of allow_override when saving a new payment method when payment_method_save is set to disabled.
    /// Use values: "always", "limited", or "unspecified".
    ///
    /// If not specified, defaults to `nil` (no override value).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_save_allow_redisplay_override: Option<stripe_shared::CardAllowRedisplay>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCustomerSessionComponentsMobilePaymentElementFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCustomerSessionComponentsMobilePaymentElementFeatures")
            .finish_non_exhaustive()
    }
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
/// Configuration for the Payment Element.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCustomerSessionComponentsPaymentElement {
    /// Whether the Payment Element is enabled.
    pub enabled: bool,
    /// This hash defines whether the Payment Element supports certain features.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CreateCustomerSessionComponentsPaymentElementFeatures>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCustomerSessionComponentsPaymentElement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCustomerSessionComponentsPaymentElement").finish_non_exhaustive()
    }
}
impl CreateCustomerSessionComponentsPaymentElement {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), features: None }
    }
}
/// This hash defines whether the Payment Element supports certain features.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCustomerSessionComponentsPaymentElementFeatures {
    /// A list of [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) values that controls which saved payment methods the Payment Element displays by filtering to only show payment methods with an `allow_redisplay` value that is present in this list.
    ///
    /// If not specified, defaults to ["always"].
    /// In order to display all saved payment methods, specify ["always", "limited", "unspecified"].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_allow_redisplay_filters: Option<Vec<stripe_shared::CardAllowRedisplay>>,
    /// Controls whether or not the Payment Element shows saved payment methods.
    /// This parameter defaults to `disabled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_redisplay: Option<stripe_shared::BalanceSettingsResourcePayoutsStatus>,
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
    pub payment_method_remove: Option<stripe_shared::BalanceSettingsResourcePayoutsStatus>,
    /// Controls whether the Payment Element displays a checkbox offering to save a new payment method.
    /// This parameter defaults to `disabled`.
    ///
    /// If a customer checks the box, the [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) value on the PaymentMethod is set to `'always'` at confirmation time.
    /// For PaymentIntents, the [`setup_future_usage`](https://docs.stripe.com/api/payment_intents/object#payment_intent_object-setup_future_usage) value is also set to the value defined in `payment_method_save_usage`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_save: Option<stripe_shared::BalanceSettingsResourcePayoutsStatus>,
    /// When using PaymentIntents and the customer checks the save checkbox, this field determines the [`setup_future_usage`](https://docs.stripe.com/api/payment_intents/object#payment_intent_object-setup_future_usage) value used to confirm the PaymentIntent.
    ///
    /// When using SetupIntents, directly configure the [`usage`](https://docs.stripe.com/api/setup_intents/object#setup_intent_object-usage) value on SetupIntent creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_save_usage: Option<stripe_shared::CreateSetupIntentUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCustomerSessionComponentsPaymentElementFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCustomerSessionComponentsPaymentElementFeatures")
            .finish_non_exhaustive()
    }
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
/// Configuration for the pricing table.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCustomerSessionComponentsPricingTable {
    /// Whether the pricing table is enabled.
    pub enabled: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCustomerSessionComponentsPricingTable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCustomerSessionComponentsPricingTable").finish_non_exhaustive()
    }
}
impl CreateCustomerSessionComponentsPricingTable {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into() }
    }
}
/// Creates a Customer Session object that includes a single-use client secret that you can use on your front-end to grant client-side API access for certain customer resources.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateCustomerSession {
    inner: CreateCustomerSessionBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateCustomerSession {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateCustomerSession").finish_non_exhaustive()
    }
}
impl CreateCustomerSession {
    /// Construct a new `CreateCustomerSession`.
    pub fn new(components: impl Into<CreateCustomerSessionComponents>) -> Self {
        Self { inner: CreateCustomerSessionBuilder::new(components.into()) }
    }
    /// The ID of an existing customer for which to create the Customer Session.
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.inner.customer = Some(customer.into());
        self
    }
    /// The ID of an existing Account for which to create the Customer Session.
    pub fn customer_account(mut self, customer_account: impl Into<String>) -> Self {
        self.inner.customer_account = Some(customer_account.into());
        self
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

/// A list of [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) values that controls which saved payment methods the customer sheet displays by filtering to only show payment methods with an `allow_redisplay` value that is present in this list.
///
/// If not specified, defaults to ["always"].
/// In order to display all saved payment methods, specify ["always", "limited", "unspecified"].
pub type CreateCustomerSessionComponentsCustomerSheetFeaturesPaymentMethodAllowRedisplayFilters =
    stripe_shared::CardAllowRedisplay;
/// Controls whether the customer sheet displays the option to remove a saved payment method."
///
/// Allowing buyers to remove their saved payment methods impacts subscriptions that depend on that payment method.
/// Removing the payment method detaches the [`customer` object](https://docs.stripe.com/api/payment_methods/object#payment_method_object-customer) from that [PaymentMethod](https://docs.stripe.com/api/payment_methods).
pub type CreateCustomerSessionComponentsCustomerSheetFeaturesPaymentMethodRemove =
    stripe_shared::BalanceSettingsResourcePayoutsStatus;
/// A list of [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) values that controls which saved payment methods the mobile payment element displays by filtering to only show payment methods with an `allow_redisplay` value that is present in this list.
///
/// If not specified, defaults to ["always"].
/// In order to display all saved payment methods, specify ["always", "limited", "unspecified"].
pub type CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodAllowRedisplayFilters =
    stripe_shared::CardAllowRedisplay;
/// Controls whether or not the mobile payment element shows saved payment methods.
pub type CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodRedisplay =
    stripe_shared::BalanceSettingsResourcePayoutsStatus;
/// Controls whether the mobile payment element displays the option to remove a saved payment method."
///
/// Allowing buyers to remove their saved payment methods impacts subscriptions that depend on that payment method.
/// Removing the payment method detaches the [`customer` object](https://docs.stripe.com/api/payment_methods/object#payment_method_object-customer) from that [PaymentMethod](https://docs.stripe.com/api/payment_methods).
pub type CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodRemove =
    stripe_shared::BalanceSettingsResourcePayoutsStatus;
/// Controls whether the mobile payment element displays a checkbox offering to save a new payment method.
///
/// If a customer checks the box, the [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) value on the PaymentMethod is set to `'always'` at confirmation time.
/// For PaymentIntents, the [`setup_future_usage`](https://docs.stripe.com/api/payment_intents/object#payment_intent_object-setup_future_usage) value is also set to the value defined in `payment_method_save_usage`.
pub type CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodSave =
    stripe_shared::BalanceSettingsResourcePayoutsStatus;
/// Allows overriding the value of allow_override when saving a new payment method when payment_method_save is set to disabled.
/// Use values: "always", "limited", or "unspecified".
///
/// If not specified, defaults to `nil` (no override value).
pub type CreateCustomerSessionComponentsMobilePaymentElementFeaturesPaymentMethodSaveAllowRedisplayOverride =
    stripe_shared::CardAllowRedisplay;
/// A list of [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) values that controls which saved payment methods the Payment Element displays by filtering to only show payment methods with an `allow_redisplay` value that is present in this list.
///
/// If not specified, defaults to ["always"].
/// In order to display all saved payment methods, specify ["always", "limited", "unspecified"].
pub type CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodAllowRedisplayFilters =
    stripe_shared::CardAllowRedisplay;
/// Controls whether or not the Payment Element shows saved payment methods.
/// This parameter defaults to `disabled`.
pub type CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRedisplay =
    stripe_shared::BalanceSettingsResourcePayoutsStatus;
/// Controls whether the Payment Element displays the option to remove a saved payment method.
/// This parameter defaults to `disabled`.
///
/// Allowing buyers to remove their saved payment methods impacts subscriptions that depend on that payment method.
/// Removing the payment method detaches the [`customer` object](https://docs.stripe.com/api/payment_methods/object#payment_method_object-customer) from that [PaymentMethod](https://docs.stripe.com/api/payment_methods).
pub type CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodRemove =
    stripe_shared::BalanceSettingsResourcePayoutsStatus;
/// Controls whether the Payment Element displays a checkbox offering to save a new payment method.
/// This parameter defaults to `disabled`.
///
/// If a customer checks the box, the [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) value on the PaymentMethod is set to `'always'` at confirmation time.
/// For PaymentIntents, the [`setup_future_usage`](https://docs.stripe.com/api/payment_intents/object#payment_intent_object-setup_future_usage) value is also set to the value defined in `payment_method_save_usage`.
pub type CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSave =
    stripe_shared::BalanceSettingsResourcePayoutsStatus;
/// When using PaymentIntents and the customer checks the save checkbox, this field determines the [`setup_future_usage`](https://docs.stripe.com/api/payment_intents/object#payment_intent_object-setup_future_usage) value used to confirm the PaymentIntent.
///
/// When using SetupIntents, directly configure the [`usage`](https://docs.stripe.com/api/setup_intents/object#setup_intent_object-usage) value on SetupIntent creation.
pub type CreateCustomerSessionComponentsPaymentElementFeaturesPaymentMethodSaveUsage =
    stripe_shared::CreateSetupIntentUsage;
