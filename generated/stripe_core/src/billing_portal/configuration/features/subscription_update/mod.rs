#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SubscriptionUpdate {
    /// The types of subscription updates that are supported for items listed in the `products` attribute.
    ///
    /// When empty, subscriptions are not updateable.
pub default_allowed_updates: Vec<SubscriptionUpdateDefaultAllowedUpdates>,
    /// Whether the feature is enabled.
pub enabled: bool,
    /// The list of products that support subscription updates.
pub products: Option<Vec<stripe_core::billing_portal::configuration::features::subscription_update::product::Product>>,
    /// Determines how to handle prorations resulting from subscription updates.
    ///
    /// Valid values are `none`, `create_prorations`, and `always_invoice`.
pub proration_behavior: SubscriptionUpdateProrationBehavior,

}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SubscriptionUpdate {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The types of subscription updates that are supported for items listed in the `products` attribute.
///
/// When empty, subscriptions are not updateable.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionUpdateDefaultAllowedUpdates {
    Price,
    PromotionCode,
    Quantity,
}

impl SubscriptionUpdateDefaultAllowedUpdates {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Price => "price",
            Self::PromotionCode => "promotion_code",
            Self::Quantity => "quantity",
        }
    }
}

impl AsRef<str> for SubscriptionUpdateDefaultAllowedUpdates {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionUpdateDefaultAllowedUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Determines how to handle prorations resulting from subscription updates.
///
/// Valid values are `none`, `create_prorations`, and `always_invoice`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionUpdateProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl SubscriptionUpdateProrationBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AlwaysInvoice => "always_invoice",
            Self::CreateProrations => "create_prorations",
            Self::None => "none",
        }
    }
}

impl AsRef<str> for SubscriptionUpdateProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionUpdateProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
pub mod product;
pub use product::Product;
