#[derive(Clone, Debug)]
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
#[doc(hidden)]
pub struct IssuingCardShippingAddressValidationBuilder {
    mode: Option<IssuingCardShippingAddressValidationMode>,
    normalized_address: Option<Option<stripe_shared::Address>>,
    result: Option<Option<IssuingCardShippingAddressValidationResult>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: IssuingCardShippingAddressValidationBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingCardShippingAddressValidationBuilder {
        type Out = IssuingCardShippingAddressValidation;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "mode" => Deserialize::begin(&mut self.mode),
                "normalized_address" => Deserialize::begin(&mut self.normalized_address),
                "result" => Deserialize::begin(&mut self.result),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                mode: Deserialize::default(),
                normalized_address: Deserialize::default(),
                result: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(mode), Some(normalized_address), Some(result)) =
                (self.mode, self.normalized_address.take(), self.result)
            else {
                return None;
            };
            Some(Self::Out { mode, normalized_address, result })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for IssuingCardShippingAddressValidation {
        type Builder = IssuingCardShippingAddressValidationBuilder;
    }

    impl FromValueOpt for IssuingCardShippingAddressValidation {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingCardShippingAddressValidationBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "mode" => b.mode = FromValueOpt::from_value(v),
                    "normalized_address" => b.normalized_address = FromValueOpt::from_value(v),
                    "result" => b.result = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The address validation capabilities to use.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingCardShippingAddressValidationMode {
    Disabled,
    NormalizationOnly,
    ValidationAndNormalization,
}
impl IssuingCardShippingAddressValidationMode {
    pub fn as_str(self) -> &'static str {
        use IssuingCardShippingAddressValidationMode::*;
        match self {
            Disabled => "disabled",
            NormalizationOnly => "normalization_only",
            ValidationAndNormalization => "validation_and_normalization",
        }
    }
}

impl std::str::FromStr for IssuingCardShippingAddressValidationMode {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardShippingAddressValidationMode::*;
        match s {
            "disabled" => Ok(Disabled),
            "normalization_only" => Ok(NormalizationOnly),
            "validation_and_normalization" => Ok(ValidationAndNormalization),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IssuingCardShippingAddressValidationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingCardShippingAddressValidationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for IssuingCardShippingAddressValidationMode {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingCardShippingAddressValidationMode> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            IssuingCardShippingAddressValidationMode::from_str(s).map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingCardShippingAddressValidationMode);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingCardShippingAddressValidationMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for IssuingCardShippingAddressValidationMode")
        })
    }
}
/// The validation result for the shipping address.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingCardShippingAddressValidationResult {
    Indeterminate,
    LikelyDeliverable,
    LikelyUndeliverable,
}
impl IssuingCardShippingAddressValidationResult {
    pub fn as_str(self) -> &'static str {
        use IssuingCardShippingAddressValidationResult::*;
        match self {
            Indeterminate => "indeterminate",
            LikelyDeliverable => "likely_deliverable",
            LikelyUndeliverable => "likely_undeliverable",
        }
    }
}

impl std::str::FromStr for IssuingCardShippingAddressValidationResult {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardShippingAddressValidationResult::*;
        match s {
            "indeterminate" => Ok(Indeterminate),
            "likely_deliverable" => Ok(LikelyDeliverable),
            "likely_undeliverable" => Ok(LikelyUndeliverable),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IssuingCardShippingAddressValidationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingCardShippingAddressValidationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for IssuingCardShippingAddressValidationResult {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingCardShippingAddressValidationResult> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            IssuingCardShippingAddressValidationResult::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingCardShippingAddressValidationResult);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingCardShippingAddressValidationResult {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for IssuingCardShippingAddressValidationResult")
        })
    }
}
