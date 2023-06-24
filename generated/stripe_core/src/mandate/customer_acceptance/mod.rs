#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CustomerAcceptance {
    /// The time at which the customer accepted the Mandate.
    pub accepted_at: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<stripe_core::mandate::customer_acceptance::offline::Offline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<stripe_core::mandate::customer_acceptance::online::Online>,
    /// The type of customer acceptance information included with the Mandate.
    ///
    /// One of `online` or `offline`.
    #[serde(rename = "type")]
    pub type_: CustomerAcceptanceType,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CustomerAcceptance {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The type of customer acceptance information included with the Mandate.
///
/// One of `online` or `offline`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CustomerAcceptanceType {
    Offline,
    Online,
}

impl CustomerAcceptanceType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Offline => "offline",
            Self::Online => "online",
        }
    }
}

impl std::str::FromStr for CustomerAcceptanceType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "offline" => Ok(Self::Offline),
            "online" => Ok(Self::Online),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CustomerAcceptanceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerAcceptanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CustomerAcceptanceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CustomerAcceptanceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CustomerAcceptanceType"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CustomerAcceptanceType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CustomerAcceptanceType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CustomerAcceptanceType::from_str(s)?);
        Ok(())
    }
}
pub mod offline;
pub use offline::Offline;
pub mod online;
pub use online::Online;
