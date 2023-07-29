#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DestinationPaymentMethodDetails {
pub billing_details: stripe_treasury::treasury::received_credit::initiating_payment_method_details::billing_details::BillingDetails,
    /// The type of the payment method used in the OutboundTransfer.
#[serde(rename = "type")]
pub type_: DestinationPaymentMethodDetailsType,
#[serde(skip_serializing_if = "Option::is_none")]
pub us_bank_account: Option<stripe_treasury::treasury::outbound_transfer::destination_payment_method_details::us_bank_account::UsBankAccount>,

}
/// The type of the payment method used in the OutboundTransfer.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DestinationPaymentMethodDetailsType {
    UsBankAccount,
}

impl DestinationPaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UsBankAccount => "us_bank_account",
        }
    }
}

impl std::str::FromStr for DestinationPaymentMethodDetailsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "us_bank_account" => Ok(Self::UsBankAccount),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for DestinationPaymentMethodDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DestinationPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for DestinationPaymentMethodDetailsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DestinationPaymentMethodDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for DestinationPaymentMethodDetailsType")
        })
    }
}
pub mod us_bank_account;
pub use us_bank_account::UsBankAccount;
