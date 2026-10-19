#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductRegistrationsResourceCountryOptionsIgic {
    /// Place of supply scheme used in an IGIC registration.
    pub place_of_supply_scheme: stripe_shared::PlaceOfSupplyScheme,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductRegistrationsResourceCountryOptionsIgic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxProductRegistrationsResourceCountryOptionsIgic").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TaxProductRegistrationsResourceCountryOptionsIgicBuilder {
    place_of_supply_scheme: Option<stripe_shared::PlaceOfSupplyScheme>,
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

    impl Deserialize for TaxProductRegistrationsResourceCountryOptionsIgic {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductRegistrationsResourceCountryOptionsIgic>,
        builder: TaxProductRegistrationsResourceCountryOptionsIgicBuilder,
    }

    impl Visitor for Place<TaxProductRegistrationsResourceCountryOptionsIgic> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxProductRegistrationsResourceCountryOptionsIgicBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TaxProductRegistrationsResourceCountryOptionsIgicBuilder {
        type Out = TaxProductRegistrationsResourceCountryOptionsIgic;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "place_of_supply_scheme" => Deserialize::begin(&mut self.place_of_supply_scheme),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { place_of_supply_scheme: None }
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
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for TaxProductRegistrationsResourceCountryOptionsIgic {
        type Builder = TaxProductRegistrationsResourceCountryOptionsIgicBuilder;
    }

    impl FromValueOpt for TaxProductRegistrationsResourceCountryOptionsIgic {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxProductRegistrationsResourceCountryOptionsIgicBuilder::deser_default();
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
/// Place of supply scheme used in an IGIC registration.
pub type TaxProductRegistrationsResourceCountryOptionsIgicPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
