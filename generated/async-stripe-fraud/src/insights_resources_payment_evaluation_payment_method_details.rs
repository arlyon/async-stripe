/// Payment method details attached to this payment evaluation.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InsightsResourcesPaymentEvaluationPaymentMethodDetails {
    /// Billing information associated with the payment evaluation.
    pub billing_details: Option<stripe_fraud::InsightsResourcesPaymentEvaluationBillingDetails>,
    /// The payment method used in this payment evaluation.
    pub payment_method: stripe_types::Expandable<stripe_shared::PaymentMethod>,
}
#[doc(hidden)]
pub struct InsightsResourcesPaymentEvaluationPaymentMethodDetailsBuilder {
    billing_details: Option<Option<stripe_fraud::InsightsResourcesPaymentEvaluationBillingDetails>>,
    payment_method: Option<stripe_types::Expandable<stripe_shared::PaymentMethod>>,
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

    impl Deserialize for InsightsResourcesPaymentEvaluationPaymentMethodDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InsightsResourcesPaymentEvaluationPaymentMethodDetails>,
        builder: InsightsResourcesPaymentEvaluationPaymentMethodDetailsBuilder,
    }

    impl Visitor for Place<InsightsResourcesPaymentEvaluationPaymentMethodDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    InsightsResourcesPaymentEvaluationPaymentMethodDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InsightsResourcesPaymentEvaluationPaymentMethodDetailsBuilder {
        type Out = InsightsResourcesPaymentEvaluationPaymentMethodDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "billing_details" => Deserialize::begin(&mut self.billing_details),
                "payment_method" => Deserialize::begin(&mut self.payment_method),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { billing_details: Deserialize::default(), payment_method: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(billing_details), Some(payment_method)) =
                (self.billing_details.take(), self.payment_method.take())
            else {
                return None;
            };
            Some(Self::Out { billing_details, payment_method })
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

    impl ObjectDeser for InsightsResourcesPaymentEvaluationPaymentMethodDetails {
        type Builder = InsightsResourcesPaymentEvaluationPaymentMethodDetailsBuilder;
    }

    impl FromValueOpt for InsightsResourcesPaymentEvaluationPaymentMethodDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                InsightsResourcesPaymentEvaluationPaymentMethodDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "billing_details" => b.billing_details = FromValueOpt::from_value(v),
                    "payment_method" => b.payment_method = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
