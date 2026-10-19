#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum InvoiceRenderingTemplateStatus {
    Active,
    Archived,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl InvoiceRenderingTemplateStatus {
    pub fn as_str(&self) -> &str {
        use InvoiceRenderingTemplateStatus::*;
        match self {
            Active => "active",
            Archived => "archived",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for InvoiceRenderingTemplateStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoiceRenderingTemplateStatus::*;
        match s {
            "active" => Ok(Active),
            "archived" => Ok(Archived),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "InvoiceRenderingTemplateStatus"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for InvoiceRenderingTemplateStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for InvoiceRenderingTemplateStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoiceRenderingTemplateStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(InvoiceRenderingTemplateStatus)).finish_non_exhaustive()
    }
}
impl serde::Serialize for InvoiceRenderingTemplateStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for InvoiceRenderingTemplateStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<InvoiceRenderingTemplateStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(InvoiceRenderingTemplateStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(InvoiceRenderingTemplateStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InvoiceRenderingTemplateStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
