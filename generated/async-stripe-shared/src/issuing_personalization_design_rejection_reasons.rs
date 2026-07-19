#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingPersonalizationDesignRejectionReasons {
    /// The reason(s) the card logo was rejected.
    pub card_logo: Option<Vec<IssuingPersonalizationDesignRejectionReasonsCardLogo>>,
    /// The reason(s) the carrier text was rejected.
    pub carrier_text: Option<Vec<IssuingPersonalizationDesignRejectionReasonsCarrierText>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingPersonalizationDesignRejectionReasons {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingPersonalizationDesignRejectionReasons").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingPersonalizationDesignRejectionReasonsBuilder {
    card_logo: Option<Option<Vec<IssuingPersonalizationDesignRejectionReasonsCardLogo>>>,
    carrier_text: Option<Option<Vec<IssuingPersonalizationDesignRejectionReasonsCarrierText>>>,
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
                builder: IssuingPersonalizationDesignRejectionReasonsBuilder {
                    card_logo: Deserialize::default(),
                    carrier_text: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "card_logo" => Deserialize::begin(&mut self.builder.card_logo),
                "carrier_text" => Deserialize::begin(&mut self.builder.carrier_text),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(card_logo), Some(carrier_text)) =
                (self.builder.card_logo.take(), self.builder.carrier_text.take())
            else {
                return Ok(());
            };
            *self.out =
                Some(IssuingPersonalizationDesignRejectionReasons { card_logo, carrier_text });
            Ok(())
        }
    }
};
/// The reason(s) the card logo was rejected.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingPersonalizationDesignRejectionReasonsCardLogo {
    GeographicLocation,
    Inappropriate,
    NetworkName,
    NonBinaryImage,
    NonFiatCurrency,
    Other,
    OtherEntity,
    PromotionalMaterial,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingPersonalizationDesignRejectionReasonsCardLogo {
    pub fn as_str(&self) -> &str {
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
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingPersonalizationDesignRejectionReasonsCardLogo {
    type Err = std::convert::Infallible;
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
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "IssuingPersonalizationDesignRejectionReasonsCardLogo"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IssuingPersonalizationDesignRejectionReasonsCardLogo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for IssuingPersonalizationDesignRejectionReasonsCardLogo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingPersonalizationDesignRejectionReasonsCardLogo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(IssuingPersonalizationDesignRejectionReasonsCardLogo))
            .finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for IssuingPersonalizationDesignRejectionReasonsCardLogo {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<IssuingPersonalizationDesignRejectionReasonsCardLogo> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            IssuingPersonalizationDesignRejectionReasonsCardLogo::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingPersonalizationDesignRejectionReasonsCardLogo {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The reason(s) the carrier text was rejected.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingPersonalizationDesignRejectionReasonsCarrierText {
    GeographicLocation,
    Inappropriate,
    NetworkName,
    NonFiatCurrency,
    Other,
    OtherEntity,
    PromotionalMaterial,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingPersonalizationDesignRejectionReasonsCarrierText {
    pub fn as_str(&self) -> &str {
        use IssuingPersonalizationDesignRejectionReasonsCarrierText::*;
        match self {
            GeographicLocation => "geographic_location",
            Inappropriate => "inappropriate",
            NetworkName => "network_name",
            NonFiatCurrency => "non_fiat_currency",
            Other => "other",
            OtherEntity => "other_entity",
            PromotionalMaterial => "promotional_material",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingPersonalizationDesignRejectionReasonsCarrierText {
    type Err = std::convert::Infallible;
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
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "IssuingPersonalizationDesignRejectionReasonsCarrierText"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IssuingPersonalizationDesignRejectionReasonsCarrierText {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for IssuingPersonalizationDesignRejectionReasonsCarrierText {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingPersonalizationDesignRejectionReasonsCarrierText {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(IssuingPersonalizationDesignRejectionReasonsCarrierText))
            .finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for IssuingPersonalizationDesignRejectionReasonsCarrierText {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<IssuingPersonalizationDesignRejectionReasonsCarrierText>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            IssuingPersonalizationDesignRejectionReasonsCarrierText::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingPersonalizationDesignRejectionReasonsCarrierText {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
