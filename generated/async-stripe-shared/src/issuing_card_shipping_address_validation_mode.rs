/// The address validation capabilities to use.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingCardShippingAddressValidationMode {
    Disabled,
    NormalizationOnly,
    ValidationAndNormalization,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingCardShippingAddressValidationMode {
    pub fn as_str(&self) -> &str {
        use IssuingCardShippingAddressValidationMode::*;
        match self {
            Disabled => "disabled",
            NormalizationOnly => "normalization_only",
            ValidationAndNormalization => "validation_and_normalization",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingCardShippingAddressValidationMode {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardShippingAddressValidationMode::*;
        match s {
            "disabled" => Ok(Disabled),
            "normalization_only" => Ok(NormalizationOnly),
            "validation_and_normalization" => Ok(ValidationAndNormalization),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "IssuingCardShippingAddressValidationMode"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IssuingCardShippingAddressValidationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for IssuingCardShippingAddressValidationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingCardShippingAddressValidationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(IssuingCardShippingAddressValidationMode)).finish_non_exhaustive()
    }
}
impl serde::Serialize for IssuingCardShippingAddressValidationMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingCardShippingAddressValidationMode {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingCardShippingAddressValidationMode> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingCardShippingAddressValidationMode::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingCardShippingAddressValidationMode);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingCardShippingAddressValidationMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
