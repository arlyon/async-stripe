#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingMeterResourceBillingMeterEventAdjustmentCancel {
    /// Unique identifier for the event.
    pub identifier: Option<String>,
}
#[doc(hidden)]
pub struct BillingMeterResourceBillingMeterEventAdjustmentCancelBuilder {
    identifier: Option<Option<String>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for BillingMeterResourceBillingMeterEventAdjustmentCancel {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingMeterResourceBillingMeterEventAdjustmentCancel>,
        builder: BillingMeterResourceBillingMeterEventAdjustmentCancelBuilder,
    }

    impl Visitor for Place<BillingMeterResourceBillingMeterEventAdjustmentCancel> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    BillingMeterResourceBillingMeterEventAdjustmentCancelBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BillingMeterResourceBillingMeterEventAdjustmentCancelBuilder {
        type Out = BillingMeterResourceBillingMeterEventAdjustmentCancel;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "identifier" => Deserialize::begin(&mut self.identifier),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { identifier: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(identifier),) = (self.identifier.take(),) else {
                return None;
            };
            Some(Self::Out { identifier })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for BillingMeterResourceBillingMeterEventAdjustmentCancel {
        type Builder = BillingMeterResourceBillingMeterEventAdjustmentCancelBuilder;
    }

    impl FromValueOpt for BillingMeterResourceBillingMeterEventAdjustmentCancel {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                BillingMeterResourceBillingMeterEventAdjustmentCancelBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "identifier" => b.identifier = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
