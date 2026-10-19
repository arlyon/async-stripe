/// The reason(s) the card logo was rejected.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CardLogo {
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
impl CardLogo {
    pub fn as_str(&self) -> &str {
        use CardLogo::*;
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

impl std::str::FromStr for CardLogo {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CardLogo::*;
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
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "CardLogo");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CardLogo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CardLogo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CardLogo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CardLogo)).finish_non_exhaustive()
    }
}
impl serde::Serialize for CardLogo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CardLogo {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CardLogo> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CardLogo::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CardLogo);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CardLogo {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
