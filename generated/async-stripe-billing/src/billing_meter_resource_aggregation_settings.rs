#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingMeterResourceAggregationSettings {
    /// Specifies how events are aggregated.
    pub formula: BillingMeterResourceAggregationSettingsFormula,
}
#[doc(hidden)]
pub struct BillingMeterResourceAggregationSettingsBuilder {
    formula: Option<BillingMeterResourceAggregationSettingsFormula>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: BillingMeterResourceAggregationSettingsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BillingMeterResourceAggregationSettingsBuilder {
        type Out = BillingMeterResourceAggregationSettings;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "formula" => Deserialize::begin(&mut self.formula),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { formula: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(formula),) = (self.formula,) else {
                return None;
            };
            Some(Self::Out { formula })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for BillingMeterResourceAggregationSettings {
        type Builder = BillingMeterResourceAggregationSettingsBuilder;
    }

    impl FromValueOpt for BillingMeterResourceAggregationSettings {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BillingMeterResourceAggregationSettingsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "formula" => b.formula = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Specifies how events are aggregated.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BillingMeterResourceAggregationSettingsFormula {
    Count,
    Last,
    Sum,
}
impl BillingMeterResourceAggregationSettingsFormula {
    pub fn as_str(self) -> &'static str {
        use BillingMeterResourceAggregationSettingsFormula::*;
        match self {
            Count => "count",
            Last => "last",
            Sum => "sum",
        }
    }
}

impl std::str::FromStr for BillingMeterResourceAggregationSettingsFormula {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingMeterResourceAggregationSettingsFormula::*;
        match s {
            "count" => Ok(Count),
            "last" => Ok(Last),
            "sum" => Ok(Sum),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for BillingMeterResourceAggregationSettingsFormula {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BillingMeterResourceAggregationSettingsFormula {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for BillingMeterResourceAggregationSettingsFormula {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<BillingMeterResourceAggregationSettingsFormula> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            BillingMeterResourceAggregationSettingsFormula::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(BillingMeterResourceAggregationSettingsFormula);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingMeterResourceAggregationSettingsFormula {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for BillingMeterResourceAggregationSettingsFormula",
            )
        })
    }
}
