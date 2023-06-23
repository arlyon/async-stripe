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
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
