/// Type of the pretax credit amount referenced.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreditNotesPretaxCreditAmountType {
    CreditBalanceTransaction,
    Discount,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreditNotesPretaxCreditAmountType {
    pub fn as_str(&self) -> &str {
        use CreditNotesPretaxCreditAmountType::*;
        match self {
            CreditBalanceTransaction => "credit_balance_transaction",
            Discount => "discount",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreditNotesPretaxCreditAmountType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreditNotesPretaxCreditAmountType::*;
        match s {
            "credit_balance_transaction" => Ok(CreditBalanceTransaction),
            "discount" => Ok(Discount),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreditNotesPretaxCreditAmountType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreditNotesPretaxCreditAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreditNotesPretaxCreditAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreditNotesPretaxCreditAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreditNotesPretaxCreditAmountType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CreditNotesPretaxCreditAmountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CreditNotesPretaxCreditAmountType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CreditNotesPretaxCreditAmountType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CreditNotesPretaxCreditAmountType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CreditNotesPretaxCreditAmountType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreditNotesPretaxCreditAmountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
