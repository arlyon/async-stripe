#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct VerifyWithMicrodeposits {
    /// The timestamp when the microdeposits are expected to land.
    pub arrival_date: stripe_types::Timestamp,
    /// The URL for the hosted verification page, which allows customers to verify their bank account.
    pub hosted_verification_url: String,
    /// The type of the microdeposit sent to the customer.
    ///
    /// Used to distinguish between different verification methods.
    pub microdeposit_type: Option<VerifyWithMicrodepositsMicrodepositType>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for VerifyWithMicrodeposits {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The type of the microdeposit sent to the customer.
///
/// Used to distinguish between different verification methods.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum VerifyWithMicrodepositsMicrodepositType {
    Amounts,
    DescriptorCode,
}

impl VerifyWithMicrodepositsMicrodepositType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Amounts => "amounts",
            Self::DescriptorCode => "descriptor_code",
        }
    }
}

impl AsRef<str> for VerifyWithMicrodepositsMicrodepositType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for VerifyWithMicrodepositsMicrodepositType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
