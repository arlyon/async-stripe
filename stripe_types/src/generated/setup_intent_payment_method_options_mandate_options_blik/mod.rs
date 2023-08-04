#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SetupIntentPaymentMethodOptionsMandateOptionsBlik {
    /// Date at which the mandate expires.
    pub expires_after: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_session: Option<stripe_types::MandateOptionsOffSessionDetailsBlik>,
    /// Type of the mandate.
    #[serde(rename = "type")]
    pub type_: Option<SetupIntentPaymentMethodOptionsMandateOptionsBlikType>,
}
/// Type of the mandate.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SetupIntentPaymentMethodOptionsMandateOptionsBlikType {
    OffSession,
    OnSession,
}

impl SetupIntentPaymentMethodOptionsMandateOptionsBlikType {
    pub fn as_str(self) -> &'static str {
        use SetupIntentPaymentMethodOptionsMandateOptionsBlikType::*;
        match self {
            OffSession => "off_session",
            OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for SetupIntentPaymentMethodOptionsMandateOptionsBlikType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupIntentPaymentMethodOptionsMandateOptionsBlikType::*;
        match s {
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for SetupIntentPaymentMethodOptionsMandateOptionsBlikType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupIntentPaymentMethodOptionsMandateOptionsBlikType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SetupIntentPaymentMethodOptionsMandateOptionsBlikType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SetupIntentPaymentMethodOptionsMandateOptionsBlikType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetupIntentPaymentMethodOptionsMandateOptionsBlikType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for SetupIntentPaymentMethodOptionsMandateOptionsBlikType",
            )
        })
    }
}
