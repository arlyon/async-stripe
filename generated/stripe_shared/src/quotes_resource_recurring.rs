#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use QuotesResourceRecurringInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(()),
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
impl serde::Serialize for QuotesResourceRecurringInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for QuotesResourceRecurringInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for QuotesResourceRecurringInterval")
        })
    }
}
