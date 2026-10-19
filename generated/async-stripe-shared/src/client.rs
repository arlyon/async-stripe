/// The client type that the end customer will pay from
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum Client {
    Android,
    Ios,
    Web,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl Client {
    pub fn as_str(&self) -> &str {
        use Client::*;
        match self {
            Android => "android",
            Ios => "ios",
            Web => "web",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for Client {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Client::*;
        match s {
            "android" => Ok(Android),
            "ios" => Ok(Ios),
            "web" => Ok(Web),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "Client");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(Client)).finish_non_exhaustive()
    }
}
impl serde::Serialize for Client {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for Client {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<Client> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(Client::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(Client);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for Client {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
