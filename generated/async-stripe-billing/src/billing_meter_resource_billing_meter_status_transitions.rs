#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingMeterResourceBillingMeterStatusTransitions {
    /// The time the meter was deactivated, if any. Measured in seconds since Unix epoch.
    pub deactivated_at: Option<stripe_types::Timestamp>,
}
#[doc(hidden)]
pub struct BillingMeterResourceBillingMeterStatusTransitionsBuilder {
    deactivated_at: Option<Option<stripe_types::Timestamp>>,
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

    impl Deserialize for BillingMeterResourceBillingMeterStatusTransitions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingMeterResourceBillingMeterStatusTransitions>,
        builder: BillingMeterResourceBillingMeterStatusTransitionsBuilder,
    }

    impl Visitor for Place<BillingMeterResourceBillingMeterStatusTransitions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BillingMeterResourceBillingMeterStatusTransitionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BillingMeterResourceBillingMeterStatusTransitionsBuilder {
        type Out = BillingMeterResourceBillingMeterStatusTransitions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "deactivated_at" => Deserialize::begin(&mut self.deactivated_at),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { deactivated_at: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(deactivated_at),) = (self.deactivated_at,) else {
                return None;
            };
            Some(Self::Out { deactivated_at })
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

    impl ObjectDeser for BillingMeterResourceBillingMeterStatusTransitions {
        type Builder = BillingMeterResourceBillingMeterStatusTransitionsBuilder;
    }

    impl FromValueOpt for BillingMeterResourceBillingMeterStatusTransitions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BillingMeterResourceBillingMeterStatusTransitionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "deactivated_at" => b.deactivated_at = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
