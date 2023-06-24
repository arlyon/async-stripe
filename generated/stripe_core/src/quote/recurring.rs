#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Recurring {
    /// Total before any discounts or taxes are applied.
    pub amount_subtotal: i64,
    /// Total after discounts and taxes are applied.
    pub amount_total: i64,
    /// The frequency at which a subscription is billed.
    ///
    /// One of `day`, `week`, `month` or `year`.
    pub interval: RecurringInterval,
    /// The number of intervals (specified in the `interval` attribute) between subscription billings.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    pub interval_count: u64,
    pub total_details: stripe_core::quote::total_details::TotalDetails,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Recurring {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
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
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<RecurringInterval> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(RecurringInterval::from_str(s)?);
        Ok(())
    }
}
