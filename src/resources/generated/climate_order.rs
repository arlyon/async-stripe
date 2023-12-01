// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{ClimateOrderId};
use crate::params::{Expandable, Metadata, Object, Timestamp};
use crate::resources::{ClimateProduct, ClimateSupplier, Currency};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "ClimateRemovalsOrders".
///
/// For more details see <https://stripe.com/docs/api/climate/order/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ClimateOrder {
    /// Unique identifier for the object.
    pub id: ClimateOrderId,

    /// Total amount of [Frontier](https://frontierclimate.com/)'s service fees in the currency's smallest unit.
    pub amount_fees: i64,

    /// Total amount of the carbon removal in the currency's smallest unit.
    pub amount_subtotal: i64,

    /// Total amount of the order including fees in the currency's smallest unit.
    pub amount_total: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary: Option<ClimateRemovalsBeneficiary>,

    /// Time at which the order was canceled.
    ///
    /// Measured in seconds since the Unix epoch.
    pub canceled_at: Option<Timestamp>,

    /// Reason for the cancellation of this order.
    pub cancellation_reason: Option<ClimateOrderCancellationReason>,

    /// For delivered orders, a URL to a delivery certificate for the order.
    pub certificate: Option<String>,

    /// Time at which the order was confirmed.
    ///
    /// Measured in seconds since the Unix epoch.
    pub confirmed_at: Option<Timestamp>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase, representing the currency for this order.
    pub currency: Currency,

    /// Time at which the order's expected_delivery_year was delayed.
    ///
    /// Measured in seconds since the Unix epoch.
    pub delayed_at: Option<Timestamp>,

    /// Time at which the order was delivered.
    ///
    /// Measured in seconds since the Unix epoch.
    pub delivered_at: Option<Timestamp>,

    /// Details about the delivery of carbon removal for this order.
    pub delivery_details: Vec<ClimateRemovalsOrderDeliveries>,

    /// The year this order is expected to be delivered.
    pub expected_delivery_year: i64,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// Quantity of carbon removal that is included in this order.
    pub metric_tons: String,

    /// Unique ID for the Climate `Product` this order is purchasing.
    pub product: Expandable<ClimateProduct>,

    /// Time at which the order's product was substituted for a different product.
    ///
    /// Measured in seconds since the Unix epoch.
    pub product_substituted_at: Option<Timestamp>,

    /// The current status of this order.
    pub status: ClimateOrderStatus,
}

impl Object for ClimateOrder {
    type Id = ClimateOrderId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "climate.order"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ClimateRemovalsBeneficiary {

    /// Publicly displayable name for the end beneficiary of carbon removal.
    pub public_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ClimateRemovalsOrderDeliveries {

    /// Time at which the delivery occurred.
    ///
    /// Measured in seconds since the Unix epoch.
    pub delivered_at: Timestamp,

    /// Specific location of this delivery.
    pub location: Option<ClimateRemovalsLocation>,

    /// Quantity of carbon removal supplied by this delivery.
    pub metric_tons: String,

    /// Once retired, a URL to the registry entry for the tons from this delivery.
    pub registry_url: Option<String>,

    pub supplier: ClimateSupplier,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ClimateRemovalsLocation {

    /// The city where the supplier is located.
    pub city: Option<String>,

    /// Two-letter ISO code representing the country where the supplier is located.
    pub country: String,

    /// The geographic latitude where the supplier is located.
    pub latitude: Option<f64>,

    /// The geographic longitude where the supplier is located.
    pub longitude: Option<f64>,

    /// The state/county/province/region where the supplier is located.
    pub region: Option<String>,
}

/// An enum representing the possible values of an `ClimateOrder`'s `cancellation_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ClimateOrderCancellationReason {
    Expired,
    ProductUnavailable,
    Requested,
}

impl ClimateOrderCancellationReason {
    pub fn as_str(self) -> &'static str {
        match self {
            ClimateOrderCancellationReason::Expired => "expired",
            ClimateOrderCancellationReason::ProductUnavailable => "product_unavailable",
            ClimateOrderCancellationReason::Requested => "requested",
        }
    }
}

impl AsRef<str> for ClimateOrderCancellationReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ClimateOrderCancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ClimateOrderCancellationReason {
    fn default() -> Self {
        Self::Expired
    }
}

/// An enum representing the possible values of an `ClimateOrder`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ClimateOrderStatus {
    AwaitingFunds,
    Canceled,
    Confirmed,
    Delivered,
    Open,
}

impl ClimateOrderStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            ClimateOrderStatus::AwaitingFunds => "awaiting_funds",
            ClimateOrderStatus::Canceled => "canceled",
            ClimateOrderStatus::Confirmed => "confirmed",
            ClimateOrderStatus::Delivered => "delivered",
            ClimateOrderStatus::Open => "open",
        }
    }
}

impl AsRef<str> for ClimateOrderStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ClimateOrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ClimateOrderStatus {
    fn default() -> Self {
        Self::AwaitingFunds
    }
}
