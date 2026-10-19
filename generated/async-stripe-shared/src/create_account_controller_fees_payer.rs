#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateAccountControllerFeesPayer {
    Account,
    Application,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateAccountControllerFeesPayer {
    pub fn as_str(&self) -> &str {
        use CreateAccountControllerFeesPayer::*;
        match self {
            Account => "account",
            Application => "application",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateAccountControllerFeesPayer {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountControllerFeesPayer::*;
        match s {
            "account" => Ok(Account),
            "application" => Ok(Application),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateAccountControllerFeesPayer"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateAccountControllerFeesPayer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateAccountControllerFeesPayer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountControllerFeesPayer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateAccountControllerFeesPayer)).finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateAccountControllerFeesPayer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CreateAccountControllerFeesPayer {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CreateAccountControllerFeesPayer> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CreateAccountControllerFeesPayer::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CreateAccountControllerFeesPayer);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateAccountControllerFeesPayer {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
