#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentLinksResourceAfterCompletion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_confirmation:
        Option<stripe_types::PaymentLinksResourceCompletionBehaviorConfirmationPage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<stripe_types::PaymentLinksResourceCompletionBehaviorRedirect>,
    /// The specified behavior after the purchase is complete.
    #[serde(rename = "type")]
    pub type_: PaymentLinksResourceAfterCompletionType,
}
/// The specified behavior after the purchase is complete.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentLinksResourceAfterCompletionType {
    HostedConfirmation,
    Redirect,
}

impl PaymentLinksResourceAfterCompletionType {
    pub fn as_str(self) -> &'static str {
        use PaymentLinksResourceAfterCompletionType::*;
        match self {
            HostedConfirmation => "hosted_confirmation",
            Redirect => "redirect",
        }
    }
}

impl std::str::FromStr for PaymentLinksResourceAfterCompletionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinksResourceAfterCompletionType::*;
        match s {
            "hosted_confirmation" => Ok(HostedConfirmation),
            "redirect" => Ok(Redirect),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentLinksResourceAfterCompletionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentLinksResourceAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentLinksResourceAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentLinksResourceAfterCompletionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentLinksResourceAfterCompletionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PaymentLinksResourceAfterCompletionType")
        })
    }
}
