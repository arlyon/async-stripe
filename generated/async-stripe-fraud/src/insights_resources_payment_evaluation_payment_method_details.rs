/// Payment method details attached to this payment evaluation.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InsightsResourcesPaymentEvaluationPaymentMethodDetails {
    /// Billing information associated with the payment evaluation.
    pub billing_details: Option<stripe_fraud::InsightsResourcesPaymentEvaluationBillingDetails>,
    /// The payment method used in this payment evaluation.
    pub payment_method: stripe_types::Expandable<stripe_shared::PaymentMethod>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InsightsResourcesPaymentEvaluationPaymentMethodDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InsightsResourcesPaymentEvaluationPaymentMethodDetails")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InsightsResourcesPaymentEvaluationPaymentMethodDetailsBuilder {
    billing_details: Option<Option<stripe_fraud::InsightsResourcesPaymentEvaluationBillingDetails>>,
    payment_method: Option<stripe_types::Expandable<stripe_shared::PaymentMethod>>,
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
                builder: InsightsResourcesPaymentEvaluationPaymentMethodDetailsBuilder {
                    billing_details: Deserialize::default(),
                    payment_method: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "billing_details" => Deserialize::begin(&mut self.builder.billing_details),
                "payment_method" => Deserialize::begin(&mut self.builder.payment_method),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(billing_details), Some(payment_method)) =
                (self.builder.billing_details.take(), self.builder.payment_method.take())
            else {
                return Ok(());
            };
            *self.out = Some(InsightsResourcesPaymentEvaluationPaymentMethodDetails {
                billing_details,
                payment_method,
            });
            Ok(())
        }
    }
};
