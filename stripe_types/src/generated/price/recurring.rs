#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Recurring {
    /// Specifies a usage aggregation strategy for prices of `usage_type=metered`.
    ///
    /// Allowed values are `sum` for summing up all usage during a period, `last_during_period` for using the last usage record reported within a period, `last_ever` for using the last usage record ever (across period bounds) or `max` which uses the usage record with the maximum reported usage during a period.
    /// Defaults to `sum`.
    pub aggregate_usage: Option<RecurringAggregateUsage>,
    /// The frequency at which a subscription is billed.
    ///
    /// One of `day`, `week`, `month` or `year`.
    pub interval: RecurringInterval,
    /// The number of intervals (specified in the `interval` attribute) between subscription billings.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    pub interval_count: u64,
    /// Default number of trial days when subscribing a customer to this price using [`trial_from_plan=true`](https://stripe.com/docs/api#create_subscription-trial_from_plan).
    pub trial_period_days: Option<u32>,
    /// Configures how the quantity per period should be determined.
    ///
    /// Can be either `metered` or `licensed`.
    /// `licensed` automatically bills the `quantity` set when adding it to a subscription.
    /// `metered` aggregates the total usage based on usage records.
    /// Defaults to `licensed`.
    pub usage_type: RecurringUsageType,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Recurring {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Specifies a usage aggregation strategy for prices of `usage_type=metered`.
///
/// Allowed values are `sum` for summing up all usage during a period, `last_during_period` for using the last usage record reported within a period, `last_ever` for using the last usage record ever (across period bounds) or `max` which uses the usage record with the maximum reported usage during a period.
/// Defaults to `sum`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RecurringAggregateUsage {
    LastDuringPeriod,
    LastEver,
    Max,
    Sum,
}

impl RecurringAggregateUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LastDuringPeriod => "last_during_period",
            Self::LastEver => "last_ever",
            Self::Max => "max",
            Self::Sum => "sum",
        }
    }
}

impl std::str::FromStr for RecurringAggregateUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "last_during_period" => Ok(Self::LastDuringPeriod),
            "last_ever" => Ok(Self::LastEver),
            "max" => Ok(Self::Max),
            "sum" => Ok(Self::Sum),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for RecurringAggregateUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for RecurringAggregateUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for RecurringAggregateUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RecurringAggregateUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for RecurringAggregateUsage"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for RecurringAggregateUsage {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<RecurringAggregateUsage> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(RecurringAggregateUsage::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// The frequency at which a subscription is billed.
///
/// One of `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RecurringInterval {
    Day,
    Month,
    Week,
    Year,
}

impl RecurringInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Month => "month",
            Self::Week => "week",
            Self::Year => "year",
        }
    }
}

impl std::str::FromStr for RecurringInterval {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "day" => Ok(Self::Day),
            "month" => Ok(Self::Month),
            "week" => Ok(Self::Week),
            "year" => Ok(Self::Year),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for RecurringInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for RecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for RecurringInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RecurringInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for RecurringInterval"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for RecurringInterval {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<RecurringInterval> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(RecurringInterval::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Configures how the quantity per period should be determined.
///
/// Can be either `metered` or `licensed`.
/// `licensed` automatically bills the `quantity` set when adding it to a subscription.
/// `metered` aggregates the total usage based on usage records.
/// Defaults to `licensed`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RecurringUsageType {
    Licensed,
    Metered,
}

impl RecurringUsageType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Licensed => "licensed",
            Self::Metered => "metered",
        }
    }
}

impl std::str::FromStr for RecurringUsageType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "licensed" => Ok(Self::Licensed),
            "metered" => Ok(Self::Metered),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for RecurringUsageType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for RecurringUsageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for RecurringUsageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RecurringUsageType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for RecurringUsageType"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for RecurringUsageType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<RecurringUsageType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(RecurringUsageType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
