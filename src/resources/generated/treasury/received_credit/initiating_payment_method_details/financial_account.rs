#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct FinancialAccount {
    /// The FinancialAccount ID.
    pub id: String,
    /// The rails the ReceivedCredit was sent over.
    ///
    /// A FinancialAccount can only send funds over `stripe`.
    pub network: FinancialAccountNetwork,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for FinancialAccount {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The rails the ReceivedCredit was sent over.
///
/// A FinancialAccount can only send funds over `stripe`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum FinancialAccountNetwork {
    Stripe,
}

impl FinancialAccountNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Stripe => "stripe",
        }
    }
}

impl AsRef<str> for FinancialAccountNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FinancialAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl crate::Object for FinancialAccount {
    type Id = String;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
