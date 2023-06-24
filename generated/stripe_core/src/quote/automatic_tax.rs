#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AutomaticTax {
    /// Automatically calculate taxes.
    pub enabled: bool,
    /// The status of the most recent automated tax calculation for this quote.
    pub status: Option<AutomaticTaxStatus>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AutomaticTax {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The status of the most recent automated tax calculation for this quote.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AutomaticTaxStatus {
    Complete,
    Failed,
    RequiresLocationInputs,
}

impl AutomaticTaxStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Failed => "failed",
            Self::RequiresLocationInputs => "requires_location_inputs",
        }
    }
}

impl std::str::FromStr for AutomaticTaxStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "complete" => Ok(Self::Complete),
            "failed" => Ok(Self::Failed),
            "requires_location_inputs" => Ok(Self::RequiresLocationInputs),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for AutomaticTaxStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AutomaticTaxStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for AutomaticTaxStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AutomaticTaxStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for AutomaticTaxStatus"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AutomaticTaxStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<AutomaticTaxStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AutomaticTaxStatus::from_str(s)?);
        Ok(())
    }
}
