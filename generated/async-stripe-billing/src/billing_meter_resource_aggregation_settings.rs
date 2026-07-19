#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingMeterResourceAggregationSettings {
    /// Specifies how events are aggregated.
    pub formula: BillingMeterResourceAggregationSettingsFormula,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingMeterResourceAggregationSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingMeterResourceAggregationSettings").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BillingMeterResourceAggregationSettingsBuilder {
    formula: Option<BillingMeterResourceAggregationSettingsFormula>,
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

    impl Deserialize for BillingMeterResourceAggregationSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingMeterResourceAggregationSettings>,
        builder: BillingMeterResourceAggregationSettingsBuilder,
    }

    impl Visitor for Place<BillingMeterResourceAggregationSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BillingMeterResourceAggregationSettingsBuilder {
                    formula: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "formula" => Deserialize::begin(&mut self.builder.formula),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(formula),) = (self.builder.formula.take(),) else {
                return Ok(());
            };
            *self.out = Some(BillingMeterResourceAggregationSettings { formula });
            Ok(())
        }
    }
};
/// Specifies how events are aggregated.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum BillingMeterResourceAggregationSettingsFormula {
    Count,
    Last,
    Sum,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl BillingMeterResourceAggregationSettingsFormula {
    pub fn as_str(&self) -> &str {
        use BillingMeterResourceAggregationSettingsFormula::*;
        match self {
            Count => "count",
            Last => "last",
            Sum => "sum",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for BillingMeterResourceAggregationSettingsFormula {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingMeterResourceAggregationSettingsFormula::*;
        match s {
            "count" => Ok(Count),
            "last" => Ok(Last),
            "sum" => Ok(Sum),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "BillingMeterResourceAggregationSettingsFormula"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for BillingMeterResourceAggregationSettingsFormula {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for BillingMeterResourceAggregationSettingsFormula {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingMeterResourceAggregationSettingsFormula {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(BillingMeterResourceAggregationSettingsFormula))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for BillingMeterResourceAggregationSettingsFormula {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for BillingMeterResourceAggregationSettingsFormula {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<BillingMeterResourceAggregationSettingsFormula> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(BillingMeterResourceAggregationSettingsFormula::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingMeterResourceAggregationSettingsFormula {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
