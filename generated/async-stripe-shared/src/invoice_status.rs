/// The status of the invoice, one of `draft`, `open`, `paid`, `uncollectible`, or `void`.
/// [Learn more](https://docs.stripe.com/billing/invoices/workflow#workflow-overview).
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum InvoiceStatus {
    Draft,
    Open,
    Paid,
    Uncollectible,
    Void,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl InvoiceStatus {
    pub fn as_str(&self) -> &str {
        use InvoiceStatus::*;
        match self {
            Draft => "draft",
            Open => "open",
            Paid => "paid",
            Uncollectible => "uncollectible",
            Void => "void",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for InvoiceStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoiceStatus::*;
        match s {
            "draft" => Ok(Draft),
            "open" => Ok(Open),
            "paid" => Ok(Paid),
            "uncollectible" => Ok(Uncollectible),
            "void" => Ok(Void),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "InvoiceStatus");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for InvoiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for InvoiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(InvoiceStatus)).finish_non_exhaustive()
    }
}
impl serde::Serialize for InvoiceStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for InvoiceStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<InvoiceStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(InvoiceStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(InvoiceStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InvoiceStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
