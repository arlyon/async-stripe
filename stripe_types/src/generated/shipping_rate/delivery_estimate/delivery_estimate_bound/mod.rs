#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeliveryEstimateBound {
    /// A unit of time.
    pub unit: DeliveryEstimateBoundUnit,
    /// Must be greater than 0.
    pub value: i64,
}
/// A unit of time.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DeliveryEstimateBoundUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}

impl DeliveryEstimateBoundUnit {
    pub fn as_str(self) -> &'static str {
        use DeliveryEstimateBoundUnit::*;
        match self {
            BusinessDay => "business_day",
            Day => "day",
            Hour => "hour",
            Month => "month",
            Week => "week",
        }
    }
}

impl std::str::FromStr for DeliveryEstimateBoundUnit {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DeliveryEstimateBoundUnit::*;
        match s {
            "business_day" => Ok(BusinessDay),
            "day" => Ok(Day),
            "hour" => Ok(Hour),
            "month" => Ok(Month),
            "week" => Ok(Week),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for DeliveryEstimateBoundUnit {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DeliveryEstimateBoundUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for DeliveryEstimateBoundUnit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DeliveryEstimateBoundUnit {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for DeliveryEstimateBoundUnit"))
    }
}
