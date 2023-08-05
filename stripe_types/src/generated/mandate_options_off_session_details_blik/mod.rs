#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct MandateOptionsOffSessionDetailsBlik {
    /// Amount of each recurring payment.
    pub amount: Option<i64>,
    /// Currency of each recurring payment.
    pub currency: Option<stripe_types::Currency>,
    /// Frequency interval of each recurring payment.
    pub interval: Option<MandateOptionsOffSessionDetailsBlikInterval>,
    /// Frequency indicator of each recurring payment.
    pub interval_count: Option<u64>,
}
/// Frequency interval of each recurring payment.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum MandateOptionsOffSessionDetailsBlikInterval {
    Day,
    Month,
    Week,
    Year,
}

impl MandateOptionsOffSessionDetailsBlikInterval {
    pub fn as_str(self) -> &'static str {
        use MandateOptionsOffSessionDetailsBlikInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for MandateOptionsOffSessionDetailsBlikInterval {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandateOptionsOffSessionDetailsBlikInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for MandateOptionsOffSessionDetailsBlikInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for MandateOptionsOffSessionDetailsBlikInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MandateOptionsOffSessionDetailsBlikInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for MandateOptionsOffSessionDetailsBlikInterval",
            )
        })
    }
}
