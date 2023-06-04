#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CustomerUpdate {
    /// The types of customer updates that are supported.
    ///
    /// When empty, customers are not updateable.
    pub allowed_updates: Vec<CustomerUpdateAllowedUpdates>,
    /// Whether the feature is enabled.
    pub enabled: bool,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CustomerUpdate {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The types of customer updates that are supported.
///
/// When empty, customers are not updateable.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum CustomerUpdateAllowedUpdates {
    Address,
    Email,
    Phone,
    Shipping,
    TaxId,
}

impl CustomerUpdateAllowedUpdates {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Address => "address",
            Self::Email => "email",
            Self::Phone => "phone",
            Self::Shipping => "shipping",
            Self::TaxId => "tax_id",
        }
    }
}

impl AsRef<str> for CustomerUpdateAllowedUpdates {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerUpdateAllowedUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
