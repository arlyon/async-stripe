#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SecretServiceResourceScope {
    /// The secret scope type.
    #[serde(rename = "type")]
    pub type_: SecretServiceResourceScopeType,
    /// The user ID, if type is set to "user"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}
/// The secret scope type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SecretServiceResourceScopeType {
    Account,
    User,
}
impl SecretServiceResourceScopeType {
    pub fn as_str(self) -> &'static str {
        use SecretServiceResourceScopeType::*;
        match self {
            Account => "account",
            User => "user",
        }
    }
}

impl std::str::FromStr for SecretServiceResourceScopeType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SecretServiceResourceScopeType::*;
        match s {
            "account" => Ok(Account),
            "user" => Ok(User),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for SecretServiceResourceScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SecretServiceResourceScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SecretServiceResourceScopeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SecretServiceResourceScopeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for SecretServiceResourceScopeType")
        })
    }
}
