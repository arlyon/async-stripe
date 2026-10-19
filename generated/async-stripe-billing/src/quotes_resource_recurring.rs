#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct QuotesResourceRecurring {
    /// Total before any discounts or taxes are applied.
    pub amount_subtotal: i64,
    /// Total after discounts and taxes are applied.
    pub amount_total: i64,
    /// The frequency at which a subscription is billed. One of `day`, `week`, `month` or `year`.
    pub interval: stripe_shared::PlanInterval,
    /// The number of intervals (specified in the `interval` attribute) between subscription billings.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    pub interval_count: u64,
    pub total_details: stripe_billing::QuotesResourceTotalDetails,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for QuotesResourceRecurring {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("QuotesResourceRecurring").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct QuotesResourceRecurringBuilder {
    amount_subtotal: Option<i64>,
    amount_total: Option<i64>,
    interval: Option<stripe_shared::PlanInterval>,
    interval_count: Option<u64>,
    total_details: Option<stripe_billing::QuotesResourceTotalDetails>,
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

    impl Deserialize for QuotesResourceRecurring {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<QuotesResourceRecurring>,
        builder: QuotesResourceRecurringBuilder,
    }

    impl Visitor for Place<QuotesResourceRecurring> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: QuotesResourceRecurringBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for QuotesResourceRecurringBuilder {
        type Out = QuotesResourceRecurring;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_subtotal" => Deserialize::begin(&mut self.amount_subtotal),
                "amount_total" => Deserialize::begin(&mut self.amount_total),
                "interval" => Deserialize::begin(&mut self.interval),
                "interval_count" => Deserialize::begin(&mut self.interval_count),
                "total_details" => Deserialize::begin(&mut self.total_details),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount_subtotal: None,
                amount_total: None,
                interval: None,
                interval_count: None,
                total_details: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(amount_subtotal),
                Some(amount_total),
                Some(interval),
                Some(interval_count),
                Some(total_details),
            ) = (
                self.amount_subtotal,
                self.amount_total,
                self.interval.take(),
                self.interval_count,
                self.total_details.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                amount_subtotal,
                amount_total,
                interval,
                interval_count,
                total_details,
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

    impl ObjectDeser for QuotesResourceRecurring {
        type Builder = QuotesResourceRecurringBuilder;
    }

    impl FromValueOpt for QuotesResourceRecurring {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = QuotesResourceRecurringBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount_subtotal" => b.amount_subtotal = FromValueOpt::from_value(v),
                    "amount_total" => b.amount_total = FromValueOpt::from_value(v),
                    "interval" => b.interval = FromValueOpt::from_value(v),
                    "interval_count" => b.interval_count = FromValueOpt::from_value(v),
                    "total_details" => b.total_details = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The frequency at which a subscription is billed. One of `day`, `week`, `month` or `year`.
pub type QuotesResourceRecurringInterval = stripe_shared::PlanInterval;
