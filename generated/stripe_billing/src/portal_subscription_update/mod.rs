#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PortalSubscriptionUpdate {
    /// The types of subscription updates that are supported for items listed in the `products` attribute.
    ///
    /// When empty, subscriptions are not updateable.
    pub default_allowed_updates: Vec<PortalSubscriptionUpdateDefaultAllowedUpdates>,
    /// Whether the feature is enabled.
    pub enabled: bool,
    /// The list of products that support subscription updates.
    pub products: Option<Vec<stripe_billing::PortalSubscriptionUpdateProduct>>,
    /// Determines how to handle prorations resulting from subscription updates.
    ///
    /// Valid values are `none`, `create_prorations`, and `always_invoice`.
    pub proration_behavior: PortalSubscriptionUpdateProrationBehavior,
}
/// The types of subscription updates that are supported for items listed in the `products` attribute.
///
/// When empty, subscriptions are not updateable.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PortalSubscriptionUpdateDefaultAllowedUpdates {
    Price,
    PromotionCode,
    Quantity,
}

impl PortalSubscriptionUpdateDefaultAllowedUpdates {
    pub fn as_str(self) -> &'static str {
        use PortalSubscriptionUpdateDefaultAllowedUpdates::*;
        match self {
            Price => "price",
            PromotionCode => "promotion_code",
            Quantity => "quantity",
        }
    }
}

impl std::str::FromStr for PortalSubscriptionUpdateDefaultAllowedUpdates {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PortalSubscriptionUpdateDefaultAllowedUpdates::*;
        match s {
            "price" => Ok(Price),
            "promotion_code" => Ok(PromotionCode),
            "quantity" => Ok(Quantity),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PortalSubscriptionUpdateDefaultAllowedUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PortalSubscriptionUpdateDefaultAllowedUpdates {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PortalSubscriptionUpdateDefaultAllowedUpdates {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PortalSubscriptionUpdateDefaultAllowedUpdates",
            )
        })
    }
}
/// Determines how to handle prorations resulting from subscription updates.
///
/// Valid values are `none`, `create_prorations`, and `always_invoice`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PortalSubscriptionUpdateProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl PortalSubscriptionUpdateProrationBehavior {
    pub fn as_str(self) -> &'static str {
        use PortalSubscriptionUpdateProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
        }
    }
}

impl std::str::FromStr for PortalSubscriptionUpdateProrationBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PortalSubscriptionUpdateProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PortalSubscriptionUpdateProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PortalSubscriptionUpdateProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PortalSubscriptionUpdateProrationBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PortalSubscriptionUpdateProrationBehavior")
        })
    }
}
