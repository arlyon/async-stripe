#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodOptionsCardInstallments {
    /// Installment plans that may be selected for this PaymentIntent.
    pub available_plans: Option<Vec<stripe_shared::PaymentMethodDetailsCardInstallmentsPlan>>,
    /// Whether Installments are enabled for this PaymentIntent.
    pub enabled: bool,
    /// Installment plan selected for this PaymentIntent.
    pub plan: Option<stripe_shared::PaymentMethodDetailsCardInstallmentsPlan>,
}
#[doc(hidden)]
pub struct PaymentMethodOptionsCardInstallmentsBuilder {
    available_plans: Option<Option<Vec<stripe_shared::PaymentMethodDetailsCardInstallmentsPlan>>>,
    enabled: Option<bool>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodOptionsCardInstallments {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodOptionsCardInstallments>,
        builder: PaymentMethodOptionsCardInstallmentsBuilder,
    }

    impl Visitor for Place<PaymentMethodOptionsCardInstallments> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodOptionsCardInstallmentsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodOptionsCardInstallmentsBuilder {
        type Out = PaymentMethodOptionsCardInstallments;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "available_plans" => Deserialize::begin(&mut self.available_plans),
                "enabled" => Deserialize::begin(&mut self.enabled),
                "plan" => Deserialize::begin(&mut self.plan),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                available_plans: Deserialize::default(),
                enabled: Deserialize::default(),
                plan: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(available_plans), Some(enabled), Some(plan)) =
                (self.available_plans.take(), self.enabled, self.plan)
            else {
                return None;
            };
            Some(Self::Out { available_plans, enabled, plan })
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

    impl ObjectDeser for PaymentMethodOptionsCardInstallments {
        type Builder = PaymentMethodOptionsCardInstallmentsBuilder;
    }

    impl FromValueOpt for PaymentMethodOptionsCardInstallments {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodOptionsCardInstallmentsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "available_plans" => b.available_plans = FromValueOpt::from_value(v),
                    "enabled" => b.enabled = FromValueOpt::from_value(v),
                    "plan" => b.plan = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
