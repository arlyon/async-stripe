#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingPersonalizationDesignRejectionReasons {
    /// The reason(s) the card logo was rejected.
    pub card_logo: Option<Vec<IssuingPersonalizationDesignRejectionReasonsCardLogo>>,
    /// The reason(s) the carrier text was rejected.
    pub carrier_text: Option<Vec<IssuingPersonalizationDesignRejectionReasonsCarrierText>>,
}
#[doc(hidden)]
pub struct IssuingPersonalizationDesignRejectionReasonsBuilder {
    card_logo: Option<Option<Vec<IssuingPersonalizationDesignRejectionReasonsCardLogo>>>,
    carrier_text: Option<Option<Vec<IssuingPersonalizationDesignRejectionReasonsCarrierText>>>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingPersonalizationDesignRejectionReasons {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingPersonalizationDesignRejectionReasons>,
        builder: IssuingPersonalizationDesignRejectionReasonsBuilder,
    }

    impl Visitor for Place<IssuingPersonalizationDesignRejectionReasons> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingPersonalizationDesignRejectionReasonsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingPersonalizationDesignRejectionReasonsBuilder {
        type Out = IssuingPersonalizationDesignRejectionReasons;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "card_logo" => Deserialize::begin(&mut self.card_logo),
                "carrier_text" => Deserialize::begin(&mut self.carrier_text),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { card_logo: Deserialize::default(), carrier_text: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(card_logo), Some(carrier_text)) =
                (self.card_logo.take(), self.carrier_text.take())
            else {
                return None;
            };
            Some(Self::Out { card_logo, carrier_text })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for IssuingPersonalizationDesignRejectionReasons {
        type Builder = IssuingPersonalizationDesignRejectionReasonsBuilder;
    }

    impl FromValueOpt for IssuingPersonalizationDesignRejectionReasons {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingPersonalizationDesignRejectionReasonsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "card_logo" => b.card_logo = FromValueOpt::from_value(v),
                    "carrier_text" => b.carrier_text = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The reason(s) the card logo was rejected.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingPersonalizationDesignRejectionReasonsCardLogo {
    GeographicLocation,
    Inappropriate,
    NetworkName,
    NonBinaryImage,
    NonFiatCurrency,
    Other,
    OtherEntity,
    PromotionalMaterial,
}
impl IssuingPersonalizationDesignRejectionReasonsCardLogo {
    pub fn as_str(self) -> &'static str {
        use IssuingPersonalizationDesignRejectionReasonsCardLogo::*;
        match self {
            GeographicLocation => "geographic_location",
            Inappropriate => "inappropriate",
            NetworkName => "network_name",
            NonBinaryImage => "non_binary_image",
            NonFiatCurrency => "non_fiat_currency",
            Other => "other",
            OtherEntity => "other_entity",
            PromotionalMaterial => "promotional_material",
        }
    }
}

impl std::str::FromStr for IssuingPersonalizationDesignRejectionReasonsCardLogo {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingPersonalizationDesignRejectionReasonsCardLogo::*;
        match s {
            "geographic_location" => Ok(GeographicLocation),
            "inappropriate" => Ok(Inappropriate),
            "network_name" => Ok(NetworkName),
            "non_binary_image" => Ok(NonBinaryImage),
            "non_fiat_currency" => Ok(NonFiatCurrency),
            "other" => Ok(Other),
            "other_entity" => Ok(OtherEntity),
            "promotional_material" => Ok(PromotionalMaterial),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IssuingPersonalizationDesignRejectionReasonsCardLogo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingPersonalizationDesignRejectionReasonsCardLogo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingPersonalizationDesignRejectionReasonsCardLogo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingPersonalizationDesignRejectionReasonsCardLogo {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingPersonalizationDesignRejectionReasonsCardLogo> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            IssuingPersonalizationDesignRejectionReasonsCardLogo::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingPersonalizationDesignRejectionReasonsCardLogo);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingPersonalizationDesignRejectionReasonsCardLogo {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for IssuingPersonalizationDesignRejectionReasonsCardLogo",
            )
        })
    }
}
/// The reason(s) the carrier text was rejected.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingPersonalizationDesignRejectionReasonsCarrierText {
    GeographicLocation,
    Inappropriate,
    NetworkName,
    NonFiatCurrency,
    Other,
    OtherEntity,
    PromotionalMaterial,
}
impl IssuingPersonalizationDesignRejectionReasonsCarrierText {
    pub fn as_str(self) -> &'static str {
        use IssuingPersonalizationDesignRejectionReasonsCarrierText::*;
        match self {
            GeographicLocation => "geographic_location",
            Inappropriate => "inappropriate",
            NetworkName => "network_name",
            NonFiatCurrency => "non_fiat_currency",
            Other => "other",
            OtherEntity => "other_entity",
            PromotionalMaterial => "promotional_material",
        }
    }
}

impl std::str::FromStr for IssuingPersonalizationDesignRejectionReasonsCarrierText {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingPersonalizationDesignRejectionReasonsCarrierText::*;
        match s {
            "geographic_location" => Ok(GeographicLocation),
            "inappropriate" => Ok(Inappropriate),
            "network_name" => Ok(NetworkName),
            "non_fiat_currency" => Ok(NonFiatCurrency),
            "other" => Ok(Other),
            "other_entity" => Ok(OtherEntity),
            "promotional_material" => Ok(PromotionalMaterial),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IssuingPersonalizationDesignRejectionReasonsCarrierText {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingPersonalizationDesignRejectionReasonsCarrierText {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingPersonalizationDesignRejectionReasonsCarrierText {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingPersonalizationDesignRejectionReasonsCarrierText {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<IssuingPersonalizationDesignRejectionReasonsCarrierText>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            IssuingPersonalizationDesignRejectionReasonsCarrierText::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingPersonalizationDesignRejectionReasonsCarrierText);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingPersonalizationDesignRejectionReasonsCarrierText {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for IssuingPersonalizationDesignRejectionReasonsCarrierText",
            )
        })
    }
}
