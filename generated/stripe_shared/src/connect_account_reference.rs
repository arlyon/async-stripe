#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ConnectAccountReference {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<stripe_types::Expandable<stripe_shared::Account>>,
    /// Type of the account referenced.
    #[serde(rename = "type")]
    pub type_: ConnectAccountReferenceType,
}
/// Type of the account referenced.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConnectAccountReferenceType {
    Account,
    Self_,
}
impl ConnectAccountReferenceType {
    pub fn as_str(self) -> &'static str {
        use ConnectAccountReferenceType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for ConnectAccountReferenceType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConnectAccountReferenceType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for ConnectAccountReferenceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ConnectAccountReferenceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ConnectAccountReferenceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ConnectAccountReferenceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ConnectAccountReferenceType"))
    }
}
