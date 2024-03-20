/// Orders represent your intent to purchase a particular Climate product.
/// When you create an order, the.
/// payment is deducted from your merchant balance.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ClimateOrder {
    /// Total amount of [Frontier](https://frontierclimate.com/)'s service fees in the currency's smallest unit.
    pub amount_fees: i64,
    /// Total amount of the carbon removal in the currency's smallest unit.
    pub amount_subtotal: i64,
    /// Total amount of the order including fees in the currency's smallest unit.
    pub amount_total: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary: Option<stripe_misc::ClimateRemovalsBeneficiary>,
    /// Time at which the order was canceled. Measured in seconds since the Unix epoch.
    pub canceled_at: Option<stripe_types::Timestamp>,
    /// Reason for the cancellation of this order.
    pub cancellation_reason: Option<ClimateOrderCancellationReason>,
    /// For delivered orders, a URL to a delivery certificate for the order.
    pub certificate: Option<String>,
    /// Time at which the order was confirmed. Measured in seconds since the Unix epoch.
    pub confirmed_at: Option<stripe_types::Timestamp>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase, representing the currency for this order.
    pub currency: stripe_types::Currency,
    /// Time at which the order's expected_delivery_year was delayed.
    /// Measured in seconds since the Unix epoch.
    pub delayed_at: Option<stripe_types::Timestamp>,
    /// Time at which the order was delivered. Measured in seconds since the Unix epoch.
    pub delivered_at: Option<stripe_types::Timestamp>,
    /// Details about the delivery of carbon removal for this order.
    pub delivery_details: Vec<stripe_misc::ClimateRemovalsOrderDeliveries>,
    /// The year this order is expected to be delivered.
    pub expected_delivery_year: i64,
    /// Unique identifier for the object.
    pub id: stripe_misc::ClimateOrderId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// Quantity of carbon removal that is included in this order.
    pub metric_tons: String,
    /// Unique ID for the Climate `Product` this order is purchasing.
    pub product: stripe_types::Expandable<stripe_misc::ClimateProduct>,
    /// Time at which the order's product was substituted for a different product.
    /// Measured in seconds since the Unix epoch.
    pub product_substituted_at: Option<stripe_types::Timestamp>,
    /// The current status of this order.
    pub status: ClimateOrderStatus,
}
/// Reason for the cancellation of this order.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ClimateOrderCancellationReason {
    Expired,
    ProductUnavailable,
    Requested,
}
impl ClimateOrderCancellationReason {
    pub fn as_str(self) -> &'static str {
        use ClimateOrderCancellationReason::*;
        match self {
            Expired => "expired",
            ProductUnavailable => "product_unavailable",
            Requested => "requested",
        }
    }
}

impl std::str::FromStr for ClimateOrderCancellationReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ClimateOrderCancellationReason::*;
        match s {
            "expired" => Ok(Expired),
            "product_unavailable" => Ok(ProductUnavailable),
            "requested" => Ok(Requested),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for ClimateOrderCancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ClimateOrderCancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ClimateOrderCancellationReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ClimateOrderCancellationReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for ClimateOrderCancellationReason")
        })
    }
}
/// The current status of this order.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ClimateOrderStatus {
    AwaitingFunds,
    Canceled,
    Confirmed,
    Delivered,
    Open,
}
impl ClimateOrderStatus {
    pub fn as_str(self) -> &'static str {
        use ClimateOrderStatus::*;
        match self {
            AwaitingFunds => "awaiting_funds",
            Canceled => "canceled",
            Confirmed => "confirmed",
            Delivered => "delivered",
            Open => "open",
        }
    }
}

impl std::str::FromStr for ClimateOrderStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ClimateOrderStatus::*;
        match s {
            "awaiting_funds" => Ok(AwaitingFunds),
            "canceled" => Ok(Canceled),
            "confirmed" => Ok(Confirmed),
            "delivered" => Ok(Delivered),
            "open" => Ok(Open),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for ClimateOrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ClimateOrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ClimateOrderStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ClimateOrderStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ClimateOrderStatus"))
    }
}
impl stripe_types::Object for ClimateOrder {
    type Id = stripe_misc::ClimateOrderId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(ClimateOrderId);
