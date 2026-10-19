#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentLinkCustomFieldsLabelType {
    Custom,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentLinkCustomFieldsLabelType {
    pub fn as_str(&self) -> &str {
        use CreatePaymentLinkCustomFieldsLabelType::*;
        match self {
            Custom => "custom",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreatePaymentLinkCustomFieldsLabelType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentLinkCustomFieldsLabelType::*;
        match s {
            "custom" => Ok(Custom),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreatePaymentLinkCustomFieldsLabelType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreatePaymentLinkCustomFieldsLabelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreatePaymentLinkCustomFieldsLabelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreatePaymentLinkCustomFieldsLabelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreatePaymentLinkCustomFieldsLabelType)).finish_non_exhaustive()
    }
}
impl serde::Serialize for CreatePaymentLinkCustomFieldsLabelType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CreatePaymentLinkCustomFieldsLabelType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CreatePaymentLinkCustomFieldsLabelType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CreatePaymentLinkCustomFieldsLabelType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CreatePaymentLinkCustomFieldsLabelType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePaymentLinkCustomFieldsLabelType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
