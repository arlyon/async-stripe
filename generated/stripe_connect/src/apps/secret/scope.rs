#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Scope {
    /// The secret scope type.
    #[serde(rename = "type")]
    pub type_: ScopeType,
    /// The user ID, if type is set to "user".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Scope {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The secret scope type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum ScopeType {
    Account,
    User,
}

impl ScopeType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Account => "account",
            Self::User => "user",
        }
    }
}

impl AsRef<str> for ScopeType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
