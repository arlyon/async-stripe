// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::{BillingPortalSessionId, CustomerId};
use crate::params::{Expand, Expandable, Object, Timestamp};
use crate::resources::BillingPortalConfiguration;
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "PortalSession".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingPortalSession {
    /// Unique identifier for the object.
    pub id: BillingPortalSessionId,

    /// The configuration used by this session, describing the features available.
    pub configuration: Expandable<BillingPortalConfiguration>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The ID of the customer for this session.
    pub customer: String,

    /// Information about a specific flow for the customer to go through.
    ///
    /// See the [docs](https://stripe.com/docs/customer-management/portal-deep-links) to learn more about using customer portal deep links and flows.
    pub flow: Option<PortalFlowsFlow>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The IETF language tag of the locale Customer Portal is displayed in.
    ///
    /// If blank or auto, the customer’s `preferred_locales` or browser’s locale is used.
    pub locale: Option<BillingPortalSessionLocale>,

    /// The account for which the session was created on behalf of.
    ///
    /// When specified, only subscriptions and invoices with this `on_behalf_of` account appear in the portal.
    /// For more information, see the [docs](https://stripe.com/docs/connect/separate-charges-and-transfers#on-behalf-of).
    /// Use the [Accounts API](https://stripe.com/docs/api/accounts/object#account_object-settings-branding) to modify the `on_behalf_of` account's branding settings, which the portal displays.
    pub on_behalf_of: Option<String>,

    /// The URL to redirect customers to when they click on the portal's link to return to your website.
    pub return_url: Option<String>,

    /// The short-lived URL of the session that gives customers access to the customer portal.
    pub url: String,
}

impl BillingPortalSession {
    /// Creates a session of the customer portal.
    pub fn create(
        client: &Client,
        params: CreateBillingPortalSession<'_>,
    ) -> Response<BillingPortalSession> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form("/billing_portal/sessions", &params)
    }
}

impl Object for BillingPortalSession {
    type Id = BillingPortalSessionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "billing_portal.session"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PortalFlowsFlow {
    pub after_completion: PortalFlowsFlowAfterCompletion,

    /// Configuration when `flow.type=subscription_cancel`.
    pub subscription_cancel: Option<PortalFlowsFlowSubscriptionCancel>,

    /// Configuration when `flow.type=subscription_update`.
    pub subscription_update: Option<PortalFlowsFlowSubscriptionUpdate>,

    /// Configuration when `flow.type=subscription_update_confirm`.
    pub subscription_update_confirm: Option<PortalFlowsFlowSubscriptionUpdateConfirm>,

    /// Type of flow that the customer will go through.
    #[serde(rename = "type")]
    pub type_: PortalFlowsFlowType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PortalFlowsFlowAfterCompletion {
    /// Configuration when `after_completion.type=hosted_confirmation`.
    pub hosted_confirmation: Option<PortalFlowsAfterCompletionHostedConfirmation>,

    /// Configuration when `after_completion.type=redirect`.
    pub redirect: Option<PortalFlowsAfterCompletionRedirect>,

    /// The specified type of behavior after the flow is completed.
    #[serde(rename = "type")]
    pub type_: PortalFlowsFlowAfterCompletionType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PortalFlowsAfterCompletionHostedConfirmation {
    /// A custom message to display to the customer after the flow is completed.
    pub custom_message: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PortalFlowsAfterCompletionRedirect {
    /// The URL the customer will be redirected to after the flow is completed.
    pub return_url: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PortalFlowsFlowSubscriptionCancel {
    /// Specify a retention strategy to be used in the cancellation flow.
    pub retention: Option<PortalFlowsRetention>,

    /// The ID of the subscription to be canceled.
    pub subscription: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PortalFlowsFlowSubscriptionUpdate {
    /// The ID of the subscription to be updated.
    pub subscription: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PortalFlowsFlowSubscriptionUpdateConfirm {
    /// The coupon or promotion code to apply to this subscription update.
    ///
    /// Currently, only up to one may be specified.
    pub discounts: Option<Vec<PortalFlowsSubscriptionUpdateConfirmDiscount>>,

    /// The [subscription item](https://stripe.com/docs/api/subscription_items) to be updated through this flow.
    ///
    /// Currently, only up to one may be specified and subscriptions with multiple items are not updatable.
    pub items: Vec<PortalFlowsSubscriptionUpdateConfirmItem>,

    /// The ID of the subscription to be updated.
    pub subscription: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PortalFlowsRetention {
    /// Configuration when `retention.type=coupon_offer`.
    pub coupon_offer: Option<PortalFlowsCouponOffer>,

    /// Type of retention strategy that will be used.
    #[serde(rename = "type")]
    pub type_: PortalFlowsRetentionType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PortalFlowsCouponOffer {
    /// The ID of the coupon to be offered.
    pub coupon: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PortalFlowsSubscriptionUpdateConfirmDiscount {
    /// The ID of the coupon to apply to this subscription update.
    pub coupon: Option<String>,

    /// The ID of a promotion code to apply to this subscription update.
    pub promotion_code: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PortalFlowsSubscriptionUpdateConfirmItem {
    /// The ID of the [subscription item](https://stripe.com/docs/api/subscriptions/object#subscription_object-items-data-id) to be updated.
    pub id: Option<String>,

    /// The price the customer should subscribe to through this flow.
    ///
    /// The price must also be included in the configuration's [`features.subscription_update.products`](https://stripe.com/docs/api/customer_portal/configuration#portal_configuration_object-features-subscription_update-products).
    pub price: Option<String>,

    /// [Quantity](https://stripe.com/docs/subscriptions/quantities) for this item that the customer should subscribe to through this flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
}

/// The parameters for `BillingPortalSession::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreateBillingPortalSession<'a> {
    /// The ID of an existing [configuration](https://stripe.com/docs/api/customer_portal/configuration) to use for this session, describing its functionality and features.
    ///
    /// If not specified, the session uses the default configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<&'a str>,

    /// The ID of an existing customer.
    pub customer: CustomerId,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Information about a specific flow for the customer to go through.
    ///
    /// See the [docs](https://stripe.com/docs/customer-management/portal-deep-links) to learn more about using customer portal deep links and flows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_data: Option<CreateBillingPortalSessionFlowData>,

    /// The IETF language tag of the locale customer portal is displayed in.
    ///
    /// If blank or auto, the customer’s `preferred_locales` or browser’s locale is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<BillingPortalSessionLocale>,

    /// The `on_behalf_of` account to use for this session.
    ///
    /// When specified, only subscriptions and invoices with this `on_behalf_of` account appear in the portal.
    /// For more information, see the [docs](https://stripe.com/docs/connect/separate-charges-and-transfers#on-behalf-of).
    /// Use the [Accounts API](https://stripe.com/docs/api/accounts/object#account_object-settings-branding) to modify the `on_behalf_of` account's branding settings, which the portal displays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,

    /// The default URL to redirect customers to when they click on the portal's link to return to your website.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
}

impl<'a> CreateBillingPortalSession<'a> {
    pub fn new(customer: CustomerId) -> Self {
        CreateBillingPortalSession {
            configuration: Default::default(),
            customer,
            expand: Default::default(),
            flow_data: Default::default(),
            locale: Default::default(),
            on_behalf_of: Default::default(),
            return_url: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateBillingPortalSessionFlowDataSubscriptionCancel {
    /// Specify a retention strategy to be used in the cancellation flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention: Option<CreateBillingPortalSessionFlowDataSubscriptionCancelRetention>,

    /// The ID of the subscription to be canceled.
    pub subscription: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateBillingPortalSessionFlowDataSubscriptionUpdate {
    /// The ID of the subscription to be updated.
    pub subscription: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateBillingPortalSessionFlowDataSubscriptionUpdateConfirm {
    /// The coupon or promotion code to apply to this subscription update.
    ///
    /// Currently, only up to one may be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts:
        Option<Vec<CreateBillingPortalSessionFlowDataSubscriptionUpdateConfirmDiscounts>>,

    /// The [subscription item](https://stripe.com/docs/api/subscription_items) to be updated through this flow.
    ///
    /// Currently, only up to one may be specified and subscriptions with multiple items are not updatable.
    pub items: Vec<CreateBillingPortalSessionFlowDataSubscriptionUpdateConfirmItems>,

    /// The ID of the subscription to be updated.
    pub subscription: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateBillingPortalSessionFlowDataAfterCompletionHostedConfirmation {
    /// A custom message to display to the customer after the flow is completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateBillingPortalSessionFlowDataAfterCompletionRedirect {
    /// The URL the customer will be redirected to after the flow is completed.
    pub return_url: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateBillingPortalSessionFlowDataSubscriptionCancelRetention {
    /// Configuration when `retention.type=coupon_offer`.
    pub coupon_offer: CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionCouponOffer,

    /// Type of retention strategy to use with the customer.
    #[serde(rename = "type")]
    pub type_: CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateBillingPortalSessionFlowDataSubscriptionUpdateConfirmDiscounts {
    /// The ID of the coupon to apply to this subscription update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    /// The ID of a promotion code to apply to this subscription update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateBillingPortalSessionFlowDataSubscriptionUpdateConfirmItems {
    /// The ID of the [subscription item](https://stripe.com/docs/api/subscriptions/object#subscription_object-items-data-id) to be updated.
    pub id: String,

    /// The price the customer should subscribe to through this flow.
    ///
    /// The price must also be included in the configuration's [`features.subscription_update.products`](https://stripe.com/docs/api/customer_portal/configuration#portal_configuration_object-features-subscription_update-products).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// [Quantity](https://stripe.com/docs/subscriptions/quantities) for this item that the customer should subscribe to through this flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionCouponOffer {
    /// The ID of the coupon to be offered.
    pub coupon: String,
}

/// An enum representing the possible values of an `BillingPortalSession`'s `locale` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BillingPortalSessionLocale {
    Auto,
    Bg,
    Cs,
    Da,
    De,
    El,
    En,
    #[serde(rename = "en-AU")]
    EnAu,
    #[serde(rename = "en-CA")]
    EnCa,
    #[serde(rename = "en-GB")]
    EnGb,
    #[serde(rename = "en-IE")]
    EnIe,
    #[serde(rename = "en-IN")]
    EnIn,
    #[serde(rename = "en-NZ")]
    EnNz,
    #[serde(rename = "en-SG")]
    EnSg,
    Es,
    #[serde(rename = "es-419")]
    Es419,
    Et,
    Fi,
    Fil,
    Fr,
    #[serde(rename = "fr-CA")]
    FrCa,
    Hr,
    Hu,
    Id,
    It,
    Ja,
    Ko,
    Lt,
    Lv,
    Ms,
    Mt,
    Nb,
    Nl,
    Pl,
    Pt,
    #[serde(rename = "pt-BR")]
    PtBr,
    Ro,
    Ru,
    Sk,
    Sl,
    Sv,
    Th,
    Tr,
    Vi,
    Zh,
    #[serde(rename = "zh-HK")]
    ZhHk,
    #[serde(rename = "zh-TW")]
    ZhTw,
}

impl BillingPortalSessionLocale {
    pub fn as_str(self) -> &'static str {
        match self {
            BillingPortalSessionLocale::Auto => "auto",
            BillingPortalSessionLocale::Bg => "bg",
            BillingPortalSessionLocale::Cs => "cs",
            BillingPortalSessionLocale::Da => "da",
            BillingPortalSessionLocale::De => "de",
            BillingPortalSessionLocale::El => "el",
            BillingPortalSessionLocale::En => "en",
            BillingPortalSessionLocale::EnAu => "en-AU",
            BillingPortalSessionLocale::EnCa => "en-CA",
            BillingPortalSessionLocale::EnGb => "en-GB",
            BillingPortalSessionLocale::EnIe => "en-IE",
            BillingPortalSessionLocale::EnIn => "en-IN",
            BillingPortalSessionLocale::EnNz => "en-NZ",
            BillingPortalSessionLocale::EnSg => "en-SG",
            BillingPortalSessionLocale::Es => "es",
            BillingPortalSessionLocale::Es419 => "es-419",
            BillingPortalSessionLocale::Et => "et",
            BillingPortalSessionLocale::Fi => "fi",
            BillingPortalSessionLocale::Fil => "fil",
            BillingPortalSessionLocale::Fr => "fr",
            BillingPortalSessionLocale::FrCa => "fr-CA",
            BillingPortalSessionLocale::Hr => "hr",
            BillingPortalSessionLocale::Hu => "hu",
            BillingPortalSessionLocale::Id => "id",
            BillingPortalSessionLocale::It => "it",
            BillingPortalSessionLocale::Ja => "ja",
            BillingPortalSessionLocale::Ko => "ko",
            BillingPortalSessionLocale::Lt => "lt",
            BillingPortalSessionLocale::Lv => "lv",
            BillingPortalSessionLocale::Ms => "ms",
            BillingPortalSessionLocale::Mt => "mt",
            BillingPortalSessionLocale::Nb => "nb",
            BillingPortalSessionLocale::Nl => "nl",
            BillingPortalSessionLocale::Pl => "pl",
            BillingPortalSessionLocale::Pt => "pt",
            BillingPortalSessionLocale::PtBr => "pt-BR",
            BillingPortalSessionLocale::Ro => "ro",
            BillingPortalSessionLocale::Ru => "ru",
            BillingPortalSessionLocale::Sk => "sk",
            BillingPortalSessionLocale::Sl => "sl",
            BillingPortalSessionLocale::Sv => "sv",
            BillingPortalSessionLocale::Th => "th",
            BillingPortalSessionLocale::Tr => "tr",
            BillingPortalSessionLocale::Vi => "vi",
            BillingPortalSessionLocale::Zh => "zh",
            BillingPortalSessionLocale::ZhHk => "zh-HK",
            BillingPortalSessionLocale::ZhTw => "zh-TW",
        }
    }
}

impl AsRef<str> for BillingPortalSessionLocale {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BillingPortalSessionLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for BillingPortalSessionLocale {
    fn default() -> Self {
        Self::Auto
    }
}

/// An enum representing the possible values of an `CreateBillingPortalSessionFlowDataAfterCompletion`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateBillingPortalSessionFlowDataAfterCompletionType {
    HostedConfirmation,
    PortalHomepage,
    Redirect,
}

impl CreateBillingPortalSessionFlowDataAfterCompletionType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateBillingPortalSessionFlowDataAfterCompletionType::HostedConfirmation => {
                "hosted_confirmation"
            }
            CreateBillingPortalSessionFlowDataAfterCompletionType::PortalHomepage => {
                "portal_homepage"
            }
            CreateBillingPortalSessionFlowDataAfterCompletionType::Redirect => "redirect",
        }
    }
}

impl AsRef<str> for CreateBillingPortalSessionFlowDataAfterCompletionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateBillingPortalSessionFlowDataAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateBillingPortalSessionFlowDataAfterCompletionType {
    fn default() -> Self {
        Self::HostedConfirmation
    }
}

/// An enum representing the possible values of an `CreateBillingPortalSessionFlowDataSubscriptionCancelRetention`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionType {
    CouponOffer,
}

impl CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionType::CouponOffer => {
                "coupon_offer"
            }
        }
    }
}

impl AsRef<str> for CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateBillingPortalSessionFlowDataSubscriptionCancelRetentionType {
    fn default() -> Self {
        Self::CouponOffer
    }
}

/// An enum representing the possible values of an `CreateBillingPortalSessionFlowData`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateBillingPortalSessionFlowDataType {
    PaymentMethodUpdate,
    SubscriptionCancel,
    SubscriptionUpdate,
    SubscriptionUpdateConfirm,
}

impl CreateBillingPortalSessionFlowDataType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateBillingPortalSessionFlowDataType::PaymentMethodUpdate => "payment_method_update",
            CreateBillingPortalSessionFlowDataType::SubscriptionCancel => "subscription_cancel",
            CreateBillingPortalSessionFlowDataType::SubscriptionUpdate => "subscription_update",
            CreateBillingPortalSessionFlowDataType::SubscriptionUpdateConfirm => {
                "subscription_update_confirm"
            }
        }
    }
}

impl AsRef<str> for CreateBillingPortalSessionFlowDataType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateBillingPortalSessionFlowDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateBillingPortalSessionFlowDataType {
    fn default() -> Self {
        Self::PaymentMethodUpdate
    }
}

/// An enum representing the possible values of an `PortalFlowsFlowAfterCompletion`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PortalFlowsFlowAfterCompletionType {
    HostedConfirmation,
    PortalHomepage,
    Redirect,
}

impl PortalFlowsFlowAfterCompletionType {
    pub fn as_str(self) -> &'static str {
        match self {
            PortalFlowsFlowAfterCompletionType::HostedConfirmation => "hosted_confirmation",
            PortalFlowsFlowAfterCompletionType::PortalHomepage => "portal_homepage",
            PortalFlowsFlowAfterCompletionType::Redirect => "redirect",
        }
    }
}

impl AsRef<str> for PortalFlowsFlowAfterCompletionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PortalFlowsFlowAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PortalFlowsFlowAfterCompletionType {
    fn default() -> Self {
        Self::HostedConfirmation
    }
}

/// An enum representing the possible values of an `PortalFlowsFlow`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PortalFlowsFlowType {
    PaymentMethodUpdate,
    SubscriptionCancel,
    SubscriptionUpdate,
    SubscriptionUpdateConfirm,
}

impl PortalFlowsFlowType {
    pub fn as_str(self) -> &'static str {
        match self {
            PortalFlowsFlowType::PaymentMethodUpdate => "payment_method_update",
            PortalFlowsFlowType::SubscriptionCancel => "subscription_cancel",
            PortalFlowsFlowType::SubscriptionUpdate => "subscription_update",
            PortalFlowsFlowType::SubscriptionUpdateConfirm => "subscription_update_confirm",
        }
    }
}

impl AsRef<str> for PortalFlowsFlowType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PortalFlowsFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PortalFlowsFlowType {
    fn default() -> Self {
        Self::PaymentMethodUpdate
    }
}

/// An enum representing the possible values of an `PortalFlowsRetention`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PortalFlowsRetentionType {
    CouponOffer,
}

impl PortalFlowsRetentionType {
    pub fn as_str(self) -> &'static str {
        match self {
            PortalFlowsRetentionType::CouponOffer => "coupon_offer",
        }
    }
}

impl AsRef<str> for PortalFlowsRetentionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PortalFlowsRetentionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PortalFlowsRetentionType {
    fn default() -> Self {
        Self::CouponOffer
    }
}
