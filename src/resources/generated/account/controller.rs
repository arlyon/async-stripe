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
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
