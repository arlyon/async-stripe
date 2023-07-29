#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Scope {
    /// The secret scope type.
    #[serde(rename = "type")]
    pub type_: ScopeType,
    /// The user ID, if type is set to "user".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}
/// The secret scope type.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ScopeType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "account" => Ok(Self::Account),
            "user" => Ok(Self::User),

            _ => Err(()),
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
impl serde::Serialize for ScopeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ScopeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ScopeType"))
    }
}
