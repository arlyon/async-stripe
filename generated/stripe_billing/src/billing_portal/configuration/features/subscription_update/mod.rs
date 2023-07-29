#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionUpdate {
    /// The types of subscription updates that are supported for items listed in the `products` attribute.
    ///
    /// When empty, subscriptions are not updateable.
pub default_allowed_updates: Vec<SubscriptionUpdateDefaultAllowedUpdates>,
    /// Whether the feature is enabled.
pub enabled: bool,
    /// The list of products that support subscription updates.
pub products: Option<Vec<stripe_billing::billing_portal::configuration::features::subscription_update::product::Product>>,
    /// Determines how to handle prorations resulting from subscription updates.
    ///
    /// Valid values are `none`, `create_prorations`, and `always_invoice`.
pub proration_behavior: SubscriptionUpdateProrationBehavior,

}
/// The types of subscription updates that are supported for items listed in the `products` attribute.
///
/// When empty, subscriptions are not updateable.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for SubscriptionUpdateDefaultAllowedUpdates {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "price" => Ok(Self::Price),
            "promotion_code" => Ok(Self::PromotionCode),
            "quantity" => Ok(Self::Quantity),

            _ => Err(()),
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
impl serde::Serialize for SubscriptionUpdateDefaultAllowedUpdates {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionUpdateDefaultAllowedUpdates {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for SubscriptionUpdateDefaultAllowedUpdates")
        })
    }
}
/// Determines how to handle prorations resulting from subscription updates.
///
/// Valid values are `none`, `create_prorations`, and `always_invoice`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for SubscriptionUpdateProrationBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "always_invoice" => Ok(Self::AlwaysInvoice),
            "create_prorations" => Ok(Self::CreateProrations),
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for SubscriptionUpdateProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionUpdateProrationBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for SubscriptionUpdateProrationBehavior")
        })
    }
}
pub mod product;
pub use product::Product;
