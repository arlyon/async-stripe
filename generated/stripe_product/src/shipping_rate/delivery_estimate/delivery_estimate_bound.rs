#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DeliveryEstimateBound {
    /// A unit of time.
    pub unit: DeliveryEstimateBoundUnit,
    /// Must be greater than 0.
    pub value: i64,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DeliveryEstimateBound {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
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
        match self {
            Self::BusinessDay => "business_day",
            Self::Day => "day",
            Self::Hour => "hour",
            Self::Month => "month",
            Self::Week => "week",
        }
    }
}

impl std::str::FromStr for DeliveryEstimateBoundUnit {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "business_day" => Ok(Self::BusinessDay),
            "day" => Ok(Self::Day),
            "hour" => Ok(Self::Hour),
            "month" => Ok(Self::Month),
            "week" => Ok(Self::Week),

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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for DeliveryEstimateBoundUnit"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DeliveryEstimateBoundUnit {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<DeliveryEstimateBoundUnit> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(DeliveryEstimateBoundUnit::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
