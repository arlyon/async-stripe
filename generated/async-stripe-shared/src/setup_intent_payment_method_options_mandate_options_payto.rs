#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SetupIntentPaymentMethodOptionsMandateOptionsPayto {
    /// Amount that will be collected. It is required when `amount_type` is `fixed`.
    pub amount: Option<i64>,
    /// The type of amount that will be collected.
    /// The amount charged must be exact or up to the value of `amount` param for `fixed` or `maximum` type respectively.
    /// Defaults to `maximum`.
    pub amount_type: Option<stripe_shared::AmountType>,
    /// Date, in YYYY-MM-DD format, after which payments will not be collected. Defaults to no end date.
    pub end_date: Option<String>,
    /// The periodicity at which payments will be collected. Defaults to `adhoc`.
    pub payment_schedule: Option<stripe_shared::MandatePaytoPaymentSchedule>,
    /// The number of payments that will be made during a payment period.
    /// Defaults to 1 except for when `payment_schedule` is `adhoc`.
    /// In that case, it defaults to no limit.
    pub payments_per_period: Option<i64>,
    /// The purpose for which payments are made. Has a default value based on your merchant category code.
    pub purpose: Option<stripe_shared::Purpose>,
    /// Date, in YYYY-MM-DD format, from which payments will be collected. Defaults to confirmation time.
    pub start_date: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SetupIntentPaymentMethodOptionsMandateOptionsPayto {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SetupIntentPaymentMethodOptionsMandateOptionsPayto").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SetupIntentPaymentMethodOptionsMandateOptionsPaytoBuilder {
    amount: Option<Option<i64>>,
    amount_type: Option<Option<stripe_shared::AmountType>>,
    end_date: Option<Option<String>>,
    payment_schedule: Option<Option<stripe_shared::MandatePaytoPaymentSchedule>>,
    payments_per_period: Option<Option<i64>>,
    purpose: Option<Option<stripe_shared::Purpose>>,
    start_date: Option<Option<String>>,
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

    impl Deserialize for SetupIntentPaymentMethodOptionsMandateOptionsPayto {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupIntentPaymentMethodOptionsMandateOptionsPayto>,
        builder: SetupIntentPaymentMethodOptionsMandateOptionsPaytoBuilder,
    }

    impl Visitor for Place<SetupIntentPaymentMethodOptionsMandateOptionsPayto> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SetupIntentPaymentMethodOptionsMandateOptionsPaytoBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SetupIntentPaymentMethodOptionsMandateOptionsPaytoBuilder {
        type Out = SetupIntentPaymentMethodOptionsMandateOptionsPayto;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "amount_type" => Deserialize::begin(&mut self.amount_type),
                "end_date" => Deserialize::begin(&mut self.end_date),
                "payment_schedule" => Deserialize::begin(&mut self.payment_schedule),
                "payments_per_period" => Deserialize::begin(&mut self.payments_per_period),
                "purpose" => Deserialize::begin(&mut self.purpose),
                "start_date" => Deserialize::begin(&mut self.start_date),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Some(None),
                amount_type: Some(None),
                end_date: Some(None),
                payment_schedule: Some(None),
                payments_per_period: Some(None),
                purpose: Some(None),
                start_date: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(amount),
                Some(amount_type),
                Some(end_date),
                Some(payment_schedule),
                Some(payments_per_period),
                Some(purpose),
                Some(start_date),
            ) = (
                self.amount,
                self.amount_type.take(),
                self.end_date.take(),
                self.payment_schedule.take(),
                self.payments_per_period,
                self.purpose.take(),
                self.start_date.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                amount,
                amount_type,
                end_date,
                payment_schedule,
                payments_per_period,
                purpose,
                start_date,
            })
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

    impl ObjectDeser for SetupIntentPaymentMethodOptionsMandateOptionsPayto {
        type Builder = SetupIntentPaymentMethodOptionsMandateOptionsPaytoBuilder;
    }

    impl FromValueOpt for SetupIntentPaymentMethodOptionsMandateOptionsPayto {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SetupIntentPaymentMethodOptionsMandateOptionsPaytoBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "amount_type" => b.amount_type = FromValueOpt::from_value(v),
                    "end_date" => b.end_date = FromValueOpt::from_value(v),
                    "payment_schedule" => b.payment_schedule = FromValueOpt::from_value(v),
                    "payments_per_period" => b.payments_per_period = FromValueOpt::from_value(v),
                    "purpose" => b.purpose = FromValueOpt::from_value(v),
                    "start_date" => b.start_date = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The type of amount that will be collected.
/// The amount charged must be exact or up to the value of `amount` param for `fixed` or `maximum` type respectively.
/// Defaults to `maximum`.
pub type SetupIntentPaymentMethodOptionsMandateOptionsPaytoAmountType = stripe_shared::AmountType;
/// The periodicity at which payments will be collected. Defaults to `adhoc`.
pub type SetupIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule =
    stripe_shared::MandatePaytoPaymentSchedule;
/// The purpose for which payments are made. Has a default value based on your merchant category code.
pub type SetupIntentPaymentMethodOptionsMandateOptionsPaytoPurpose = stripe_shared::Purpose;
