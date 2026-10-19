#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingMeterResourceAggregationSettings {
    /// Specifies how events are aggregated.
    pub formula: stripe_shared::Formula,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingMeterResourceAggregationSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingMeterResourceAggregationSettings").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BillingMeterResourceAggregationSettingsBuilder {
    formula: Option<stripe_shared::Formula>,
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
            Self { formula: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(formula),) = (self.formula.take(),) else {
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
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
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
pub type BillingMeterResourceAggregationSettingsFormula = stripe_shared::Formula;
