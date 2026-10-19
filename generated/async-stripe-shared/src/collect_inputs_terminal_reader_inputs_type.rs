#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CollectInputsTerminalReaderInputsType {
    Email,
    Numeric,
    Phone,
    Selection,
    Signature,
    Text,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CollectInputsTerminalReaderInputsType {
    pub fn as_str(&self) -> &str {
        use CollectInputsTerminalReaderInputsType::*;
        match self {
            Email => "email",
            Numeric => "numeric",
            Phone => "phone",
            Selection => "selection",
            Signature => "signature",
            Text => "text",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CollectInputsTerminalReaderInputsType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CollectInputsTerminalReaderInputsType::*;
        match s {
            "email" => Ok(Email),
            "numeric" => Ok(Numeric),
            "phone" => Ok(Phone),
            "selection" => Ok(Selection),
            "signature" => Ok(Signature),
            "text" => Ok(Text),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CollectInputsTerminalReaderInputsType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CollectInputsTerminalReaderInputsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CollectInputsTerminalReaderInputsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CollectInputsTerminalReaderInputsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CollectInputsTerminalReaderInputsType)).finish_non_exhaustive()
    }
}
impl serde::Serialize for CollectInputsTerminalReaderInputsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CollectInputsTerminalReaderInputsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CollectInputsTerminalReaderInputsType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CollectInputsTerminalReaderInputsType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CollectInputsTerminalReaderInputsType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CollectInputsTerminalReaderInputsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
