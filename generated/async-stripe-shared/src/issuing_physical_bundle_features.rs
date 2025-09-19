#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingPhysicalBundleFeatures {
    /// The policy for how to use card logo images in a card design with this physical bundle.
    pub card_logo: IssuingPhysicalBundleFeaturesCardLogo,
    /// The policy for how to use carrier letter text in a card design with this physical bundle.
    pub carrier_text: IssuingPhysicalBundleFeaturesCarrierText,
    /// The policy for how to use a second line on a card with this physical bundle.
    pub second_line: IssuingPhysicalBundleFeaturesSecondLine,
}
#[doc(hidden)]
pub struct IssuingPhysicalBundleFeaturesBuilder {
    card_logo: Option<IssuingPhysicalBundleFeaturesCardLogo>,
    carrier_text: Option<IssuingPhysicalBundleFeaturesCarrierText>,
    second_line: Option<IssuingPhysicalBundleFeaturesSecondLine>,
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

    impl Deserialize for IssuingPhysicalBundleFeatures {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingPhysicalBundleFeatures>,
        builder: IssuingPhysicalBundleFeaturesBuilder,
    }

    impl Visitor for Place<IssuingPhysicalBundleFeatures> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingPhysicalBundleFeaturesBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingPhysicalBundleFeaturesBuilder {
        type Out = IssuingPhysicalBundleFeatures;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "card_logo" => Deserialize::begin(&mut self.card_logo),
                "carrier_text" => Deserialize::begin(&mut self.carrier_text),
                "second_line" => Deserialize::begin(&mut self.second_line),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                card_logo: Deserialize::default(),
                carrier_text: Deserialize::default(),
                second_line: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(card_logo), Some(carrier_text), Some(second_line)) =
                (self.card_logo, self.carrier_text, self.second_line)
            else {
                return None;
            };
            Some(Self::Out { card_logo, carrier_text, second_line })
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

    impl ObjectDeser for IssuingPhysicalBundleFeatures {
        type Builder = IssuingPhysicalBundleFeaturesBuilder;
    }

    impl FromValueOpt for IssuingPhysicalBundleFeatures {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingPhysicalBundleFeaturesBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "card_logo" => b.card_logo = FromValueOpt::from_value(v),
                    "carrier_text" => b.carrier_text = FromValueOpt::from_value(v),
                    "second_line" => b.second_line = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The policy for how to use card logo images in a card design with this physical bundle.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingPhysicalBundleFeaturesCardLogo {
    Optional,
    Required,
    Unsupported,
}
impl IssuingPhysicalBundleFeaturesCardLogo {
    pub fn as_str(self) -> &'static str {
        use IssuingPhysicalBundleFeaturesCardLogo::*;
        match self {
            Optional => "optional",
            Required => "required",
            Unsupported => "unsupported",
        }
    }
}

impl std::str::FromStr for IssuingPhysicalBundleFeaturesCardLogo {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingPhysicalBundleFeaturesCardLogo::*;
        match s {
            "optional" => Ok(Optional),
            "required" => Ok(Required),
            "unsupported" => Ok(Unsupported),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IssuingPhysicalBundleFeaturesCardLogo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingPhysicalBundleFeaturesCardLogo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingPhysicalBundleFeaturesCardLogo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingPhysicalBundleFeaturesCardLogo {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingPhysicalBundleFeaturesCardLogo> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(IssuingPhysicalBundleFeaturesCardLogo::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingPhysicalBundleFeaturesCardLogo);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingPhysicalBundleFeaturesCardLogo {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for IssuingPhysicalBundleFeaturesCardLogo")
        })
    }
}
/// The policy for how to use carrier letter text in a card design with this physical bundle.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingPhysicalBundleFeaturesCarrierText {
    Optional,
    Required,
    Unsupported,
}
impl IssuingPhysicalBundleFeaturesCarrierText {
    pub fn as_str(self) -> &'static str {
        use IssuingPhysicalBundleFeaturesCarrierText::*;
        match self {
            Optional => "optional",
            Required => "required",
            Unsupported => "unsupported",
        }
    }
}

impl std::str::FromStr for IssuingPhysicalBundleFeaturesCarrierText {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingPhysicalBundleFeaturesCarrierText::*;
        match s {
            "optional" => Ok(Optional),
            "required" => Ok(Required),
            "unsupported" => Ok(Unsupported),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IssuingPhysicalBundleFeaturesCarrierText {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingPhysicalBundleFeaturesCarrierText {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingPhysicalBundleFeaturesCarrierText {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingPhysicalBundleFeaturesCarrierText {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingPhysicalBundleFeaturesCarrierText> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            IssuingPhysicalBundleFeaturesCarrierText::from_str(s).map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingPhysicalBundleFeaturesCarrierText);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingPhysicalBundleFeaturesCarrierText {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for IssuingPhysicalBundleFeaturesCarrierText")
        })
    }
}
/// The policy for how to use a second line on a card with this physical bundle.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingPhysicalBundleFeaturesSecondLine {
    Optional,
    Required,
    Unsupported,
}
impl IssuingPhysicalBundleFeaturesSecondLine {
    pub fn as_str(self) -> &'static str {
        use IssuingPhysicalBundleFeaturesSecondLine::*;
        match self {
            Optional => "optional",
            Required => "required",
            Unsupported => "unsupported",
        }
    }
}

impl std::str::FromStr for IssuingPhysicalBundleFeaturesSecondLine {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingPhysicalBundleFeaturesSecondLine::*;
        match s {
            "optional" => Ok(Optional),
            "required" => Ok(Required),
            "unsupported" => Ok(Unsupported),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IssuingPhysicalBundleFeaturesSecondLine {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingPhysicalBundleFeaturesSecondLine {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingPhysicalBundleFeaturesSecondLine {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingPhysicalBundleFeaturesSecondLine {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingPhysicalBundleFeaturesSecondLine> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            IssuingPhysicalBundleFeaturesSecondLine::from_str(s).map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingPhysicalBundleFeaturesSecondLine);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingPhysicalBundleFeaturesSecondLine {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for IssuingPhysicalBundleFeaturesSecondLine")
        })
    }
}
