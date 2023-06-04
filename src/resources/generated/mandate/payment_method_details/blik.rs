#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Blik {
    /// Date at which the mandate expires.
pub expires_after: Option<crate::Timestamp>,
#[serde(skip_serializing_if = "Option::is_none")]
pub off_session: Option<crate::payment_intent::payment_method_options::blik_mandate_options_off_session_details::BlikMandateOptionsOffSessionDetails>,
    /// Type of the mandate.
#[serde(rename = "type")]
pub type_: Option<BlikType>,

}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Blik {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Type of the mandate.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum BlikType {
    OffSession,
    OnSession,
}

impl BlikType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for BlikType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BlikType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
