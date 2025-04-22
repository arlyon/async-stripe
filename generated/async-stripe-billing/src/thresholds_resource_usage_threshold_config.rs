/// The usage threshold alert configuration enables setting up alerts for when a certain usage threshold on a specific meter is crossed.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ThresholdsResourceUsageThresholdConfig {
    /// The filters allow limiting the scope of this usage alert.
    /// You can only specify up to one filter at this time.
    pub filters: Option<Vec<stripe_billing::ThresholdsResourceUsageAlertFilter>>,
    /// The value at which this alert will trigger.
    pub gte: i64,
    /// The [Billing Meter](/api/billing/meter) ID whose usage is monitored.
    pub meter: stripe_types::Expandable<stripe_billing::BillingMeter>,
    /// Defines how the alert will behave.
    pub recurrence: ThresholdsResourceUsageThresholdConfigRecurrence,
}
#[doc(hidden)]
pub struct ThresholdsResourceUsageThresholdConfigBuilder {
    filters: Option<Option<Vec<stripe_billing::ThresholdsResourceUsageAlertFilter>>>,
    gte: Option<i64>,
    meter: Option<stripe_types::Expandable<stripe_billing::BillingMeter>>,
    recurrence: Option<ThresholdsResourceUsageThresholdConfigRecurrence>,
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

    impl Deserialize for ThresholdsResourceUsageThresholdConfig {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ThresholdsResourceUsageThresholdConfig>,
        builder: ThresholdsResourceUsageThresholdConfigBuilder,
    }

    impl Visitor for Place<ThresholdsResourceUsageThresholdConfig> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ThresholdsResourceUsageThresholdConfigBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ThresholdsResourceUsageThresholdConfigBuilder {
        type Out = ThresholdsResourceUsageThresholdConfig;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "filters" => Deserialize::begin(&mut self.filters),
                "gte" => Deserialize::begin(&mut self.gte),
                "meter" => Deserialize::begin(&mut self.meter),
                "recurrence" => Deserialize::begin(&mut self.recurrence),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                filters: Deserialize::default(),
                gte: Deserialize::default(),
                meter: Deserialize::default(),
                recurrence: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(filters), Some(gte), Some(meter), Some(recurrence)) =
                (self.filters.take(), self.gte, self.meter.take(), self.recurrence)
            else {
                return None;
            };
            Some(Self::Out { filters, gte, meter, recurrence })
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

    impl ObjectDeser for ThresholdsResourceUsageThresholdConfig {
        type Builder = ThresholdsResourceUsageThresholdConfigBuilder;
    }

    impl FromValueOpt for ThresholdsResourceUsageThresholdConfig {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ThresholdsResourceUsageThresholdConfigBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "filters" => b.filters = FromValueOpt::from_value(v),
                    "gte" => b.gte = FromValueOpt::from_value(v),
                    "meter" => b.meter = FromValueOpt::from_value(v),
                    "recurrence" => b.recurrence = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Defines how the alert will behave.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ThresholdsResourceUsageThresholdConfigRecurrence {
    OneTime,
}
impl ThresholdsResourceUsageThresholdConfigRecurrence {
    pub fn as_str(self) -> &'static str {
        use ThresholdsResourceUsageThresholdConfigRecurrence::*;
        match self {
            OneTime => "one_time",
        }
    }
}

impl std::str::FromStr for ThresholdsResourceUsageThresholdConfigRecurrence {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThresholdsResourceUsageThresholdConfigRecurrence::*;
        match s {
            "one_time" => Ok(OneTime),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ThresholdsResourceUsageThresholdConfigRecurrence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ThresholdsResourceUsageThresholdConfigRecurrence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ThresholdsResourceUsageThresholdConfigRecurrence {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ThresholdsResourceUsageThresholdConfigRecurrence {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ThresholdsResourceUsageThresholdConfigRecurrence> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            ThresholdsResourceUsageThresholdConfigRecurrence::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ThresholdsResourceUsageThresholdConfigRecurrence);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ThresholdsResourceUsageThresholdConfigRecurrence {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for ThresholdsResourceUsageThresholdConfigRecurrence",
            )
        })
    }
}
