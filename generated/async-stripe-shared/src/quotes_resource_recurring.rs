#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct QuotesResourceRecurring {
    /// Total before any discounts or taxes are applied.
    pub amount_subtotal: i64,
    /// Total after discounts and taxes are applied.
    pub amount_total: i64,
    /// The frequency at which a subscription is billed. One of `day`, `week`, `month` or `year`.
    pub interval: QuotesResourceRecurringInterval,
    /// The number of intervals (specified in the `interval` attribute) between subscription billings.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    pub interval_count: u64,
    pub total_details: stripe_shared::QuotesResourceTotalDetails,
}
#[doc(hidden)]
pub struct QuotesResourceRecurringBuilder {
    amount_subtotal: Option<i64>,
    amount_total: Option<i64>,
    interval: Option<QuotesResourceRecurringInterval>,
    interval_count: Option<u64>,
    total_details: Option<stripe_shared::QuotesResourceTotalDetails>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
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
                amount_subtotal: Deserialize::default(),
                amount_total: Deserialize::default(),
                interval: Deserialize::default(),
                interval_count: Deserialize::default(),
                total_details: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                amount_subtotal: self.amount_subtotal?,
                amount_total: self.amount_total?,
                interval: self.interval?,
                interval_count: self.interval_count?,
                total_details: self.total_details.take()?,
            })
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
                    "amount_subtotal" => b.amount_subtotal = Some(FromValueOpt::from_value(v)?),
                    "amount_total" => b.amount_total = Some(FromValueOpt::from_value(v)?),
                    "interval" => b.interval = Some(FromValueOpt::from_value(v)?),
                    "interval_count" => b.interval_count = Some(FromValueOpt::from_value(v)?),
                    "total_details" => b.total_details = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The frequency at which a subscription is billed. One of `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum QuotesResourceRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}
impl QuotesResourceRecurringInterval {
    pub fn as_str(self) -> &'static str {
        use QuotesResourceRecurringInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for QuotesResourceRecurringInterval {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use QuotesResourceRecurringInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for QuotesResourceRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for QuotesResourceRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for QuotesResourceRecurringInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for QuotesResourceRecurringInterval {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<QuotesResourceRecurringInterval> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(QuotesResourceRecurringInterval::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(QuotesResourceRecurringInterval);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for QuotesResourceRecurringInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for QuotesResourceRecurringInterval")
        })
    }
}
