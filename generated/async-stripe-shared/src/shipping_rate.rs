/// Shipping rates describe the price of shipping presented to your customers and
/// applied to a purchase.
/// For more information, see [Charge for shipping](https://docs.stripe.com/payments/during-payment/charge-shipping).
///
/// For more details see <<https://stripe.com/docs/api/shipping_rates/object>>.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ShippingRate {
    /// Whether the shipping rate can be used for new purchases. Defaults to `true`.
    pub active: bool,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The estimated range for how long shipping will take, meant to be displayable to the customer.
    /// This will appear on CheckoutSessions.
    pub delivery_estimate: Option<stripe_shared::ShippingRateDeliveryEstimate>,
    /// The name of the shipping rate, meant to be displayable to the customer.
    /// This will appear on CheckoutSessions.
    pub display_name: Option<String>,
    pub fixed_amount: Option<stripe_shared::ShippingRateFixedAmount>,
    /// Unique identifier for the object.
    pub id: stripe_shared::ShippingRateId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    pub tax_behavior: Option<stripe_shared::ShippingRateTaxBehavior>,
    /// A [tax code](https://docs.stripe.com/tax/tax-categories) ID.
    /// The Shipping tax code is `txcd_92010001`.
    pub tax_code: Option<stripe_types::Expandable<stripe_shared::TaxCode>>,
    /// The type of calculation to use on the shipping rate.
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: stripe_shared::ShippingRateType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ShippingRate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ShippingRate").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ShippingRateBuilder {
    active: Option<bool>,
    created: Option<stripe_types::Timestamp>,
    delivery_estimate: Option<Option<stripe_shared::ShippingRateDeliveryEstimate>>,
    display_name: Option<Option<String>>,
    fixed_amount: Option<Option<stripe_shared::ShippingRateFixedAmount>>,
    id: Option<stripe_shared::ShippingRateId>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    tax_behavior: Option<Option<stripe_shared::ShippingRateTaxBehavior>>,
    tax_code: Option<Option<stripe_types::Expandable<stripe_shared::TaxCode>>>,
    type_: Option<stripe_shared::ShippingRateType>,
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

    impl Deserialize for ShippingRate {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ShippingRate>,
        builder: ShippingRateBuilder,
    }

    impl Visitor for Place<ShippingRate> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ShippingRateBuilder {
                    active: Deserialize::default(),
                    created: Deserialize::default(),
                    delivery_estimate: Deserialize::default(),
                    display_name: Deserialize::default(),
                    fixed_amount: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                    metadata: Deserialize::default(),
                    tax_behavior: Deserialize::default(),
                    tax_code: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "active" => Deserialize::begin(&mut self.builder.active),
                "created" => Deserialize::begin(&mut self.builder.created),
                "delivery_estimate" => Deserialize::begin(&mut self.builder.delivery_estimate),
                "display_name" => Deserialize::begin(&mut self.builder.display_name),
                "fixed_amount" => Deserialize::begin(&mut self.builder.fixed_amount),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "tax_behavior" => Deserialize::begin(&mut self.builder.tax_behavior),
                "tax_code" => Deserialize::begin(&mut self.builder.tax_code),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(active),
                Some(created),
                Some(delivery_estimate),
                Some(display_name),
                Some(fixed_amount),
                Some(id),
                Some(livemode),
                Some(metadata),
                Some(tax_behavior),
                Some(tax_code),
                Some(type_),
            ) = (
                self.builder.active,
                self.builder.created,
                self.builder.delivery_estimate.take(),
                self.builder.display_name.take(),
                self.builder.fixed_amount.take(),
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.metadata.take(),
                self.builder.tax_behavior.take(),
                self.builder.tax_code.take(),
                self.builder.type_.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(ShippingRate {
                active,
                created,
                delivery_estimate,
                display_name,
                fixed_amount,
                id,
                livemode,
                metadata,
                tax_behavior,
                tax_code,
                type_,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for ShippingRate {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("ShippingRate", 12)?;
        s.serialize_field("active", &self.active)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("delivery_estimate", &self.delivery_estimate)?;
        s.serialize_field("display_name", &self.display_name)?;
        s.serialize_field("fixed_amount", &self.fixed_amount)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("tax_behavior", &self.tax_behavior)?;
        s.serialize_field("tax_code", &self.tax_code)?;
        s.serialize_field("type", &self.type_)?;

        s.serialize_field("object", "shipping_rate")?;
        s.end()
    }
}
impl stripe_types::Object for ShippingRate {
    type Id = stripe_shared::ShippingRateId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(ShippingRateId);
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ShippingRateTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ShippingRateTaxBehavior {
    pub fn as_str(&self) -> &str {
        use ShippingRateTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ShippingRateTaxBehavior {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ShippingRateTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "ShippingRateTaxBehavior");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ShippingRateTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ShippingRateTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ShippingRateTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ShippingRateTaxBehavior)).finish_non_exhaustive()
    }
}
impl serde::Serialize for ShippingRateTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for ShippingRateTaxBehavior {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<ShippingRateTaxBehavior> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ShippingRateTaxBehavior::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ShippingRateTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ShippingRateType {
    FixedAmount,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ShippingRateType {
    pub fn as_str(&self) -> &str {
        use ShippingRateType::*;
        match self {
            FixedAmount => "fixed_amount",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ShippingRateType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ShippingRateType::*;
        match s {
            "fixed_amount" => Ok(FixedAmount),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "ShippingRateType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ShippingRateType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ShippingRateType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ShippingRateType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ShippingRateType)).finish_non_exhaustive()
    }
}
impl serde::Serialize for ShippingRateType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for ShippingRateType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<ShippingRateType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ShippingRateType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ShippingRateType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
