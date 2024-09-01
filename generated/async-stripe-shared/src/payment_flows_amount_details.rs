#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsAmountDetails {
    pub tip: Option<stripe_shared::PaymentFlowsAmountDetailsResourceTip>,
}
#[doc(hidden)]
pub struct PaymentFlowsAmountDetailsBuilder {
    tip: Option<Option<stripe_shared::PaymentFlowsAmountDetailsResourceTip>>,
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

    impl Deserialize for PaymentFlowsAmountDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentFlowsAmountDetails>,
        builder: PaymentFlowsAmountDetailsBuilder,
    }

    impl Visitor for Place<PaymentFlowsAmountDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentFlowsAmountDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentFlowsAmountDetailsBuilder {
        type Out = PaymentFlowsAmountDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "tip" => Deserialize::begin(&mut self.tip),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { tip: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(tip),) = (self.tip,) else {
                return None;
            };
            Some(Self::Out { tip })
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

    impl ObjectDeser for PaymentFlowsAmountDetails {
        type Builder = PaymentFlowsAmountDetailsBuilder;
    }

    impl FromValueOpt for PaymentFlowsAmountDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentFlowsAmountDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "tip" => b.tip = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
