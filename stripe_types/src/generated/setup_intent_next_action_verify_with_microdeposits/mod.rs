#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SetupIntentNextActionVerifyWithMicrodeposits {
    /// The timestamp when the microdeposits are expected to land.
    pub arrival_date: stripe_types::Timestamp,
    /// The URL for the hosted verification page, which allows customers to verify their bank account.
    pub hosted_verification_url: String,
    /// The type of the microdeposit sent to the customer.
    ///
    /// Used to distinguish between different verification methods.
    pub microdeposit_type: Option<SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType>,
}
/// The type of the microdeposit sent to the customer.
///
/// Used to distinguish between different verification methods.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    Amounts,
    DescriptorCode,
}

impl SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    pub fn as_str(self) -> &'static str {
        use SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType::*;
        match self {
            Amounts => "amounts",
            DescriptorCode => "descriptor_code",
        }
    }
}

impl std::str::FromStr for SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType::*;
        match s {
            "amounts" => Ok(Amounts),
            "descriptor_code" => Ok(DescriptorCode),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType",
            )
        })
    }
}
