#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionPendingInvoiceItemInterval {
    /// Specifies invoicing frequency. Either `day`, `week`, `month` or `year`.
    pub interval: SubscriptionPendingInvoiceItemIntervalInterval,
    /// The number of intervals between invoices.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    pub interval_count: u64,
}
#[doc(hidden)]
pub struct SubscriptionPendingInvoiceItemIntervalBuilder {
    interval: Option<SubscriptionPendingInvoiceItemIntervalInterval>,
    interval_count: Option<u64>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
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
            Self { interval: Deserialize::default(), interval_count: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { interval: self.interval?, interval_count: self.interval_count? })
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
                    "interval" => b.interval = Some(FromValueOpt::from_value(v)?),
                    "interval_count" => b.interval_count = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Specifies invoicing frequency. Either `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SubscriptionPendingInvoiceItemIntervalInterval {
    Day,
    Month,
    Week,
    Year,
}
impl SubscriptionPendingInvoiceItemIntervalInterval {
    pub fn as_str(self) -> &'static str {
        use SubscriptionPendingInvoiceItemIntervalInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for SubscriptionPendingInvoiceItemIntervalInterval {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionPendingInvoiceItemIntervalInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for SubscriptionPendingInvoiceItemIntervalInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SubscriptionPendingInvoiceItemIntervalInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionPendingInvoiceItemIntervalInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for SubscriptionPendingInvoiceItemIntervalInterval {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<SubscriptionPendingInvoiceItemIntervalInterval> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SubscriptionPendingInvoiceItemIntervalInterval::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(SubscriptionPendingInvoiceItemIntervalInterval);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SubscriptionPendingInvoiceItemIntervalInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for SubscriptionPendingInvoiceItemIntervalInterval",
            )
        })
    }
}
