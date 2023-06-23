#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct BlikMandateOptionsOffSessionDetails {
    /// Amount of each recurring payment.
    pub amount: Option<i64>,
    /// Currency of each recurring payment.
    pub currency: Option<stripe_types::Currency>,
    /// Frequency interval of each recurring payment.
    pub interval: Option<BlikMandateOptionsOffSessionDetailsInterval>,
    /// Frequency indicator of each recurring payment.
    pub interval_count: Option<u64>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for BlikMandateOptionsOffSessionDetails {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Frequency interval of each recurring payment.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum BlikMandateOptionsOffSessionDetailsInterval {
    Day,
    Month,
    Week,
    Year,
}

impl BlikMandateOptionsOffSessionDetailsInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Month => "month",
            Self::Week => "week",
            Self::Year => "year",
        }
    }
}

impl AsRef<str> for BlikMandateOptionsOffSessionDetailsInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BlikMandateOptionsOffSessionDetailsInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
