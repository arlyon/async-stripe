#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentIntentProcessing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_shared::PaymentIntentCardProcessing>,
    /// Type of the payment method for which payment is in `processing` state, one of `card`.
    #[serde(rename = "type")]
    pub type_: PaymentIntentProcessingType,
}
/// Type of the payment method for which payment is in `processing` state, one of `card`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentIntentProcessingType {
    Card,
}
impl PaymentIntentProcessingType {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentProcessingType::*;
        match self {
            Card => "card",
        }
    }
}

impl std::str::FromStr for PaymentIntentProcessingType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentProcessingType::*;
        match s {
            "card" => Ok(Card),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for PaymentIntentProcessingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentIntentProcessingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentIntentProcessingType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentIntentProcessingType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for PaymentIntentProcessingType"))
    }
}
