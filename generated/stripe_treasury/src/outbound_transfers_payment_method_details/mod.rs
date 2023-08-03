#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct OutboundTransfersPaymentMethodDetails {
    pub billing_details: stripe_treasury::TreasurySharedResourceBillingDetails,
    /// The type of the payment method used in the OutboundTransfer.
    #[serde(rename = "type")]
    pub type_: OutboundTransfersPaymentMethodDetailsType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<stripe_treasury::OutboundTransfersPaymentMethodDetailsUsBankAccount>,
}
/// The type of the payment method used in the OutboundTransfer.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum OutboundTransfersPaymentMethodDetailsType {
    UsBankAccount,
}

impl OutboundTransfersPaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        use OutboundTransfersPaymentMethodDetailsType::*;
        match self {
            UsBankAccount => "us_bank_account",
        }
    }
}

impl std::str::FromStr for OutboundTransfersPaymentMethodDetailsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use OutboundTransfersPaymentMethodDetailsType::*;
        match s {
            "us_bank_account" => Ok(UsBankAccount),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for OutboundTransfersPaymentMethodDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OutboundTransfersPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for OutboundTransfersPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for OutboundTransfersPaymentMethodDetailsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for OutboundTransfersPaymentMethodDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for OutboundTransfersPaymentMethodDetailsType"))
    }
}
