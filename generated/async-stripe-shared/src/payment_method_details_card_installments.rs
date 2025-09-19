#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsCardInstallments {
    /// Installment plan selected for the payment.
    pub plan: Option<stripe_shared::PaymentMethodDetailsCardInstallmentsPlan>,
}
#[doc(hidden)]
pub struct PaymentMethodDetailsCardInstallmentsBuilder {
    plan: Option<Option<stripe_shared::PaymentMethodDetailsCardInstallmentsPlan>>,
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

    impl Deserialize for PaymentMethodDetailsCardInstallments {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsCardInstallments>,
        builder: PaymentMethodDetailsCardInstallmentsBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsCardInstallments> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsCardInstallmentsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsCardInstallmentsBuilder {
        type Out = PaymentMethodDetailsCardInstallments;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "plan" => Deserialize::begin(&mut self.plan),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { plan: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(plan),) = (self.plan,) else {
                return None;
            };
            Some(Self::Out { plan })
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

    impl ObjectDeser for PaymentMethodDetailsCardInstallments {
        type Builder = PaymentMethodDetailsCardInstallmentsBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsCardInstallments {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsCardInstallmentsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "plan" => b.plan = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
