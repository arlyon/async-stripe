/// The type of this amount. We currently only support `monetary` billing credits.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateBillingCreditGrantAmountType {
    Monetary,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateBillingCreditGrantAmountType {
    pub fn as_str(&self) -> &str {
        use CreateBillingCreditGrantAmountType::*;
        match self {
            Monetary => "monetary",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateBillingCreditGrantAmountType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingCreditGrantAmountType::*;
        match s {
            "monetary" => Ok(Monetary),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateBillingCreditGrantAmountType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateBillingCreditGrantAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateBillingCreditGrantAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateBillingCreditGrantAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateBillingCreditGrantAmountType)).finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateBillingCreditGrantAmountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CreateBillingCreditGrantAmountType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CreateBillingCreditGrantAmountType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CreateBillingCreditGrantAmountType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CreateBillingCreditGrantAmountType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateBillingCreditGrantAmountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
