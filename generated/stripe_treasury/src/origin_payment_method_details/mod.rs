#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct OriginPaymentMethodDetails {
    pub billing_details: stripe_treasury::billing_details::BillingDetails,
    /// The type of the payment method used in the InboundTransfer.
    #[serde(rename = "type")]
    pub type_: OriginPaymentMethodDetailsType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<stripe_treasury::us_bank_account::UsBankAccount>,
}
/// The type of the payment method used in the InboundTransfer.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OriginPaymentMethodDetailsType {
    UsBankAccount,
}

impl OriginPaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        use OriginPaymentMethodDetailsType::*;
        match self {
            UsBankAccount => "us_bank_account",
        }
    }
}

impl std::str::FromStr for OriginPaymentMethodDetailsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use OriginPaymentMethodDetailsType::*;
        match s {
            "us_bank_account" => Ok(UsBankAccount),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for OriginPaymentMethodDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OriginPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for OriginPaymentMethodDetailsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for OriginPaymentMethodDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom("Unknown value for OriginPaymentMethodDetailsType")
        })
    }
}
