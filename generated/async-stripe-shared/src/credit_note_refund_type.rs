#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreditNoteRefundType {
    PaymentRecordRefund,
    Refund,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreditNoteRefundType {
    pub fn as_str(&self) -> &str {
        use CreditNoteRefundType::*;
        match self {
            PaymentRecordRefund => "payment_record_refund",
            Refund => "refund",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreditNoteRefundType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreditNoteRefundType::*;
        match s {
            "payment_record_refund" => Ok(PaymentRecordRefund),
            "refund" => Ok(Refund),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "CreditNoteRefundType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreditNoteRefundType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreditNoteRefundType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreditNoteRefundType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreditNoteRefundType)).finish_non_exhaustive()
    }
}
impl serde::Serialize for CreditNoteRefundType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CreditNoteRefundType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CreditNoteRefundType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CreditNoteRefundType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CreditNoteRefundType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreditNoteRefundType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
