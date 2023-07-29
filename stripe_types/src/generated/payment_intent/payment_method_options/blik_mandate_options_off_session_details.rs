#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
/// Frequency interval of each recurring payment.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for BlikMandateOptionsOffSessionDetailsInterval {
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
impl serde::Serialize for BlikMandateOptionsOffSessionDetailsInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BlikMandateOptionsOffSessionDetailsInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for BlikMandateOptionsOffSessionDetailsInterval",
            )
        })
    }
}
