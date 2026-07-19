/// The usage threshold alert configuration enables setting up alerts for when a certain usage threshold on a specific meter is crossed.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ThresholdsResourceUsageThresholdConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ThresholdsResourceUsageThresholdConfig").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: ThresholdsResourceUsageThresholdConfigBuilder {
                    filters: Deserialize::default(),
                    gte: Deserialize::default(),
                    meter: Deserialize::default(),
                    recurrence: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "filters" => Deserialize::begin(&mut self.builder.filters),
                "gte" => Deserialize::begin(&mut self.builder.gte),
                "meter" => Deserialize::begin(&mut self.builder.meter),
                "recurrence" => Deserialize::begin(&mut self.builder.recurrence),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(filters), Some(gte), Some(meter), Some(recurrence)) = (
                self.builder.filters.take(),
                self.builder.gte,
                self.builder.meter.take(),
                self.builder.recurrence.take(),
            ) else {
                return Ok(());
            };
            *self.out =
                Some(ThresholdsResourceUsageThresholdConfig { filters, gte, meter, recurrence });
            Ok(())
        }
    }
};
/// Defines how the alert will behave.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ThresholdsResourceUsageThresholdConfigRecurrence {
    OneTime,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ThresholdsResourceUsageThresholdConfigRecurrence {
    pub fn as_str(&self) -> &str {
        use ThresholdsResourceUsageThresholdConfigRecurrence::*;
        match self {
            OneTime => "one_time",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ThresholdsResourceUsageThresholdConfigRecurrence {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ThresholdsResourceUsageThresholdConfigRecurrence::*;
        match s {
            "one_time" => Ok(OneTime),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "ThresholdsResourceUsageThresholdConfigRecurrence"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ThresholdsResourceUsageThresholdConfigRecurrence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ThresholdsResourceUsageThresholdConfigRecurrence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ThresholdsResourceUsageThresholdConfigRecurrence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ThresholdsResourceUsageThresholdConfigRecurrence))
            .finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for ThresholdsResourceUsageThresholdConfigRecurrence {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<ThresholdsResourceUsageThresholdConfigRecurrence> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            ThresholdsResourceUsageThresholdConfigRecurrence::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ThresholdsResourceUsageThresholdConfigRecurrence {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
