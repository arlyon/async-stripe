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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for VerifyWithMicrodepositsMicrodepositType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "amounts" => Ok(Self::Amounts),
            "descriptor_code" => Ok(Self::DescriptorCode),

            _ => Err(()),
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
impl serde::Serialize for VerifyWithMicrodepositsMicrodepositType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for VerifyWithMicrodepositsMicrodepositType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for VerifyWithMicrodepositsMicrodepositType")
        })
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for VerifyWithMicrodepositsMicrodepositType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<VerifyWithMicrodepositsMicrodepositType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(VerifyWithMicrodepositsMicrodepositType::from_str(s)?);
        Ok(())
    }
}
