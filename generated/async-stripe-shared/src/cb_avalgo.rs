/// The cryptogram calculation algorithm used by the card Issuer's ACS
/// to calculate the Authentication cryptogram. Also known as `cavvAlgorithm`.
/// messageExtension: CB-AVALGO
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CbAvalgo {
    V0,
    V1,
    V2,
    V3,
    V4,
    A,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CbAvalgo {
    pub fn as_str(&self) -> &str {
        use CbAvalgo::*;
        match self {
            V0 => "0",
            V1 => "1",
            V2 => "2",
            V3 => "3",
            V4 => "4",
            A => "A",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CbAvalgo {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CbAvalgo::*;
        match s {
            "0" => Ok(V0),
            "1" => Ok(V1),
            "2" => Ok(V2),
            "3" => Ok(V3),
            "4" => Ok(V4),
            "A" => Ok(A),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "CbAvalgo");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CbAvalgo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CbAvalgo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CbAvalgo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CbAvalgo)).finish_non_exhaustive()
    }
}
impl serde::Serialize for CbAvalgo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CbAvalgo {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
