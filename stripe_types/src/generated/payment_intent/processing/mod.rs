#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Processing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types::payment_intent::processing::card::Card>,
    /// Type of the payment method for which payment is in `processing` state, one of `card`.
    #[serde(rename = "type")]
    pub type_: ProcessingType,
}
/// Type of the payment method for which payment is in `processing` state, one of `card`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ProcessingType {
    Card,
}

impl ProcessingType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Card => "card",
        }
    }
}

impl std::str::FromStr for ProcessingType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "card" => Ok(Self::Card),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ProcessingType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ProcessingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ProcessingType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ProcessingType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ProcessingType"))
    }
}
pub mod card;
pub use card::Card;
