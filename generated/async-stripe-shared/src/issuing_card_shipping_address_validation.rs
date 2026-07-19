#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingCardShippingAddressValidation {
    /// The address validation capabilities to use.
    pub mode: IssuingCardShippingAddressValidationMode,
    /// The normalized shipping address.
    pub normalized_address: Option<stripe_shared::Address>,
    /// The validation result for the shipping address.
    pub result: Option<IssuingCardShippingAddressValidationResult>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingCardShippingAddressValidation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingCardShippingAddressValidation").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingCardShippingAddressValidationBuilder {
    mode: Option<IssuingCardShippingAddressValidationMode>,
    normalized_address: Option<Option<stripe_shared::Address>>,
    result: Option<Option<IssuingCardShippingAddressValidationResult>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

    make_place!(Place);

    impl Deserialize for IssuingCardShippingAddressValidation {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingCardShippingAddressValidation>,
        builder: IssuingCardShippingAddressValidationBuilder,
    }

    impl Visitor for Place<IssuingCardShippingAddressValidation> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingCardShippingAddressValidationBuilder {
                    mode: Deserialize::default(),
                    normalized_address: Deserialize::default(),
                    result: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "mode" => Deserialize::begin(&mut self.builder.mode),
                "normalized_address" => Deserialize::begin(&mut self.builder.normalized_address),
                "result" => Deserialize::begin(&mut self.builder.result),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(mode), Some(normalized_address), Some(result)) = (
                self.builder.mode.take(),
                self.builder.normalized_address.take(),
                self.builder.result.take(),
            ) else {
                return Ok(());
            };
            *self.out =
                Some(IssuingCardShippingAddressValidation { mode, normalized_address, result });
            Ok(())
        }
    }
};
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
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingCardShippingAddressValidationMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for IssuingCardShippingAddressValidationMode {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<IssuingCardShippingAddressValidationMode> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingCardShippingAddressValidationMode::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingCardShippingAddressValidationMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The validation result for the shipping address.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingCardShippingAddressValidationResult {
    Indeterminate,
    LikelyDeliverable,
    LikelyUndeliverable,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingCardShippingAddressValidationResult {
    pub fn as_str(&self) -> &str {
        use IssuingCardShippingAddressValidationResult::*;
        match self {
            Indeterminate => "indeterminate",
            LikelyDeliverable => "likely_deliverable",
            LikelyUndeliverable => "likely_undeliverable",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingCardShippingAddressValidationResult {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardShippingAddressValidationResult::*;
        match s {
            "indeterminate" => Ok(Indeterminate),
            "likely_deliverable" => Ok(LikelyDeliverable),
            "likely_undeliverable" => Ok(LikelyUndeliverable),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "IssuingCardShippingAddressValidationResult"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IssuingCardShippingAddressValidationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for IssuingCardShippingAddressValidationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingCardShippingAddressValidationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(IssuingCardShippingAddressValidationResult))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingCardShippingAddressValidationResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for IssuingCardShippingAddressValidationResult {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<IssuingCardShippingAddressValidationResult> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(IssuingCardShippingAddressValidationResult::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingCardShippingAddressValidationResult {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
