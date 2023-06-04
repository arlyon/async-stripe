#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct MandateOptions {
    /// Transaction type of the mandate.
    pub transaction_type: Option<MandateOptionsTransactionType>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for MandateOptions {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Transaction type of the mandate.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum MandateOptionsTransactionType {
    Business,
    Personal,
}

impl MandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Business => "business",
            Self::Personal => "personal",
        }
    }
}

impl AsRef<str> for MandateOptionsTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for MandateOptionsTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
