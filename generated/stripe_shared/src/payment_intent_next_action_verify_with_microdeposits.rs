#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentIntentNextActionVerifyWithMicrodeposits {
    /// The timestamp when the microdeposits are expected to land.
    pub arrival_date: stripe_types::Timestamp,
    /// The URL for the hosted verification page, which allows customers to verify their bank account.
    pub hosted_verification_url: String,
    /// The type of the microdeposit sent to the customer.
    /// Used to distinguish between different verification methods.
    pub microdeposit_type: Option<PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType>,
}
/// The type of the microdeposit sent to the customer.
/// Used to distinguish between different verification methods.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    Amounts,
    DescriptorCode,
}
impl PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType::*;
        match self {
            Amounts => "amounts",
            DescriptorCode => "descriptor_code",
        }
    }
}

impl std::str::FromStr for PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType::*;
        match s {
            "amounts" => Ok(Amounts),
            "descriptor_code" => Ok(DescriptorCode),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de>
    for PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType",
            )
        })
    }
}
