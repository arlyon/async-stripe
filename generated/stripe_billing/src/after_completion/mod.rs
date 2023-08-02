#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AfterCompletion {
    /// Configuration when `after_completion.type=hosted_confirmation`.
    pub hosted_confirmation: Option<stripe_billing::hosted_confirmation::HostedConfirmation>,
    /// Configuration when `after_completion.type=redirect`.
    pub redirect: Option<stripe_billing::redirect::Redirect>,
    /// The specified type of behavior after the flow is completed.
    #[serde(rename = "type")]
    pub type_: AfterCompletionType,
}
/// The specified type of behavior after the flow is completed.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AfterCompletionType {
    HostedConfirmation,
    PortalHomepage,
    Redirect,
}

impl AfterCompletionType {
    pub fn as_str(self) -> &'static str {
        use AfterCompletionType::*;
        match self {
            HostedConfirmation => "hosted_confirmation",
            PortalHomepage => "portal_homepage",
            Redirect => "redirect",
        }
    }
}

impl std::str::FromStr for AfterCompletionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AfterCompletionType::*;
        match s {
            "hosted_confirmation" => Ok(HostedConfirmation),
            "portal_homepage" => Ok(PortalHomepage),
            "redirect" => Ok(Redirect),
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
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for AfterCompletionType"))
    }
}
