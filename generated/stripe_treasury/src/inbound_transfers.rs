/// For more details see <<https://stripe.com/docs/api/treasury/inbound_transfers>>.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InboundTransfers {
    pub billing_details: stripe_treasury::TreasurySharedResourceBillingDetails,
    /// The type of the payment method used in the InboundTransfer.
    #[serde(rename = "type")]
    pub type_: InboundTransfersType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<stripe_treasury::InboundTransfersPaymentMethodDetailsUsBankAccount>,
}
/// The type of the payment method used in the InboundTransfer.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InboundTransfersType {
    UsBankAccount,
}
impl InboundTransfersType {
    pub fn as_str(self) -> &'static str {
        use InboundTransfersType::*;
        match self {
            UsBankAccount => "us_bank_account",
        }
    }
}

impl std::str::FromStr for InboundTransfersType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InboundTransfersType::*;
        match s {
            "us_bank_account" => Ok(UsBankAccount),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for InboundTransfersType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InboundTransfersType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for InboundTransfersType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InboundTransfersType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for InboundTransfersType"))
    }
}
