// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::ids::BillingPortalConfigurationId;
use crate::params::{Object, Timestamp};

/// The resource representing a Stripe "PortalConfiguration".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BillingPortalConfiguration {
    /// Unique identifier for the object.
    pub id: BillingPortalConfigurationId,

    /// Whether the configuration is active and can be used to create portal sessions.
    pub active: bool,

    /// ID of the Connect Application that created the configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<String>,

    pub business_profile: PortalBusinessProfile,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The default URL to redirect customers to when they click on the portal's link to return to your website.
    ///
    /// This can be [overriden](https://stripe.com/docs/api/customer_portal/sessions/create#create_portal_session-return_url) when creating the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_return_url: Option<String>,

    pub features: PortalFeatures,

    /// Whether the configuration is the default.
    ///
    /// If `true`, this configuration can be managed in the Dashboard and portal sessions will use this configuration unless it is overriden when creating the session.
    pub is_default: bool,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PortalBusinessProfile {
    /// The messaging shown to customers in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headline: Option<String>,

    /// A link to the business’s publicly available privacy policy.
    pub privacy_policy_url: String,

    /// A link to the business’s publicly available terms of service.
    pub terms_of_service_url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PortalFeatures {
    pub customer_update: PortalCustomerUpdate,

    pub invoice_history: PortalInvoiceList,

    pub payment_method_update: PortalPaymentMethodUpdate,

    pub subscription_cancel: PortalSubscriptionCancel,

    pub subscription_pause: PortalSubscriptionPause,

    pub subscription_update: PortalSubscriptionUpdate,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PortalCustomerUpdate {
    /// The types of customer updates that are supported.
    ///
    /// When empty, customers are not updateable.
    pub allowed_updates: Vec<PortalCustomerUpdateAllowedUpdates>,

    /// Whether the feature is enabled.
    pub enabled: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PortalInvoiceList {
    /// Whether the feature is enabled.
    pub enabled: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PortalPaymentMethodUpdate {
    /// Whether the feature is enabled.
    pub enabled: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PortalSubscriptionCancel {
    /// Whether the feature is enabled.
    pub enabled: bool,

    /// Whether to cancel subscriptions immediately or at the end of the billing period.
    pub mode: PortalSubscriptionCancelMode,

    /// Whether to create prorations when canceling subscriptions.
    ///
    /// Possible values are `none` and `create_prorations`.
    pub proration_behavior: PortalSubscriptionCancelProrationBehavior,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PortalSubscriptionPause {
    /// Whether the feature is enabled.
    pub enabled: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PortalSubscriptionUpdate {
    /// The types of subscription updates that are supported for items listed in the `products` attribute.
    ///
    /// When empty, subscriptions are not updateable.
    pub default_allowed_updates: Vec<PortalSubscriptionUpdateDefaultAllowedUpdates>,

    /// Whether the feature is enabled.
    pub enabled: bool,

    /// The list of products that support subscription updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<PortalSubscriptionUpdateProduct>>,

    /// Determines how to handle prorations resulting from subscription updates.
    ///
    /// Valid values are `none`, `create_prorations`, and `always_invoice`.
    pub proration_behavior: PortalSubscriptionUpdateProrationBehavior,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
    Phone,
    Shipping,
    TaxId,
}

impl PortalCustomerUpdateAllowedUpdates {
    pub fn as_str(self) -> &'static str {
        match self {
            PortalCustomerUpdateAllowedUpdates::Address => "address",
            PortalCustomerUpdateAllowedUpdates::Email => "email",
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
