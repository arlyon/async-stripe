/// Subscription items allow you to create customer subscriptions with more than
/// one plan, making it easy to represent complex billing relationships.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SubscriptionItem {
    /// Define thresholds at which an invoice will be sent, and the related subscription advanced to a new billing period.
    pub billing_thresholds:
        Option<stripe_core::subscription_item::billing_thresholds::BillingThresholds>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: i64,
    /// Unique identifier for the object.
    pub id: stripe_core::subscription_item::SubscriptionItemId,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: SubscriptionItemObject,
    pub plan: stripe_core::plan::Plan,
    pub price: stripe_core::price::Price,
    /// The [quantity](https://stripe.com/docs/subscriptions/quantities) of the plan to which the customer should be subscribed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// The `subscription` this `subscription_item` belongs to.
    pub subscription: String,
    /// The tax rates which apply to this `subscription_item`.
    ///
    /// When set, the `default_tax_rates` on the subscription do not apply to this `subscription_item`.
    pub tax_rates: Option<Vec<stripe_core::tax_rate::TaxRate>>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SubscriptionItem {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SubscriptionItemObject {
    SubscriptionItem,
}

impl SubscriptionItemObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SubscriptionItem => "subscription_item",
        }
    }
}

impl std::str::FromStr for SubscriptionItemObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "subscription_item" => Ok(Self::SubscriptionItem),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for SubscriptionItemObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionItemObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for SubscriptionItemObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionItemObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for SubscriptionItemObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SubscriptionItemObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<SubscriptionItemObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SubscriptionItemObject::from_str(s)?);
        Ok(())
    }
}
impl stripe_types::Object for SubscriptionItem {
    type Id = stripe_core::subscription_item::SubscriptionItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(SubscriptionItemId, "si_");
pub mod deleted;
pub mod requests;
pub use deleted::DeletedSubscriptionItem;
pub mod billing_thresholds;
pub use billing_thresholds::BillingThresholds;
