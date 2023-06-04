#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Processing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<crate::payment_intent::processing::card::Card>,
    /// Type of the payment method for which payment is in `processing` state, one of `card`.
    #[serde(rename = "type")]
    pub type_: ProcessingType,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Processing {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Type of the payment method for which payment is in `processing` state, one of `card`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
pub mod card;
pub use card::Card;
