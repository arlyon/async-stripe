#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionPaymentMethodOptionsMandateOptionsPix {
    /// Amount to be charged for future payments.
    pub amount: Option<i64>,
    /// Determines if the amount includes the IOF tax.
    pub amount_includes_iof: Option<stripe_shared::MandatePixAmountIncludesIof>,
    /// Date when the mandate expires and no further payments will be charged, in `YYYY-MM-DD`.
    pub end_date: Option<String>,
    /// Schedule at which the future payments will be charged.
    pub payment_schedule: Option<stripe_shared::MandatePixPaymentSchedule>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionPaymentMethodOptionsMandateOptionsPix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionPaymentMethodOptionsMandateOptionsPix").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionPaymentMethodOptionsMandateOptionsPixBuilder {
    amount: Option<Option<i64>>,
    amount_includes_iof: Option<Option<stripe_shared::MandatePixAmountIncludesIof>>,
    end_date: Option<Option<String>>,
    payment_schedule: Option<Option<stripe_shared::MandatePixPaymentSchedule>>,
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

    impl Deserialize for SubscriptionPaymentMethodOptionsMandateOptionsPix {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionPaymentMethodOptionsMandateOptionsPix>,
        builder: SubscriptionPaymentMethodOptionsMandateOptionsPixBuilder,
    }

    impl Visitor for Place<SubscriptionPaymentMethodOptionsMandateOptionsPix> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionPaymentMethodOptionsMandateOptionsPixBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionPaymentMethodOptionsMandateOptionsPixBuilder {
        type Out = SubscriptionPaymentMethodOptionsMandateOptionsPix;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "amount_includes_iof" => Deserialize::begin(&mut self.amount_includes_iof),
                "end_date" => Deserialize::begin(&mut self.end_date),
                "payment_schedule" => Deserialize::begin(&mut self.payment_schedule),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Some(None),
                amount_includes_iof: Some(None),
                end_date: Some(None),
                payment_schedule: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(amount), Some(amount_includes_iof), Some(end_date), Some(payment_schedule)) = (
                self.amount,
                self.amount_includes_iof.take(),
                self.end_date.take(),
                self.payment_schedule.take(),
            ) else {
                return None;
            };
            Some(Self::Out { amount, amount_includes_iof, end_date, payment_schedule })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for SubscriptionPaymentMethodOptionsMandateOptionsPix {
        type Builder = SubscriptionPaymentMethodOptionsMandateOptionsPixBuilder;
    }

    impl FromValueOpt for SubscriptionPaymentMethodOptionsMandateOptionsPix {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionPaymentMethodOptionsMandateOptionsPixBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "amount_includes_iof" => b.amount_includes_iof = FromValueOpt::from_value(v),
                    "end_date" => b.end_date = FromValueOpt::from_value(v),
                    "payment_schedule" => b.payment_schedule = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Determines if the amount includes the IOF tax.
pub type SubscriptionPaymentMethodOptionsMandateOptionsPixAmountIncludesIof =
    stripe_shared::MandatePixAmountIncludesIof;
/// Schedule at which the future payments will be charged.
pub type SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule =
    stripe_shared::MandatePixPaymentSchedule;
