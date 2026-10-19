/// The customer's preferred card network for co-branded cards.
/// Supports `cartes_bancaires`, `mastercard`, or `visa`.
/// Selection of a network that does not apply to the card will be stored as `invalid_preference` on the card.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum Preferred {
    CartesBancaires,
    Mastercard,
    Visa,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl Preferred {
    pub fn as_str(&self) -> &str {
        use Preferred::*;
        match self {
            CartesBancaires => "cartes_bancaires",
            Mastercard => "mastercard",
            Visa => "visa",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for Preferred {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Preferred::*;
        match s {
            "cartes_bancaires" => Ok(CartesBancaires),
            "mastercard" => Ok(Mastercard),
            "visa" => Ok(Visa),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "Preferred");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for Preferred {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for Preferred {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for Preferred {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(Preferred)).finish_non_exhaustive()
    }
}
impl serde::Serialize for Preferred {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for Preferred {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
