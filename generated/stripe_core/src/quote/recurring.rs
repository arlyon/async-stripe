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
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
