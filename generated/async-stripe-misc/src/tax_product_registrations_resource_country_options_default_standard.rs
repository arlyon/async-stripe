#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductRegistrationsResourceCountryOptionsDefaultStandard {
    /// Place of supply scheme used in an Default standard registration.
    pub place_of_supply_scheme:
        TaxProductRegistrationsResourceCountryOptionsDefaultStandardPlaceOfSupplyScheme,
}
#[doc(hidden)]
pub struct TaxProductRegistrationsResourceCountryOptionsDefaultStandardBuilder {
    place_of_supply_scheme:
        Option<TaxProductRegistrationsResourceCountryOptionsDefaultStandardPlaceOfSupplyScheme>,
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

    impl Deserialize for TaxProductRegistrationsResourceCountryOptionsDefaultStandard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductRegistrationsResourceCountryOptionsDefaultStandard>,
        builder: TaxProductRegistrationsResourceCountryOptionsDefaultStandardBuilder,
    }

    impl Visitor for Place<TaxProductRegistrationsResourceCountryOptionsDefaultStandard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: TaxProductRegistrationsResourceCountryOptionsDefaultStandardBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for TaxProductRegistrationsResourceCountryOptionsDefaultStandardBuilder {
        type Out = TaxProductRegistrationsResourceCountryOptionsDefaultStandard;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "place_of_supply_scheme" => Deserialize::begin(&mut self.place_of_supply_scheme),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { place_of_supply_scheme: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(place_of_supply_scheme),) = (self.place_of_supply_scheme.take(),) else {
                return None;
            };
            Some(Self::Out { place_of_supply_scheme })
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

    impl ObjectDeser for TaxProductRegistrationsResourceCountryOptionsDefaultStandard {
        type Builder = TaxProductRegistrationsResourceCountryOptionsDefaultStandardBuilder;
    }

    impl FromValueOpt for TaxProductRegistrationsResourceCountryOptionsDefaultStandard {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                TaxProductRegistrationsResourceCountryOptionsDefaultStandardBuilder::deser_default(
                );
            for (k, v) in obj {
                match k.as_str() {
                    "place_of_supply_scheme" => {
                        b.place_of_supply_scheme = FromValueOpt::from_value(v)
                    }
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Place of supply scheme used in an Default standard registration.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TaxProductRegistrationsResourceCountryOptionsDefaultStandardPlaceOfSupplyScheme {
    InboundGoods,
    Standard,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TaxProductRegistrationsResourceCountryOptionsDefaultStandardPlaceOfSupplyScheme {
    pub fn as_str(&self) -> &str {
        use TaxProductRegistrationsResourceCountryOptionsDefaultStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
            Standard => "standard",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for TaxProductRegistrationsResourceCountryOptionsDefaultStandardPlaceOfSupplyScheme
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductRegistrationsResourceCountryOptionsDefaultStandardPlaceOfSupplyScheme::*;
        match s {
            "inbound_goods" => Ok(InboundGoods),
            "standard" => Ok(Standard),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TaxProductRegistrationsResourceCountryOptionsDefaultStandardPlaceOfSupplyScheme"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display
    for TaxProductRegistrationsResourceCountryOptionsDefaultStandardPlaceOfSupplyScheme
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for TaxProductRegistrationsResourceCountryOptionsDefaultStandardPlaceOfSupplyScheme
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize
    for TaxProductRegistrationsResourceCountryOptionsDefaultStandardPlaceOfSupplyScheme
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for TaxProductRegistrationsResourceCountryOptionsDefaultStandardPlaceOfSupplyScheme
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<
        TaxProductRegistrationsResourceCountryOptionsDefaultStandardPlaceOfSupplyScheme,
    >
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxProductRegistrationsResourceCountryOptionsDefaultStandardPlaceOfSupplyScheme::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    TaxProductRegistrationsResourceCountryOptionsDefaultStandardPlaceOfSupplyScheme
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for TaxProductRegistrationsResourceCountryOptionsDefaultStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
