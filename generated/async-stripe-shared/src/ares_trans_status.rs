/// The `transStatus` returned from the card Issuer’s ACS in the ARes.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum AresTransStatus {
    A,
    C,
    I,
    N,
    R,
    U,
    Y,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl AresTransStatus {
    pub fn as_str(&self) -> &str {
        use AresTransStatus::*;
        match self {
            A => "A",
            C => "C",
            I => "I",
            N => "N",
            R => "R",
            U => "U",
            Y => "Y",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for AresTransStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AresTransStatus::*;
        match s {
            "A" => Ok(A),
            "C" => Ok(C),
            "I" => Ok(I),
            "N" => Ok(N),
            "R" => Ok(R),
            "U" => Ok(U),
            "Y" => Ok(Y),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "AresTransStatus");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for AresTransStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for AresTransStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AresTransStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(AresTransStatus)).finish_non_exhaustive()
    }
}
impl serde::Serialize for AresTransStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for AresTransStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
