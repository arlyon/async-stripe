#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct MandatePix {
    /// Determines if the amount includes the IOF tax.
    pub amount_includes_iof: Option<stripe_shared::MandatePixAmountIncludesIof>,
    /// Type of amount.
    pub amount_type: Option<stripe_shared::AmountType>,
    /// Date when the mandate expires and no further payments will be charged, in `YYYY-MM-DD`.
    pub end_date: Option<String>,
    /// Schedule at which the future payments will be charged.
    pub payment_schedule: Option<stripe_shared::MandatePixPaymentSchedule>,
    /// Subscription name displayed to buyers in their bank app.
    pub reference: Option<String>,
    /// Start date of the mandate, in `YYYY-MM-DD`.
    pub start_date: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for MandatePix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MandatePix").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct MandatePixBuilder {
    amount_includes_iof: Option<Option<stripe_shared::MandatePixAmountIncludesIof>>,
    amount_type: Option<Option<stripe_shared::AmountType>>,
    end_date: Option<Option<String>>,
    payment_schedule: Option<Option<stripe_shared::MandatePixPaymentSchedule>>,
    reference: Option<Option<String>>,
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

    impl Deserialize for MandatePix {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<MandatePix>,
        builder: MandatePixBuilder,
    }

    impl Visitor for Place<MandatePix> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: MandatePixBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for MandatePixBuilder {
        type Out = MandatePix;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_includes_iof" => Deserialize::begin(&mut self.amount_includes_iof),
                "amount_type" => Deserialize::begin(&mut self.amount_type),
                "end_date" => Deserialize::begin(&mut self.end_date),
                "payment_schedule" => Deserialize::begin(&mut self.payment_schedule),
                "reference" => Deserialize::begin(&mut self.reference),
                "start_date" => Deserialize::begin(&mut self.start_date),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount_includes_iof: Some(None),
                amount_type: Some(None),
                end_date: Some(None),
                payment_schedule: Some(None),
                reference: Some(None),
                start_date: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(amount_includes_iof),
                Some(amount_type),
                Some(end_date),
                Some(payment_schedule),
                Some(reference),
                Some(start_date),
            ) = (
                self.amount_includes_iof.take(),
                self.amount_type.take(),
                self.end_date.take(),
                self.payment_schedule.take(),
                self.reference.take(),
                self.start_date.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                amount_includes_iof,
                amount_type,
                end_date,
                payment_schedule,
                reference,
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

    impl ObjectDeser for MandatePix {
        type Builder = MandatePixBuilder;
    }

    impl FromValueOpt for MandatePix {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = MandatePixBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount_includes_iof" => b.amount_includes_iof = FromValueOpt::from_value(v),
                    "amount_type" => b.amount_type = FromValueOpt::from_value(v),
                    "end_date" => b.end_date = FromValueOpt::from_value(v),
                    "payment_schedule" => b.payment_schedule = FromValueOpt::from_value(v),
                    "reference" => b.reference = FromValueOpt::from_value(v),
                    "start_date" => b.start_date = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Determines if the amount includes the IOF tax.
pub type MandatePixAmountIncludesIof = stripe_shared::MandatePixAmountIncludesIof;
/// Type of amount.
pub type MandatePixAmountType = stripe_shared::AmountType;
/// Schedule at which the future payments will be charged.
pub type MandatePixPaymentSchedule = stripe_shared::MandatePixPaymentSchedule;
