/// The purpose of the transaction.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TransactionPurpose {
    Goods,
    Other,
    Services,
    Unspecified,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TransactionPurpose {
    pub fn as_str(&self) -> &str {
        use TransactionPurpose::*;
        match self {
            Goods => "goods",
            Other => "other",
            Services => "services",
            Unspecified => "unspecified",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TransactionPurpose {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TransactionPurpose::*;
        match s {
            "goods" => Ok(Goods),
            "other" => Ok(Other),
            "services" => Ok(Services),
            "unspecified" => Ok(Unspecified),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "TransactionPurpose");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TransactionPurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TransactionPurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TransactionPurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TransactionPurpose)).finish_non_exhaustive()
    }
}
impl serde::Serialize for TransactionPurpose {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TransactionPurpose {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TransactionPurpose> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TransactionPurpose::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TransactionPurpose);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TransactionPurpose {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
