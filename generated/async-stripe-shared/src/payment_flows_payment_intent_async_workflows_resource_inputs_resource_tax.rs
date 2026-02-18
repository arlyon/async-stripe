#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsPaymentIntentAsyncWorkflowsResourceInputsResourceTax {
    /// The [TaxCalculation](https://docs.stripe.com/api/tax/calculations) id
    pub calculation: String,
}
#[doc(hidden)]
pub struct PaymentFlowsPaymentIntentAsyncWorkflowsResourceInputsResourceTaxBuilder {
    calculation: Option<String>,
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
            builder: PaymentFlowsPaymentIntentAsyncWorkflowsResourceInputsResourceTaxBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for PaymentFlowsPaymentIntentAsyncWorkflowsResourceInputsResourceTaxBuilder {
        type Out = PaymentFlowsPaymentIntentAsyncWorkflowsResourceInputsResourceTax;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "calculation" => Deserialize::begin(&mut self.calculation),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { calculation: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(calculation),) = (self.calculation.take(),) else {
                return None;
            };
            Some(Self::Out { calculation })
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

    impl ObjectDeser for PaymentFlowsPaymentIntentAsyncWorkflowsResourceInputsResourceTax {
        type Builder = PaymentFlowsPaymentIntentAsyncWorkflowsResourceInputsResourceTaxBuilder;
    }

    impl FromValueOpt for PaymentFlowsPaymentIntentAsyncWorkflowsResourceInputsResourceTax {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentFlowsPaymentIntentAsyncWorkflowsResourceInputsResourceTaxBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "calculation" => b.calculation = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
