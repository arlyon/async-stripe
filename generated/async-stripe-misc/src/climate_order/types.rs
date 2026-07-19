/// Orders represent your intent to purchase a particular Climate product.
/// When you create an order, the.
/// payment is deducted from your merchant balance.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ClimateOrder {
    /// Total amount of [Frontier](https://frontierclimate.com/)'s service fees in the currency's smallest unit.
    pub amount_fees: i64,
    /// Total amount of the carbon removal in the currency's smallest unit.
    pub amount_subtotal: i64,
    /// Total amount of the order including fees in the currency's smallest unit.
    pub amount_total: i64,
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
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ClimateOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ClimateOrder").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ClimateOrderBuilder {
    amount_fees: Option<i64>,
    amount_subtotal: Option<i64>,
    amount_total: Option<i64>,
    beneficiary: Option<Option<stripe_misc::ClimateRemovalsBeneficiary>>,
    canceled_at: Option<Option<stripe_types::Timestamp>>,
    cancellation_reason: Option<Option<ClimateOrderCancellationReason>>,
    certificate: Option<Option<String>>,
    confirmed_at: Option<Option<stripe_types::Timestamp>>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    delayed_at: Option<Option<stripe_types::Timestamp>>,
    delivered_at: Option<Option<stripe_types::Timestamp>>,
    delivery_details: Option<Vec<stripe_misc::ClimateRemovalsOrderDeliveries>>,
    expected_delivery_year: Option<i64>,
    id: Option<stripe_misc::ClimateOrderId>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    metric_tons: Option<String>,
    product: Option<stripe_types::Expandable<stripe_misc::ClimateProduct>>,
    product_substituted_at: Option<Option<stripe_types::Timestamp>>,
    status: Option<ClimateOrderStatus>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

    make_place!(Place);

    impl Deserialize for ClimateOrder {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ClimateOrder>,
        builder: ClimateOrderBuilder,
    }

    impl Visitor for Place<ClimateOrder> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ClimateOrderBuilder {
                    amount_fees: Deserialize::default(),
                    amount_subtotal: Deserialize::default(),
                    amount_total: Deserialize::default(),
                    beneficiary: Deserialize::default(),
                    canceled_at: Deserialize::default(),
                    cancellation_reason: Deserialize::default(),
                    certificate: Deserialize::default(),
                    confirmed_at: Deserialize::default(),
                    created: Deserialize::default(),
                    currency: Deserialize::default(),
                    delayed_at: Deserialize::default(),
                    delivered_at: Deserialize::default(),
                    delivery_details: Deserialize::default(),
                    expected_delivery_year: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                    metadata: Deserialize::default(),
                    metric_tons: Deserialize::default(),
                    product: Deserialize::default(),
                    product_substituted_at: Deserialize::default(),
                    status: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_fees" => Deserialize::begin(&mut self.builder.amount_fees),
                "amount_subtotal" => Deserialize::begin(&mut self.builder.amount_subtotal),
                "amount_total" => Deserialize::begin(&mut self.builder.amount_total),
                "beneficiary" => Deserialize::begin(&mut self.builder.beneficiary),
                "canceled_at" => Deserialize::begin(&mut self.builder.canceled_at),
                "cancellation_reason" => Deserialize::begin(&mut self.builder.cancellation_reason),
                "certificate" => Deserialize::begin(&mut self.builder.certificate),
                "confirmed_at" => Deserialize::begin(&mut self.builder.confirmed_at),
                "created" => Deserialize::begin(&mut self.builder.created),
                "currency" => Deserialize::begin(&mut self.builder.currency),
                "delayed_at" => Deserialize::begin(&mut self.builder.delayed_at),
                "delivered_at" => Deserialize::begin(&mut self.builder.delivered_at),
                "delivery_details" => Deserialize::begin(&mut self.builder.delivery_details),
                "expected_delivery_year" => {
                    Deserialize::begin(&mut self.builder.expected_delivery_year)
                }
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "metric_tons" => Deserialize::begin(&mut self.builder.metric_tons),
                "product" => Deserialize::begin(&mut self.builder.product),
                "product_substituted_at" => {
                    Deserialize::begin(&mut self.builder.product_substituted_at)
                }
                "status" => Deserialize::begin(&mut self.builder.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(amount_fees),
                Some(amount_subtotal),
                Some(amount_total),
                Some(beneficiary),
                Some(canceled_at),
                Some(cancellation_reason),
                Some(certificate),
                Some(confirmed_at),
                Some(created),
                Some(currency),
                Some(delayed_at),
                Some(delivered_at),
                Some(delivery_details),
                Some(expected_delivery_year),
                Some(id),
                Some(livemode),
                Some(metadata),
                Some(metric_tons),
                Some(product),
                Some(product_substituted_at),
                Some(status),
            ) = (
                self.builder.amount_fees,
                self.builder.amount_subtotal,
                self.builder.amount_total,
                self.builder.beneficiary.take(),
                self.builder.canceled_at,
                self.builder.cancellation_reason.take(),
                self.builder.certificate.take(),
                self.builder.confirmed_at,
                self.builder.created,
                self.builder.currency.take(),
                self.builder.delayed_at,
                self.builder.delivered_at,
                self.builder.delivery_details.take(),
                self.builder.expected_delivery_year,
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.metadata.take(),
                self.builder.metric_tons.take(),
                self.builder.product.take(),
                self.builder.product_substituted_at,
                self.builder.status.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(ClimateOrder {
                amount_fees,
                amount_subtotal,
                amount_total,
                beneficiary,
                canceled_at,
                cancellation_reason,
                certificate,
                confirmed_at,
                created,
                currency,
                delayed_at,
                delivered_at,
                delivery_details,
                expected_delivery_year,
                id,
                livemode,
                metadata,
                metric_tons,
                product,
                product_substituted_at,
                status,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for ClimateOrder {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("ClimateOrder", 22)?;
        s.serialize_field("amount_fees", &self.amount_fees)?;
        s.serialize_field("amount_subtotal", &self.amount_subtotal)?;
        s.serialize_field("amount_total", &self.amount_total)?;
        s.serialize_field("beneficiary", &self.beneficiary)?;
        s.serialize_field("canceled_at", &self.canceled_at)?;
        s.serialize_field("cancellation_reason", &self.cancellation_reason)?;
        s.serialize_field("certificate", &self.certificate)?;
        s.serialize_field("confirmed_at", &self.confirmed_at)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("delayed_at", &self.delayed_at)?;
        s.serialize_field("delivered_at", &self.delivered_at)?;
        s.serialize_field("delivery_details", &self.delivery_details)?;
        s.serialize_field("expected_delivery_year", &self.expected_delivery_year)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("metric_tons", &self.metric_tons)?;
        s.serialize_field("product", &self.product)?;
        s.serialize_field("product_substituted_at", &self.product_substituted_at)?;
        s.serialize_field("status", &self.status)?;

        s.serialize_field("object", "climate.order")?;
        s.end()
    }
}
/// Reason for the cancellation of this order.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ClimateOrderCancellationReason {
    Expired,
    ProductUnavailable,
    Requested,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ClimateOrderCancellationReason {
    pub fn as_str(&self) -> &str {
        use ClimateOrderCancellationReason::*;
        match self {
            Expired => "expired",
            ProductUnavailable => "product_unavailable",
            Requested => "requested",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ClimateOrderCancellationReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ClimateOrderCancellationReason::*;
        match s {
            "expired" => Ok(Expired),
            "product_unavailable" => Ok(ProductUnavailable),
            "requested" => Ok(Requested),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "ClimateOrderCancellationReason"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ClimateOrderCancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ClimateOrderCancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ClimateOrderCancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ClimateOrderCancellationReason)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ClimateOrderCancellationReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for ClimateOrderCancellationReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<ClimateOrderCancellationReason> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ClimateOrderCancellationReason::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ClimateOrderCancellationReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The current status of this order.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ClimateOrderStatus {
    AwaitingFunds,
    Canceled,
    Confirmed,
    Delivered,
    Open,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ClimateOrderStatus {
    pub fn as_str(&self) -> &str {
        use ClimateOrderStatus::*;
        match self {
            AwaitingFunds => "awaiting_funds",
            Canceled => "canceled",
            Confirmed => "confirmed",
            Delivered => "delivered",
            Open => "open",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ClimateOrderStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ClimateOrderStatus::*;
        match s {
            "awaiting_funds" => Ok(AwaitingFunds),
            "canceled" => Ok(Canceled),
            "confirmed" => Ok(Confirmed),
            "delivered" => Ok(Delivered),
            "open" => Ok(Open),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "ClimateOrderStatus");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ClimateOrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ClimateOrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ClimateOrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ClimateOrderStatus)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ClimateOrderStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for ClimateOrderStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<ClimateOrderStatus> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ClimateOrderStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ClimateOrderStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
impl stripe_types::Object for ClimateOrder {
    type Id = stripe_misc::ClimateOrderId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(ClimateOrderId);
