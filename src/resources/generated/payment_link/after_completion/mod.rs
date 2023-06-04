#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AfterCompletion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_confirmation:
        Option<crate::payment_link::after_completion::hosted_confirmation::HostedConfirmation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<crate::payment_link::after_completion::redirect::Redirect>,
    /// The specified behavior after the purchase is complete.
    #[serde(rename = "type")]
    pub type_: AfterCompletionType,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AfterCompletion {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The specified behavior after the purchase is complete.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
pub mod hosted_confirmation;
pub use hosted_confirmation::HostedConfirmation;
pub mod redirect;
pub use redirect::Redirect;
