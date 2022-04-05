// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::resources::Currency;

/// The resource representing a Stripe "mandate_options_off_session_details_blik".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MandateOptionsOffSessionDetailsBlik {
    /// Amount of each recurring payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// Currency of each recurring payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// Frequency interval of each recurring payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<MandateOptionsOffSessionDetailsBlikInterval>,

    /// Frequency indicator of each recurring payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}

/// An enum representing the possible values of an `MandateOptionsOffSessionDetailsBlik`'s `interval` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MandateOptionsOffSessionDetailsBlikInterval {
    Day,
    Month,
    Week,
    Year,
}

impl MandateOptionsOffSessionDetailsBlikInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            MandateOptionsOffSessionDetailsBlikInterval::Day => "day",
            MandateOptionsOffSessionDetailsBlikInterval::Month => "month",
            MandateOptionsOffSessionDetailsBlikInterval::Week => "week",
            MandateOptionsOffSessionDetailsBlikInterval::Year => "year",
        }
    }
}

impl AsRef<str> for MandateOptionsOffSessionDetailsBlikInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for MandateOptionsOffSessionDetailsBlikInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for MandateOptionsOffSessionDetailsBlikInterval {
    fn default() -> Self {
        Self::Day
    }
}
