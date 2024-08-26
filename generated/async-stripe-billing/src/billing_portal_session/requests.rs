use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct CreateBillingPortalSessionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<String>,
    customer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flow_data: Option<CreateBillingPortalSessionFlowData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locale: Option<stripe_billing::BillingPortalSessionLocale>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_behalf_of: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    return_url: Option<String>,
}
impl CreateBillingPortalSessionBuilder {
    fn new(customer: impl Into<String>) -> Self {
        Self {
            configuration: None,
            customer: customer.into(),
            expand: None,
            flow_data: None,
            locale: None,
            on_behalf_of: None,
            return_url: None,
        }
    }
}
/// Information about a specific flow for the customer to go through.
/// See the [docs](https://stripe.com/docs/customer-management/portal-deep-links) to learn more about using customer portal deep links and flows.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalSessionFlowData {
    /// Behavior after the flow is completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_completion: Option<CreateBillingPortalSessionFlowDataAfterCompletion>,
    /// Configuration when `flow_data.type=subscription_cancel`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_cancel: Option<CreateBillingPortalSessionFlowDataSubscriptionCancel>,
    /// Configuration when `flow_data.type=subscription_update`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_update: Option<CreateBillingPortalSessionFlowDataSubscriptionUpdate>,
    /// Configuration when `flow_data.type=subscription_update_confirm`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_update_confirm:
        Option<CreateBillingPortalSessionFlowDataSubscriptionUpdateConfirm>,
    /// Type of flow that the customer will go through.
    #[serde(rename = "type")]
    pub type_: CreateBillingPortalSessionFlowDataType,
}
impl CreateBillingPortalSessionFlowData {
    pub fn new(type_: impl Into<CreateBillingPortalSessionFlowDataType>) -> Self {
        Self {
            after_completion: None,
            subscription_cancel: None,
            subscription_update: None,
            subscription_update_confirm: None,
            type_: type_.into(),
        }
    }
}
/// Behavior after the flow is completed.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalSessionFlowDataAfterCompletion {
    /// Configuration when `after_completion.type=hosted_confirmation`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_confirmation:
        Option<CreateBillingPortalSessionFlowDataAfterCompletionHostedConfirmation>,
    /// Configuration when `after_completion.type=redirect`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<CreateBillingPortalSessionFlowDataAfterCompletionRedirect>,
    /// The specified behavior after the flow is completed.
    #[serde(rename = "type")]
    pub type_: CreateBillingPortalSessionFlowDataAfterCompletionType,
}
impl CreateBillingPortalSessionFlowDataAfterCompletion {
    pub fn new(type_: impl Into<CreateBillingPortalSessionFlowDataAfterCompletionType>) -> Self {
        Self { hosted_confirmation: None, redirect: None, type_: type_.into() }
    }
}
/// Configuration when `after_completion.type=hosted_confirmation`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalSessionFlowDataAfterCompletionHostedConfirmation {
    /// A custom message to display to the customer after the flow is completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<String>,
}
impl CreateBillingPortalSessionFlowDataAfterCompletionHostedConfirmation {
    pub fn new() -> Self {
        Self { custom_message: None }
    }
}
impl Default for CreateBillingPortalSessionFlowDataAfterCompletionHostedConfirmation {
    fn default() -> Self {
        Self::new()
    }
}
/// Configuration when `after_completion.type=redirect`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalSessionFlowDataAfterCompletionRedirect {
    /// The URL the customer will be redirected to after the flow is completed.
    pub return_url: String,
}
impl CreateBillingPortalSessionFlowDataAfterCompletionRedirect {
    pub fn new(return_url: impl Into<String>) -> Self {
        Self { return_url: return_url.into() }
    }
}
/// The specified behavior after the flow is completed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateBillingPortalSessionFlowDataAfterCompletionType {
    HostedConfirmation,
    PortalHomepage,
    Redirect,
}
impl CreateBillingPortalSessionFlowDataAfterCompletionType {
    pub fn as_str(self) -> &'static str {
        use CreateBillingPortalSessionFlowDataAfterCompletionType::*;
        match self {
            HostedConfirmation => "hosted_confirmation",
            PortalHomepage => "portal_homepage",
            Redirect => "redirect",
        }
    }
}

impl std::str::FromStr for CreateBillingPortalSessionFlowDataAfterCompletionType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingPortalSessionFlowDataAfterCompletionType::*;
        match s {
            "hosted_confirmation" => Ok(HostedConfirmation),
            "portal_homepage" => Ok(PortalHomepage),
            "redirect" => Ok(Redirect),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateBillingPortalSessionFlowDataAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateBillingPortalSessionFlowDataAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateBillingPortalSessionFlowDataAfterCompletionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateBillingPortalSessionFlowDataAfterCompletionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateBillingPortalSessionFlowDataAfterCompletionType",
            )
        })
    }
}
/// Configuration when `flow_data.type=subscription_cancel`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalSessionFlowDataSubscriptionCancel {
    /// Specify a retention strategy to be used in the cancellation flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention: Option<CreateBillingPortalSessionFlowDataSubscriptionCancelRetention>,
    /// The ID of the subscription to be canceled.
    pub subscription: String,
}
impl CreateBillingPortalSessionFlowDataSubscriptionCancel {
    pub fn new(subscription: impl Into<String>) -> Self {
        Self { retention: None, subscription: subscription.into() }
    }
}
/// Specify a retention strategy to be used in the cancellation flow.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalSessionFlowDataSubscriptionCancelRetention {
    /// Configuration when `retention.type=coupon_offer`.
    pub coupon_offer: CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionCouponOffer,
    /// Type of retention strategy to use with the customer.
    #[serde(rename = "type")]
    pub type_: CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionType,
}
impl CreateBillingPortalSessionFlowDataSubscriptionCancelRetention {
    pub fn new(
        coupon_offer: impl Into<
            CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionCouponOffer,
        >,
        type_: impl Into<CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionType>,
    ) -> Self {
        Self { coupon_offer: coupon_offer.into(), type_: type_.into() }
    }
}
/// Configuration when `retention.type=coupon_offer`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionCouponOffer {
    /// The ID of the coupon to be offered.
    pub coupon: String,
}
impl CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionCouponOffer {
    pub fn new(coupon: impl Into<String>) -> Self {
        Self { coupon: coupon.into() }
    }
}
/// Type of retention strategy to use with the customer.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionType {
    CouponOffer,
}
impl CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionType {
    pub fn as_str(self) -> &'static str {
        use CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionType::*;
        match self {
            CouponOffer => "coupon_offer",
        }
    }
}

impl std::str::FromStr for CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionType::*;
        match s {
            "coupon_offer" => Ok(CouponOffer),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionType"))
    }
}
/// Configuration when `flow_data.type=subscription_update`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalSessionFlowDataSubscriptionUpdate {
    /// The ID of the subscription to be updated.
    pub subscription: String,
}
impl CreateBillingPortalSessionFlowDataSubscriptionUpdate {
    pub fn new(subscription: impl Into<String>) -> Self {
        Self { subscription: subscription.into() }
    }
}
/// Configuration when `flow_data.type=subscription_update_confirm`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalSessionFlowDataSubscriptionUpdateConfirm {
    /// The coupon or promotion code to apply to this subscription update.
    /// Currently, only up to one may be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts:
        Option<Vec<CreateBillingPortalSessionFlowDataSubscriptionUpdateConfirmDiscounts>>,
    /// The [subscription item](https://stripe.com/docs/api/subscription_items) to be updated through this flow.
    /// Currently, only up to one may be specified and subscriptions with multiple items are not updatable.
    pub items: Vec<CreateBillingPortalSessionFlowDataSubscriptionUpdateConfirmItems>,
    /// The ID of the subscription to be updated.
    pub subscription: String,
}
impl CreateBillingPortalSessionFlowDataSubscriptionUpdateConfirm {
    pub fn new(
        items: impl Into<Vec<CreateBillingPortalSessionFlowDataSubscriptionUpdateConfirmItems>>,
        subscription: impl Into<String>,
    ) -> Self {
        Self { discounts: None, items: items.into(), subscription: subscription.into() }
    }
}
/// The coupon or promotion code to apply to this subscription update.
/// Currently, only up to one may be specified.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalSessionFlowDataSubscriptionUpdateConfirmDiscounts {
    /// The ID of the coupon to apply to this subscription update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,
    /// The ID of a promotion code to apply to this subscription update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
}
impl CreateBillingPortalSessionFlowDataSubscriptionUpdateConfirmDiscounts {
    pub fn new() -> Self {
        Self { coupon: None, promotion_code: None }
    }
}
impl Default for CreateBillingPortalSessionFlowDataSubscriptionUpdateConfirmDiscounts {
    fn default() -> Self {
        Self::new()
    }
}
/// The [subscription item](https://stripe.com/docs/api/subscription_items) to be updated through this flow.
/// Currently, only up to one may be specified and subscriptions with multiple items are not updatable.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalSessionFlowDataSubscriptionUpdateConfirmItems {
    /// The ID of the [subscription item](https://stripe.com/docs/api/subscriptions/object#subscription_object-items-data-id) to be updated.
    pub id: String,
    /// The price the customer should subscribe to through this flow.
    /// The price must also be included in the configuration's [`features.subscription_update.products`](https://stripe.com/docs/api/customer_portal/configuration#portal_configuration_object-features-subscription_update-products).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// [Quantity](https://stripe.com/docs/subscriptions/quantities) for this item that the customer should subscribe to through this flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
}
impl CreateBillingPortalSessionFlowDataSubscriptionUpdateConfirmItems {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into(), price: None, quantity: None }
    }
}
/// Type of flow that the customer will go through.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateBillingPortalSessionFlowDataType {
    PaymentMethodUpdate,
    SubscriptionCancel,
    SubscriptionUpdate,
    SubscriptionUpdateConfirm,
}
impl CreateBillingPortalSessionFlowDataType {
    pub fn as_str(self) -> &'static str {
        use CreateBillingPortalSessionFlowDataType::*;
        match self {
            PaymentMethodUpdate => "payment_method_update",
            SubscriptionCancel => "subscription_cancel",
            SubscriptionUpdate => "subscription_update",
            SubscriptionUpdateConfirm => "subscription_update_confirm",
        }
    }
}

impl std::str::FromStr for CreateBillingPortalSessionFlowDataType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingPortalSessionFlowDataType::*;
        match s {
            "payment_method_update" => Ok(PaymentMethodUpdate),
            "subscription_cancel" => Ok(SubscriptionCancel),
            "subscription_update" => Ok(SubscriptionUpdate),
            "subscription_update_confirm" => Ok(SubscriptionUpdateConfirm),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateBillingPortalSessionFlowDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateBillingPortalSessionFlowDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateBillingPortalSessionFlowDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateBillingPortalSessionFlowDataType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateBillingPortalSessionFlowDataType")
        })
    }
}
/// Creates a session of the customer portal.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalSession {
    inner: CreateBillingPortalSessionBuilder,
}
impl CreateBillingPortalSession {
    /// Construct a new `CreateBillingPortalSession`.
    pub fn new(customer: impl Into<String>) -> Self {
        Self { inner: CreateBillingPortalSessionBuilder::new(customer.into()) }
    }
    /// The ID of an existing [configuration](https://stripe.com/docs/api/customer_portal/configuration) to use for this session, describing its functionality and features.
    /// If not specified, the session uses the default configuration.
    pub fn configuration(mut self, configuration: impl Into<String>) -> Self {
        self.inner.configuration = Some(configuration.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Information about a specific flow for the customer to go through.
    /// See the [docs](https://stripe.com/docs/customer-management/portal-deep-links) to learn more about using customer portal deep links and flows.
    pub fn flow_data(mut self, flow_data: impl Into<CreateBillingPortalSessionFlowData>) -> Self {
        self.inner.flow_data = Some(flow_data.into());
        self
    }
    /// The IETF language tag of the locale customer portal is displayed in.
    /// If blank or auto, the customer’s `preferred_locales` or browser’s locale is used.
    pub fn locale(mut self, locale: impl Into<stripe_billing::BillingPortalSessionLocale>) -> Self {
        self.inner.locale = Some(locale.into());
        self
    }
    /// The `on_behalf_of` account to use for this session.
    /// When specified, only subscriptions and invoices with this `on_behalf_of` account appear in the portal.
    /// For more information, see the [docs](https://stripe.com/docs/connect/separate-charges-and-transfers#settlement-merchant).
    /// Use the [Accounts API](https://stripe.com/docs/api/accounts/object#account_object-settings-branding) to modify the `on_behalf_of` account's branding settings, which the portal displays.
    pub fn on_behalf_of(mut self, on_behalf_of: impl Into<String>) -> Self {
        self.inner.on_behalf_of = Some(on_behalf_of.into());
        self
    }
    /// The default URL to redirect customers to when they click on the portal's link to return to your website.
    pub fn return_url(mut self, return_url: impl Into<String>) -> Self {
        self.inner.return_url = Some(return_url.into());
        self
    }
}
impl CreateBillingPortalSession {
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

impl StripeRequest for CreateBillingPortalSession {
    type Output = stripe_billing::BillingPortalSession;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/billing_portal/sessions").form(&self.inner)
    }
}
