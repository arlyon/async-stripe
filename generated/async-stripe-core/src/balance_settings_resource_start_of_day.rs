#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BalanceSettingsResourceStartOfDay {
    /// Hour at which the customized start of day begins according to the given timezone.
    /// Must be a [supported customized start of day hour](/connect/customized-start-of-day#available-timezones-and-cutoffs).
    pub hour: i64,
    /// Minutes at which the customized start of day begins according to the given timezone.
    /// Must be either 0 or 30.
    pub minutes: i64,
    /// Timezone for the customized start of day.
    /// Must be a [supported customized start of day timezone](/connect/customized-start-of-day#available-timezones-and-cutoffs).
    pub timezone: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BalanceSettingsResourceStartOfDay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BalanceSettingsResourceStartOfDay").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BalanceSettingsResourceStartOfDayBuilder {
    hour: Option<i64>,
    minutes: Option<i64>,
    timezone: Option<String>,
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

    impl Deserialize for BalanceSettingsResourceStartOfDay {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BalanceSettingsResourceStartOfDay>,
        builder: BalanceSettingsResourceStartOfDayBuilder,
    }

    impl Visitor for Place<BalanceSettingsResourceStartOfDay> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BalanceSettingsResourceStartOfDayBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BalanceSettingsResourceStartOfDayBuilder {
        type Out = BalanceSettingsResourceStartOfDay;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "hour" => Deserialize::begin(&mut self.hour),
                "minutes" => Deserialize::begin(&mut self.minutes),
                "timezone" => Deserialize::begin(&mut self.timezone),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { hour: None, minutes: None, timezone: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self { hour: Some(hour), minutes: Some(minutes), timezone: Some(timezone) } => {
                    Some(Self::Out { hour, minutes, timezone })
                }
                builder => {
                    *self = builder;
                    None
                }
            }
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

    impl ObjectDeser for BalanceSettingsResourceStartOfDay {
        type Builder = BalanceSettingsResourceStartOfDayBuilder;
    }

    impl FromValueOpt for BalanceSettingsResourceStartOfDay {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
