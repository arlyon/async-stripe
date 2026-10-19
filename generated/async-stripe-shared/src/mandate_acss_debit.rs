#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct MandateAcssDebit {
    /// List of Stripe products where this mandate can be selected automatically.
    pub default_for: Option<Vec<stripe_shared::DefaultFor>>,
    /// Description of the interval.
    /// Only required if the 'payment_schedule' parameter is 'interval' or 'combined'.
    pub interval_description: Option<String>,
    /// Payment schedule for the mandate.
    pub payment_schedule: stripe_shared::PaymentSchedule,
    /// Transaction type of the mandate.
    pub transaction_type: stripe_shared::TransactionType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for MandateAcssDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MandateAcssDebit").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct MandateAcssDebitBuilder {
    default_for: Option<Option<Vec<stripe_shared::DefaultFor>>>,
    interval_description: Option<Option<String>>,
    payment_schedule: Option<stripe_shared::PaymentSchedule>,
    transaction_type: Option<stripe_shared::TransactionType>,
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

    impl Deserialize for MandateAcssDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<MandateAcssDebit>,
        builder: MandateAcssDebitBuilder,
    }

    impl Visitor for Place<MandateAcssDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: MandateAcssDebitBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for MandateAcssDebitBuilder {
        type Out = MandateAcssDebit;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "default_for" => Deserialize::begin(&mut self.default_for),
                "interval_description" => Deserialize::begin(&mut self.interval_description),
                "payment_schedule" => Deserialize::begin(&mut self.payment_schedule),
                "transaction_type" => Deserialize::begin(&mut self.transaction_type),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                default_for: Some(None),
                interval_description: Some(None),
                payment_schedule: None,
                transaction_type: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(default_for),
                Some(interval_description),
                Some(payment_schedule),
                Some(transaction_type),
            ) = (
                self.default_for.take(),
                self.interval_description.take(),
                self.payment_schedule.take(),
                self.transaction_type.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                default_for,
                interval_description,
                payment_schedule,
                transaction_type,
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

    impl ObjectDeser for MandateAcssDebit {
        type Builder = MandateAcssDebitBuilder;
    }

    impl FromValueOpt for MandateAcssDebit {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = MandateAcssDebitBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "default_for" => b.default_for = FromValueOpt::from_value(v),
                    "interval_description" => b.interval_description = FromValueOpt::from_value(v),
                    "payment_schedule" => b.payment_schedule = FromValueOpt::from_value(v),
                    "transaction_type" => b.transaction_type = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// List of Stripe products where this mandate can be selected automatically.
pub type MandateAcssDebitDefaultFor = stripe_shared::DefaultFor;
/// Payment schedule for the mandate.
pub type MandateAcssDebitPaymentSchedule = stripe_shared::PaymentSchedule;
/// Transaction type of the mandate.
pub type MandateAcssDebitTransactionType = stripe_shared::TransactionType;
