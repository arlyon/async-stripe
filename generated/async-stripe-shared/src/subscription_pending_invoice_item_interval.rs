#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionPendingInvoiceItemInterval {
    /// Specifies invoicing frequency. Either `day`, `week`, `month` or `year`.
    pub interval: stripe_shared::PlanInterval,
    /// The number of intervals between invoices.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    pub interval_count: u64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionPendingInvoiceItemInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionPendingInvoiceItemInterval").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionPendingInvoiceItemIntervalBuilder {
    interval: Option<stripe_shared::PlanInterval>,
    interval_count: Option<u64>,
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

    impl Deserialize for SubscriptionPendingInvoiceItemInterval {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionPendingInvoiceItemInterval>,
        builder: SubscriptionPendingInvoiceItemIntervalBuilder,
    }

    impl Visitor for Place<SubscriptionPendingInvoiceItemInterval> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionPendingInvoiceItemIntervalBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionPendingInvoiceItemIntervalBuilder {
        type Out = SubscriptionPendingInvoiceItemInterval;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "interval" => Deserialize::begin(&mut self.interval),
                "interval_count" => Deserialize::begin(&mut self.interval_count),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { interval: None, interval_count: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(interval), Some(interval_count)) =
                (self.interval.take(), self.interval_count)
            else {
                return None;
            };
            Some(Self::Out { interval, interval_count })
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

    impl ObjectDeser for SubscriptionPendingInvoiceItemInterval {
        type Builder = SubscriptionPendingInvoiceItemIntervalBuilder;
    }

    impl FromValueOpt for SubscriptionPendingInvoiceItemInterval {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionPendingInvoiceItemIntervalBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "interval" => b.interval = FromValueOpt::from_value(v),
                    "interval_count" => b.interval_count = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Specifies invoicing frequency. Either `day`, `week`, `month` or `year`.
pub type SubscriptionPendingInvoiceItemIntervalInterval = stripe_shared::PlanInterval;
