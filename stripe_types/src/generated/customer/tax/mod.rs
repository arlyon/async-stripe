#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Tax {
    /// Surfaces if automatic tax computation is possible given the current customer location information.
    pub automatic_tax: TaxAutomaticTax,
    /// A recent IP address of the customer used for tax reporting and tax location inference.
    pub ip_address: Option<String>,
    /// The customer's location as identified by Stripe Tax.
    pub location: Option<stripe_types::customer::tax::location::Location>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Tax {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Surfaces if automatic tax computation is possible given the current customer location information.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TaxAutomaticTax {
    Failed,
    NotCollecting,
    Supported,
    UnrecognizedLocation,
}

impl TaxAutomaticTax {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Failed => "failed",
            Self::NotCollecting => "not_collecting",
            Self::Supported => "supported",
            Self::UnrecognizedLocation => "unrecognized_location",
        }
    }
}

impl std::str::FromStr for TaxAutomaticTax {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "failed" => Ok(Self::Failed),
            "not_collecting" => Ok(Self::NotCollecting),
            "supported" => Ok(Self::Supported),
            "unrecognized_location" => Ok(Self::UnrecognizedLocation),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for TaxAutomaticTax {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxAutomaticTax {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for TaxAutomaticTax {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxAutomaticTax {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TaxAutomaticTax"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TaxAutomaticTax {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TaxAutomaticTax> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxAutomaticTax::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
pub mod location;
pub use location::Location;
