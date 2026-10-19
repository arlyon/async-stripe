/// The secret scope type.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum FindAppsSecretScopeType {
    Account,
    User,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl FindAppsSecretScopeType {
    pub fn as_str(&self) -> &str {
        use FindAppsSecretScopeType::*;
        match self {
            Account => "account",
            User => "user",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for FindAppsSecretScopeType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FindAppsSecretScopeType::*;
        match s {
            "account" => Ok(Account),
            "user" => Ok(User),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "FindAppsSecretScopeType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for FindAppsSecretScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for FindAppsSecretScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FindAppsSecretScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(FindAppsSecretScopeType)).finish_non_exhaustive()
    }
}
impl serde::Serialize for FindAppsSecretScopeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for FindAppsSecretScopeType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<FindAppsSecretScopeType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(FindAppsSecretScopeType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(FindAppsSecretScopeType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for FindAppsSecretScopeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
