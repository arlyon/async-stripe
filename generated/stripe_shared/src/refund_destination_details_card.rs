#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RefundDestinationDetailsCard {
    /// Value of the reference number assigned to the refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// Status of the reference number on the refund. This can be `pending`, `available` or `unavailable`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_status: Option<String>,
    /// Type of the reference number assigned to the refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_type: Option<String>,
    /// The type of refund. This can be `refund`, `reversal`, or `pending`.
    #[serde(rename = "type")]
    pub type_: RefundDestinationDetailsCardType,
}
/// The type of refund. This can be `refund`, `reversal`, or `pending`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RefundDestinationDetailsCardType {
    Pending,
    Refund,
    Reversal,
}
impl RefundDestinationDetailsCardType {
    pub fn as_str(self) -> &'static str {
        use RefundDestinationDetailsCardType::*;
        match self {
            Pending => "pending",
            Refund => "refund",
            Reversal => "reversal",
        }
    }
}

impl std::str::FromStr for RefundDestinationDetailsCardType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RefundDestinationDetailsCardType::*;
        match s {
            "pending" => Ok(Pending),
            "refund" => Ok(Refund),
            "reversal" => Ok(Reversal),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for RefundDestinationDetailsCardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for RefundDestinationDetailsCardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for RefundDestinationDetailsCardType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RefundDestinationDetailsCardType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for RefundDestinationDetailsCardType")
        })
    }
}
