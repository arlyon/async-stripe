/// The type of condition.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PortalResourceScheduleUpdateAtPeriodEndConditionType {
    DecreasingItemAmount,
    ShorteningInterval,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PortalResourceScheduleUpdateAtPeriodEndConditionType {
    pub fn as_str(&self) -> &str {
        use PortalResourceScheduleUpdateAtPeriodEndConditionType::*;
        match self {
            DecreasingItemAmount => "decreasing_item_amount",
            ShorteningInterval => "shortening_interval",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PortalResourceScheduleUpdateAtPeriodEndConditionType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PortalResourceScheduleUpdateAtPeriodEndConditionType::*;
        match s {
            "decreasing_item_amount" => Ok(DecreasingItemAmount),
            "shortening_interval" => Ok(ShorteningInterval),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PortalResourceScheduleUpdateAtPeriodEndConditionType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PortalResourceScheduleUpdateAtPeriodEndConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PortalResourceScheduleUpdateAtPeriodEndConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalResourceScheduleUpdateAtPeriodEndConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PortalResourceScheduleUpdateAtPeriodEndConditionType))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for PortalResourceScheduleUpdateAtPeriodEndConditionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PortalResourceScheduleUpdateAtPeriodEndConditionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PortalResourceScheduleUpdateAtPeriodEndConditionType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PortalResourceScheduleUpdateAtPeriodEndConditionType::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PortalResourceScheduleUpdateAtPeriodEndConditionType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PortalResourceScheduleUpdateAtPeriodEndConditionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
