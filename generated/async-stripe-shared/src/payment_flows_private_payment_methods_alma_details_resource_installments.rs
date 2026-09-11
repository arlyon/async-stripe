#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsPrivatePaymentMethodsAlmaDetailsResourceInstallments {
    /// The number of installments.
    pub count: u64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentFlowsPrivatePaymentMethodsAlmaDetailsResourceInstallments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentFlowsPrivatePaymentMethodsAlmaDetailsResourceInstallments")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentFlowsPrivatePaymentMethodsAlmaDetailsResourceInstallmentsBuilder {
    count: Option<u64>,
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

    impl Deserialize for PaymentFlowsPrivatePaymentMethodsAlmaDetailsResourceInstallments {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentFlowsPrivatePaymentMethodsAlmaDetailsResourceInstallments>,
        builder: PaymentFlowsPrivatePaymentMethodsAlmaDetailsResourceInstallmentsBuilder,
    }

    impl Visitor for Place<PaymentFlowsPrivatePaymentMethodsAlmaDetailsResourceInstallments> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentFlowsPrivatePaymentMethodsAlmaDetailsResourceInstallmentsBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for PaymentFlowsPrivatePaymentMethodsAlmaDetailsResourceInstallmentsBuilder {
        type Out = PaymentFlowsPrivatePaymentMethodsAlmaDetailsResourceInstallments;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "count" => Deserialize::begin(&mut self.count),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { count: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(count),) = (self.count,) else {
                return None;
            };
            Some(Self::Out { count })
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

    impl ObjectDeser for PaymentFlowsPrivatePaymentMethodsAlmaDetailsResourceInstallments {
        type Builder = PaymentFlowsPrivatePaymentMethodsAlmaDetailsResourceInstallmentsBuilder;
    }

    impl FromValueOpt for PaymentFlowsPrivatePaymentMethodsAlmaDetailsResourceInstallments {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentFlowsPrivatePaymentMethodsAlmaDetailsResourceInstallmentsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "count" => b.count = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
