#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CustomerAcceptance {
    /// The time at which the customer accepted the Mandate.
    pub accepted_at: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<stripe_core::mandate::customer_acceptance::offline::Offline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<stripe_core::mandate::customer_acceptance::online::Online>,
    /// The type of customer acceptance information included with the Mandate.
    ///
    /// One of `online` or `offline`.
    #[serde(rename = "type")]
    pub type_: CustomerAcceptanceType,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CustomerAcceptance {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The type of customer acceptance information included with the Mandate.
///
/// One of `online` or `offline`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum CustomerAcceptanceType {
    Offline,
    Online,
}

impl CustomerAcceptanceType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Offline => "offline",
            Self::Online => "online",
        }
    }
}

impl AsRef<str> for CustomerAcceptanceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerAcceptanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
pub mod offline;
pub use offline::Offline;
pub mod online;
pub use online::Online;
