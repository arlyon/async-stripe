#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AfterCompletion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_confirmation: Option<
        stripe_payment::payment_link::after_completion::hosted_confirmation::HostedConfirmation,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<stripe_payment::payment_link::after_completion::redirect::Redirect>,
    /// The specified behavior after the purchase is complete.
    #[serde(rename = "type")]
    pub type_: AfterCompletionType,
}
/// The specified behavior after the purchase is complete.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AfterCompletionType {
    HostedConfirmation,
    Redirect,
}

impl AfterCompletionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HostedConfirmation => "hosted_confirmation",
            Self::Redirect => "redirect",
        }
    }
}

impl std::str::FromStr for AfterCompletionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hosted_confirmation" => Ok(Self::HostedConfirmation),
            "redirect" => Ok(Self::Redirect),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for AfterCompletionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for AfterCompletionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AfterCompletionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for AfterCompletionType"))
    }
}
pub mod hosted_confirmation;
pub use hosted_confirmation::HostedConfirmation;
pub mod redirect;
pub use redirect::Redirect;
