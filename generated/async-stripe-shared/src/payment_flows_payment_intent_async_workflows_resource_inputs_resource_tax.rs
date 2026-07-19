#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsPaymentIntentAsyncWorkflowsResourceInputsResourceTax {
    /// The [TaxCalculation](https://docs.stripe.com/api/tax/calculations) id
    pub calculation: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentFlowsPaymentIntentAsyncWorkflowsResourceInputsResourceTax {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentFlowsPaymentIntentAsyncWorkflowsResourceInputsResourceTax")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentFlowsPaymentIntentAsyncWorkflowsResourceInputsResourceTaxBuilder {
    calculation: Option<String>,
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

    impl Deserialize for PaymentFlowsPaymentIntentAsyncWorkflowsResourceInputsResourceTax {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentFlowsPaymentIntentAsyncWorkflowsResourceInputsResourceTax>,
        builder: PaymentFlowsPaymentIntentAsyncWorkflowsResourceInputsResourceTaxBuilder,
    }

    impl Visitor for Place<PaymentFlowsPaymentIntentAsyncWorkflowsResourceInputsResourceTax> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentFlowsPaymentIntentAsyncWorkflowsResourceInputsResourceTaxBuilder {
                    calculation: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "calculation" => Deserialize::begin(&mut self.builder.calculation),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(calculation),) = (self.builder.calculation.take(),) else {
                return Ok(());
            };
            *self.out = Some(PaymentFlowsPaymentIntentAsyncWorkflowsResourceInputsResourceTax {
                calculation,
            });
            Ok(())
        }
    }
};
