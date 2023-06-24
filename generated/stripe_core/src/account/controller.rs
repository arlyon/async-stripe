#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Controller {
    /// `true` if the Connect application retrieving the resource controls the account and can therefore exercise [platform controls](https://stripe.com/docs/connect/platform-controls-for-standard-accounts).
    ///
    /// Otherwise, this field is null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_controller: Option<bool>,
    /// The controller type.
    ///
    /// Can be `application`, if a Connect application controls the account, or `account`, if the account controls itself.
    #[serde(rename = "type")]
    pub type_: ControllerType,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Controller {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The controller type.
///
/// Can be `application`, if a Connect application controls the account, or `account`, if the account controls itself.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ControllerType {
    Account,
    Application,
}

impl ControllerType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Account => "account",
            Self::Application => "application",
        }
    }
}

impl std::str::FromStr for ControllerType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "account" => Ok(Self::Account),
            "application" => Ok(Self::Application),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ControllerType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ControllerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ControllerType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ControllerType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ControllerType"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ControllerType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<ControllerType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ControllerType::from_str(s)?);
        Ok(())
    }
}
