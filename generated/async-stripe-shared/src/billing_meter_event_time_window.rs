#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum BillingMeterEventTimeWindow {
    Day,
    Hour,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl BillingMeterEventTimeWindow {
    pub fn as_str(&self) -> &str {
        use BillingMeterEventTimeWindow::*;
        match self {
            Day => "day",
            Hour => "hour",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for BillingMeterEventTimeWindow {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingMeterEventTimeWindow::*;
        match s {
            "day" => Ok(Day),
            "hour" => Ok(Hour),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "BillingMeterEventTimeWindow"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for BillingMeterEventTimeWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for BillingMeterEventTimeWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingMeterEventTimeWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(BillingMeterEventTimeWindow)).finish_non_exhaustive()
    }
}
impl serde::Serialize for BillingMeterEventTimeWindow {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for BillingMeterEventTimeWindow {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<BillingMeterEventTimeWindow> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BillingMeterEventTimeWindow::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(BillingMeterEventTimeWindow);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingMeterEventTimeWindow {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
