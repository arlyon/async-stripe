#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DisputePaymentMethodDetails {
    /// Card specific dispute details.
    pub card: Option<stripe_shared::DisputePaymentMethodDetailsCard>,
    /// Payment method type.
    #[serde(rename = "type")]
    pub type_: DisputePaymentMethodDetailsType,
}
/// Payment method type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DisputePaymentMethodDetailsType {
    Card,
}
impl DisputePaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        use DisputePaymentMethodDetailsType::*;
        match self {
            Card => "card",
        }
    }
}

impl std::str::FromStr for DisputePaymentMethodDetailsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DisputePaymentMethodDetailsType::*;
        match s {
            "card" => Ok(Card),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for DisputePaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for DisputePaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for DisputePaymentMethodDetailsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DisputePaymentMethodDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for DisputePaymentMethodDetailsType")
        })
    }
}
