#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct BlikMandateOptions {
    /// Date at which the mandate expires.
pub expires_after: Option<stripe_types::Timestamp>,
#[serde(skip_serializing_if = "Option::is_none")]
pub off_session: Option<stripe_types::payment_intent::payment_method_options::blik_mandate_options_off_session_details::BlikMandateOptionsOffSessionDetails>,
    /// Type of the mandate.
#[serde(rename = "type")]
pub type_: Option<BlikMandateOptionsType>,

}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for BlikMandateOptions {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Type of the mandate.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BlikMandateOptionsType {
    OffSession,
    OnSession,
}

impl BlikMandateOptionsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for BlikMandateOptionsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "off_session" => Ok(Self::OffSession),
            "on_session" => Ok(Self::OnSession),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for BlikMandateOptionsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BlikMandateOptionsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for BlikMandateOptionsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BlikMandateOptionsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for BlikMandateOptionsType"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for BlikMandateOptionsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<BlikMandateOptionsType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BlikMandateOptionsType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
