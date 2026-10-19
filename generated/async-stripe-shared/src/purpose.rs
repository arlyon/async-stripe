/// The purpose for which payments are made. Has a default value based on your merchant category code.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum Purpose {
    DependantSupport,
    Government,
    Loan,
    Mortgage,
    Other,
    Pension,
    Personal,
    Retail,
    Salary,
    Tax,
    Utility,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl Purpose {
    pub fn as_str(&self) -> &str {
        use Purpose::*;
        match self {
            DependantSupport => "dependant_support",
            Government => "government",
            Loan => "loan",
            Mortgage => "mortgage",
            Other => "other",
            Pension => "pension",
            Personal => "personal",
            Retail => "retail",
            Salary => "salary",
            Tax => "tax",
            Utility => "utility",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for Purpose {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Purpose::*;
        match s {
            "dependant_support" => Ok(DependantSupport),
            "government" => Ok(Government),
            "loan" => Ok(Loan),
            "mortgage" => Ok(Mortgage),
            "other" => Ok(Other),
            "pension" => Ok(Pension),
            "personal" => Ok(Personal),
            "retail" => Ok(Retail),
            "salary" => Ok(Salary),
            "tax" => Ok(Tax),
            "utility" => Ok(Utility),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "Purpose");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for Purpose {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for Purpose {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for Purpose {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(Purpose)).finish_non_exhaustive()
    }
}
impl serde::Serialize for Purpose {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for Purpose {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<Purpose> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(Purpose::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(Purpose);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for Purpose {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
