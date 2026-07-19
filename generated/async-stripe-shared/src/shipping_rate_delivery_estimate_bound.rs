#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ShippingRateDeliveryEstimateBound {
    /// A unit of time.
    pub unit: ShippingRateDeliveryEstimateBoundUnit,
    /// Must be greater than 0.
    pub value: i64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ShippingRateDeliveryEstimateBound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ShippingRateDeliveryEstimateBound").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ShippingRateDeliveryEstimateBoundBuilder {
    unit: Option<ShippingRateDeliveryEstimateBoundUnit>,
    value: Option<i64>,
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

    impl Deserialize for ShippingRateDeliveryEstimateBound {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ShippingRateDeliveryEstimateBound>,
        builder: ShippingRateDeliveryEstimateBoundBuilder,
    }

    impl Visitor for Place<ShippingRateDeliveryEstimateBound> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ShippingRateDeliveryEstimateBoundBuilder {
                    unit: Deserialize::default(),
                    value: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "unit" => Deserialize::begin(&mut self.builder.unit),
                "value" => Deserialize::begin(&mut self.builder.value),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(unit), Some(value)) = (self.builder.unit.take(), self.builder.value) else {
                return Ok(());
            };
            *self.out = Some(ShippingRateDeliveryEstimateBound { unit, value });
            Ok(())
        }
    }
};
/// A unit of time.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ShippingRateDeliveryEstimateBoundUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ShippingRateDeliveryEstimateBoundUnit {
    pub fn as_str(&self) -> &str {
        use ShippingRateDeliveryEstimateBoundUnit::*;
        match self {
            BusinessDay => "business_day",
            Day => "day",
            Hour => "hour",
            Month => "month",
            Week => "week",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ShippingRateDeliveryEstimateBoundUnit {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ShippingRateDeliveryEstimateBoundUnit::*;
        match s {
            "business_day" => Ok(BusinessDay),
            "day" => Ok(Day),
            "hour" => Ok(Hour),
            "month" => Ok(Month),
            "week" => Ok(Week),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "ShippingRateDeliveryEstimateBoundUnit"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ShippingRateDeliveryEstimateBoundUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ShippingRateDeliveryEstimateBoundUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ShippingRateDeliveryEstimateBoundUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ShippingRateDeliveryEstimateBoundUnit)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ShippingRateDeliveryEstimateBoundUnit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for ShippingRateDeliveryEstimateBoundUnit {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<ShippingRateDeliveryEstimateBoundUnit> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ShippingRateDeliveryEstimateBoundUnit::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ShippingRateDeliveryEstimateBoundUnit {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
