#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductRegistrationsResourceCountryOptionsEuStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme:
        TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductRegistrationsResourceCountryOptionsEuStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxProductRegistrationsResourceCountryOptionsEuStandard")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TaxProductRegistrationsResourceCountryOptionsEuStandardBuilder {
    place_of_supply_scheme:
        Option<TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme>,
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

    impl Deserialize for TaxProductRegistrationsResourceCountryOptionsEuStandard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductRegistrationsResourceCountryOptionsEuStandard>,
        builder: TaxProductRegistrationsResourceCountryOptionsEuStandardBuilder,
    }

    impl Visitor for Place<TaxProductRegistrationsResourceCountryOptionsEuStandard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxProductRegistrationsResourceCountryOptionsEuStandardBuilder {
                    place_of_supply_scheme: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "place_of_supply_scheme" => {
                    Deserialize::begin(&mut self.builder.place_of_supply_scheme)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(place_of_supply_scheme),) = (self.builder.place_of_supply_scheme.take(),)
            else {
                return Ok(());
            };
            *self.out = Some(TaxProductRegistrationsResourceCountryOptionsEuStandard {
                place_of_supply_scheme,
            });
            Ok(())
        }
    }
};
/// Place of supply scheme used in an EU standard registration.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme {
    InboundGoods,
    SmallSeller,
    Standard,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme {
    pub fn as_str(&self) -> &str {
        use TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
            SmallSeller => "small_seller",
            Standard => "standard",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme::*;
        match s {
            "inbound_goods" => Ok(InboundGoods),
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display
    for TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug
    for TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme
        ))
        .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize
    for TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize
    for TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme
{
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
