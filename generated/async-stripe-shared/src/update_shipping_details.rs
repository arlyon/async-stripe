/// Determines which entity is allowed to update the shipping details.
///
/// Default is `client_only`.
/// Stripe Checkout client will automatically update the shipping details.
/// If set to `server_only`, only your server is allowed to update the shipping details.
///
/// This parameter is only supported when `ui_mode=elements`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateShippingDetails {
    ClientOnly,
    ServerOnly,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdateShippingDetails {
    pub fn as_str(&self) -> &str {
        use UpdateShippingDetails::*;
        match self {
            ClientOnly => "client_only",
            ServerOnly => "server_only",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdateShippingDetails {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateShippingDetails::*;
        match s {
            "client_only" => Ok(ClientOnly),
            "server_only" => Ok(ServerOnly),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "UpdateShippingDetails");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdateShippingDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for UpdateShippingDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateShippingDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(UpdateShippingDetails)).finish_non_exhaustive()
    }
}
impl serde::Serialize for UpdateShippingDetails {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for UpdateShippingDetails {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<UpdateShippingDetails> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(UpdateShippingDetails::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(UpdateShippingDetails);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateShippingDetails {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
