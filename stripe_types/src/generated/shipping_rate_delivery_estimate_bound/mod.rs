#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ShippingRateDeliveryEstimateBound {
    /// A unit of time.
    pub unit: ShippingRateDeliveryEstimateBoundUnit,
    /// Must be greater than 0.
    pub value: i64,
}
/// A unit of time.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ShippingRateDeliveryEstimateBoundUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}

impl ShippingRateDeliveryEstimateBoundUnit {
    pub fn as_str(self) -> &'static str {
        use ShippingRateDeliveryEstimateBoundUnit::*;
        match self {
            BusinessDay => "business_day",
            Day => "day",
            Hour => "hour",
            Month => "month",
            Week => "week",
        }
    }
}

impl std::str::FromStr for ShippingRateDeliveryEstimateBoundUnit {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ShippingRateDeliveryEstimateBoundUnit::*;
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

impl AsRef<str> for ShippingRateDeliveryEstimateBoundUnit {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ShippingRateDeliveryEstimateBoundUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ShippingRateDeliveryEstimateBoundUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ShippingRateDeliveryEstimateBoundUnit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ShippingRateDeliveryEstimateBoundUnit {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for ShippingRateDeliveryEstimateBoundUnit"))
    }
}
