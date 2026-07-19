#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsPaymentIntentAsyncWorkflows {
    pub inputs: Option<stripe_shared::PaymentFlowsPaymentIntentAsyncWorkflowsResourceInputs>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentFlowsPaymentIntentAsyncWorkflows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentFlowsPaymentIntentAsyncWorkflows").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentFlowsPaymentIntentAsyncWorkflowsBuilder {
    inputs: Option<Option<stripe_shared::PaymentFlowsPaymentIntentAsyncWorkflowsResourceInputs>>,
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

    impl Deserialize for PaymentFlowsPaymentIntentAsyncWorkflows {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentFlowsPaymentIntentAsyncWorkflows>,
        builder: PaymentFlowsPaymentIntentAsyncWorkflowsBuilder,
    }

    impl Visitor for Place<PaymentFlowsPaymentIntentAsyncWorkflows> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentFlowsPaymentIntentAsyncWorkflowsBuilder {
                    inputs: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "inputs" => Deserialize::begin(&mut self.builder.inputs),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(inputs),) = (self.builder.inputs.take(),) else {
                return Ok(());
            };
            *self.out = Some(PaymentFlowsPaymentIntentAsyncWorkflows { inputs });
            Ok(())
        }
    }
};
