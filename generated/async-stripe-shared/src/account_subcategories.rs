/// Restricts the Session to subcategories of accounts that can be linked.
/// Valid subcategories are: `checking`, `savings`, `mortgage`, `line_of_credit`, `credit_card`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum AccountSubcategories {
    Checking,
    CreditCard,
    LineOfCredit,
    Mortgage,
    Savings,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl AccountSubcategories {
    pub fn as_str(&self) -> &str {
        use AccountSubcategories::*;
        match self {
            Checking => "checking",
            CreditCard => "credit_card",
            LineOfCredit => "line_of_credit",
            Mortgage => "mortgage",
            Savings => "savings",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for AccountSubcategories {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountSubcategories::*;
        match s {
            "checking" => Ok(Checking),
            "credit_card" => Ok(CreditCard),
            "line_of_credit" => Ok(LineOfCredit),
            "mortgage" => Ok(Mortgage),
            "savings" => Ok(Savings),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "AccountSubcategories");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for AccountSubcategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for AccountSubcategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountSubcategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(AccountSubcategories)).finish_non_exhaustive()
    }
}
impl serde::Serialize for AccountSubcategories {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for AccountSubcategories {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<AccountSubcategories> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AccountSubcategories::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(AccountSubcategories);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for AccountSubcategories {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
