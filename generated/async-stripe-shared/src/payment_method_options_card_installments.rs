#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodOptionsCardInstallments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodOptionsCardInstallments").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: PaymentMethodOptionsCardInstallmentsBuilder {
                    available_plans: Deserialize::default(),
                    enabled: Deserialize::default(),
                    plan: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "available_plans" => Deserialize::begin(&mut self.builder.available_plans),
                "enabled" => Deserialize::begin(&mut self.builder.enabled),
                "plan" => Deserialize::begin(&mut self.builder.plan),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(available_plans), Some(enabled), Some(plan)) = (
                self.builder.available_plans.take(),
                self.builder.enabled,
                self.builder.plan.take(),
            ) else {
                return Ok(());
            };
            *self.out =
                Some(PaymentMethodOptionsCardInstallments { available_plans, enabled, plan });
            Ok(())
        }
    }
};
