// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::BillingPortalConfigurationId;
use crate::params::{Expandable, Metadata, Object, Timestamp};
use crate::resources::Application;
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "PortalConfiguration".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingPortalConfiguration {
    /// Unique identifier for the object.
    pub id: BillingPortalConfigurationId,

    /// Whether the configuration is active and can be used to create portal sessions.
    pub active: bool,

    /// ID of the Connect Application that created the configuration.
    pub application: Option<Expandable<Application>>,

    pub business_profile: PortalBusinessProfile,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The default URL to redirect customers to when they click on the portal's link to return to your website.
    ///
    /// This can be [overriden](https://stripe.com/docs/api/customer_portal/sessions/create#create_portal_session-return_url) when creating the session.
    pub default_return_url: Option<String>,

    pub features: PortalFeatures,

    /// Whether the configuration is the default.
    ///
    /// If `true`, this configuration can be managed in the Dashboard and portal sessions will use this configuration unless it is overriden when creating the session.
    pub is_default: bool,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    pub login_page: PortalLoginPage,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<Metadata>,

    /// Time at which the object was last updated.
    ///
    /// Measured in seconds since the Unix epoch.
    pub updated: Timestamp,
}

impl Object for BillingPortalConfiguration {
    type Id = BillingPortalConfigurationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "billing_portal.configuration"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PortalBusinessProfile {
    /// The messaging shown to customers in the portal.
    pub headline: Option<String>,

    /// A link to the business’s publicly available privacy policy.
    pub privacy_policy_url: Option<String>,

    /// A link to the business’s publicly available terms of service.
    pub terms_of_service_url: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PortalFeatures {
    pub customer_update: PortalCustomerUpdate,

    pub invoice_history: PortalInvoiceList,

    pub payment_method_update: PortalPaymentMethodUpdate,

    pub subscription_cancel: PortalSubscriptionCancel,

    pub subscription_pause: PortalSubscriptionPause,

    pub subscription_update: PortalSubscriptionUpdate,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PortalCustomerUpdate {
    /// The types of customer updates that are supported.
    ///
    /// When empty, customers are not updateable.
    pub allowed_updates: Vec<PortalCustomerUpdateAllowedUpdates>,

    /// Whether the feature is enabled.
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PortalInvoiceList {
    /// Whether the feature is enabled.
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PortalLoginPage {
    /// If `true`, a shareable `url` will be generated that will take your customers to a hosted login page for the customer portal.
    ///
    /// If `false`, the previously generated `url`, if any, will be deactivated.
    pub enabled: bool,

    /// A shareable URL to the hosted portal login page.
    ///
    /// Your customers will be able to log in with their [email](https://stripe.com/docs/api/customers/object#customer_object-email) and receive a link to their customer portal.
    pub url: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PortalPaymentMethodUpdate {
    /// Whether the feature is enabled.
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PortalSubscriptionCancel {
    pub cancellation_reason: PortalSubscriptionCancellationReason,

    /// Whether the feature is enabled.
    pub enabled: bool,

    /// Whether to cancel subscriptions immediately or at the end of the billing period.
    pub mode: PortalSubscriptionCancelMode,

    /// Whether to create prorations when canceling subscriptions.
    ///
    /// Possible values are `none` and `create_prorations`.
    pub proration_behavior: PortalSubscriptionCancelProrationBehavior,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PortalSubscriptionCancellationReason {
    /// Whether the feature is enabled.
    pub enabled: bool,

    /// Which cancellation reasons will be given as options to the customer.
    pub options: Vec<PortalSubscriptionCancellationReasonOptions>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PortalSubscriptionPause {
    /// Whether the feature is enabled.
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PortalSubscriptionUpdate {
    /// The types of subscription updates that are supported for items listed in the `products` attribute.
    ///
    /// When empty, subscriptions are not updateable.
    pub default_allowed_updates: Vec<PortalSubscriptionUpdateDefaultAllowedUpdates>,

    /// Whether the feature is enabled.
    pub enabled: bool,

    /// The list of up to 10 products that support subscription updates.
    pub products: Option<Vec<PortalSubscriptionUpdateProduct>>,

    /// Determines how to handle prorations resulting from subscription updates.
    ///
    /// Valid values are `none`, `create_prorations`, and `always_invoice`.
    pub proration_behavior: PortalSubscriptionUpdateProrationBehavior,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PortalSubscriptionUpdateProduct {
    /// The list of price IDs which, when subscribed to, a subscription can be updated.
    pub prices: Vec<String>,

    /// The product ID.
    pub product: String,
}

/// An enum representing the possible values of an `PortalCustomerUpdate`'s `allowed_updates` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PortalCustomerUpdateAllowedUpdates {
    Address,
    Email,
    Name,
    Phone,
    Shipping,
    TaxId,
}

impl PortalCustomerUpdateAllowedUpdates {
    pub fn as_str(self) -> &'static str {
        match self {
            PortalCustomerUpdateAllowedUpdates::Address => "address",
            PortalCustomerUpdateAllowedUpdates::Email => "email",
            PortalCustomerUpdateAllowedUpdates::Name => "name",
            PortalCustomerUpdateAllowedUpdates::Phone => "phone",
            PortalCustomerUpdateAllowedUpdates::Shipping => "shipping",
            PortalCustomerUpdateAllowedUpdates::TaxId => "tax_id",
        }
    }
}

impl AsRef<str> for PortalCustomerUpdateAllowedUpdates {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PortalCustomerUpdateAllowedUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PortalCustomerUpdateAllowedUpdates {
    fn default() -> Self {
        Self::Address
    }
}

/// An enum representing the possible values of an `PortalSubscriptionCancel`'s `mode` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PortalSubscriptionCancelMode {
    AtPeriodEnd,
    Immediately,
}

impl PortalSubscriptionCancelMode {
    pub fn as_str(self) -> &'static str {
        match self {
            PortalSubscriptionCancelMode::AtPeriodEnd => "at_period_end",
            PortalSubscriptionCancelMode::Immediately => "immediately",
        }
    }
}

impl AsRef<str> for PortalSubscriptionCancelMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PortalSubscriptionCancelMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PortalSubscriptionCancelMode {
    fn default() -> Self {
        Self::AtPeriodEnd
    }
}

/// An enum representing the possible values of an `PortalSubscriptionCancel`'s `proration_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PortalSubscriptionCancelProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl PortalSubscriptionCancelProrationBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            PortalSubscriptionCancelProrationBehavior::AlwaysInvoice => "always_invoice",
            PortalSubscriptionCancelProrationBehavior::CreateProrations => "create_prorations",
            PortalSubscriptionCancelProrationBehavior::None => "none",
        }
    }
}

impl AsRef<str> for PortalSubscriptionCancelProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PortalSubscriptionCancelProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PortalSubscriptionCancelProrationBehavior {
    fn default() -> Self {
        Self::AlwaysInvoice
    }
}

/// An enum representing the possible values of an `PortalSubscriptionCancellationReason`'s `options` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PortalSubscriptionCancellationReasonOptions {
    CustomerService,
    LowQuality,
    MissingFeatures,
    Other,
    SwitchedService,
    TooComplex,
    TooExpensive,
    Unused,
}

impl PortalSubscriptionCancellationReasonOptions {
    pub fn as_str(self) -> &'static str {
        match self {
            PortalSubscriptionCancellationReasonOptions::CustomerService => "customer_service",
            PortalSubscriptionCancellationReasonOptions::LowQuality => "low_quality",
            PortalSubscriptionCancellationReasonOptions::MissingFeatures => "missing_features",
            PortalSubscriptionCancellationReasonOptions::Other => "other",
            PortalSubscriptionCancellationReasonOptions::SwitchedService => "switched_service",
            PortalSubscriptionCancellationReasonOptions::TooComplex => "too_complex",
            PortalSubscriptionCancellationReasonOptions::TooExpensive => "too_expensive",
            PortalSubscriptionCancellationReasonOptions::Unused => "unused",
        }
    }
}

impl AsRef<str> for PortalSubscriptionCancellationReasonOptions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PortalSubscriptionCancellationReasonOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PortalSubscriptionCancellationReasonOptions {
    fn default() -> Self {
        Self::CustomerService
    }
}

/// An enum representing the possible values of an `PortalSubscriptionUpdate`'s `default_allowed_updates` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PortalSubscriptionUpdateDefaultAllowedUpdates {
    Price,
    PromotionCode,
    Quantity,
}

impl PortalSubscriptionUpdateDefaultAllowedUpdates {
    pub fn as_str(self) -> &'static str {
        match self {
            PortalSubscriptionUpdateDefaultAllowedUpdates::Price => "price",
            PortalSubscriptionUpdateDefaultAllowedUpdates::PromotionCode => "promotion_code",
            PortalSubscriptionUpdateDefaultAllowedUpdates::Quantity => "quantity",
        }
    }
}

impl AsRef<str> for PortalSubscriptionUpdateDefaultAllowedUpdates {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PortalSubscriptionUpdateDefaultAllowedUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PortalSubscriptionUpdateDefaultAllowedUpdates {
    fn default() -> Self {
        Self::Price
    }
}

/// An enum representing the possible values of an `PortalSubscriptionUpdate`'s `proration_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PortalSubscriptionUpdateProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl PortalSubscriptionUpdateProrationBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            PortalSubscriptionUpdateProrationBehavior::AlwaysInvoice => "always_invoice",
            PortalSubscriptionUpdateProrationBehavior::CreateProrations => "create_prorations",
            PortalSubscriptionUpdateProrationBehavior::None => "none",
        }
    }
}

impl AsRef<str> for PortalSubscriptionUpdateProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PortalSubscriptionUpdateProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PortalSubscriptionUpdateProrationBehavior {
    fn default() -> Self {
        Self::AlwaysInvoice
    }
}
