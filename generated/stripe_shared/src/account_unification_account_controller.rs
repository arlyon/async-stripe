#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AccountUnificationAccountController {
    /// `true` if the Connect application retrieving the resource controls the account and can therefore exercise [platform controls](https://stripe.com/docs/connect/platform-controls-for-standard-accounts).
    /// Otherwise, this field is null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_controller: Option<bool>,
    /// The controller type.
    /// Can be `application`, if a Connect application controls the account, or `account`, if the account controls itself.
    #[serde(rename = "type")]
    pub type_: AccountUnificationAccountControllerType,
}
/// The controller type.
/// Can be `application`, if a Connect application controls the account, or `account`, if the account controls itself.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountUnificationAccountControllerType {
    Account,
    Application,
}
impl AccountUnificationAccountControllerType {
    pub fn as_str(self) -> &'static str {
        use AccountUnificationAccountControllerType::*;
        match self {
            Account => "account",
            Application => "application",
        }
    }
}

impl std::str::FromStr for AccountUnificationAccountControllerType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountUnificationAccountControllerType::*;
        match s {
            "account" => Ok(Account),
            "application" => Ok(Application),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for AccountUnificationAccountControllerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountUnificationAccountControllerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountUnificationAccountControllerType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountUnificationAccountControllerType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountUnificationAccountControllerType")
        })
    }
}
