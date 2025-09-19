#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsPaymentIntentPresentmentDetails {
    /// Amount intended to be collected by this payment, denominated in `presentment_currency`.
    pub presentment_amount: i64,
    /// Currency presented to the customer during payment.
    pub presentment_currency: stripe_types::Currency,
}
#[doc(hidden)]
pub struct PaymentFlowsPaymentIntentPresentmentDetailsBuilder {
    presentment_amount: Option<i64>,
    presentment_currency: Option<stripe_types::Currency>,
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

    impl Deserialize for PaymentFlowsPaymentIntentPresentmentDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentFlowsPaymentIntentPresentmentDetails>,
        builder: PaymentFlowsPaymentIntentPresentmentDetailsBuilder,
    }

    impl Visitor for Place<PaymentFlowsPaymentIntentPresentmentDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentFlowsPaymentIntentPresentmentDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentFlowsPaymentIntentPresentmentDetailsBuilder {
        type Out = PaymentFlowsPaymentIntentPresentmentDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "presentment_amount" => Deserialize::begin(&mut self.presentment_amount),
                "presentment_currency" => Deserialize::begin(&mut self.presentment_currency),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                presentment_amount: Deserialize::default(),
                presentment_currency: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(presentment_amount), Some(presentment_currency)) =
                (self.presentment_amount, self.presentment_currency.take())
            else {
                return None;
            };
            Some(Self::Out { presentment_amount, presentment_currency })
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

    impl ObjectDeser for PaymentFlowsPaymentIntentPresentmentDetails {
        type Builder = PaymentFlowsPaymentIntentPresentmentDetailsBuilder;
    }

    impl FromValueOpt for PaymentFlowsPaymentIntentPresentmentDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentFlowsPaymentIntentPresentmentDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "presentment_amount" => b.presentment_amount = FromValueOpt::from_value(v),
                    "presentment_currency" => b.presentment_currency = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
