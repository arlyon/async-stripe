#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductRegistrationsResourceCountryOptionsUnitedStates {
    pub local_amusement_tax:
        Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsLocalAmusementTax>,
    pub local_lease_tax:
        Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsLocalLeaseTax>,
    pub mass_transit_parking_tax:
        Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsMassTransitParkingTax>,
    pub parking_tax: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsParkingTax>,
    /// Two-letter US state code ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    pub state: String,
    pub state_sales_tax:
        Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsStateSalesTax>,
    /// Type of registration in the US.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsUsType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductRegistrationsResourceCountryOptionsUnitedStates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxProductRegistrationsResourceCountryOptionsUnitedStates")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TaxProductRegistrationsResourceCountryOptionsUnitedStatesBuilder {
    local_amusement_tax: Option<
        Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsLocalAmusementTax>,
    >,
    local_lease_tax:
        Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsLocalLeaseTax>>,
    mass_transit_parking_tax: Option<
        Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsMassTransitParkingTax>,
    >,
    parking_tax:
        Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsParkingTax>>,
    state: Option<String>,
    state_sales_tax:
        Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsStateSalesTax>>,
    type_: Option<stripe_shared::CreateTaxRegistrationCountryOptionsUsType>,
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

    impl Deserialize for TaxProductRegistrationsResourceCountryOptionsUnitedStates {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductRegistrationsResourceCountryOptionsUnitedStates>,
        builder: TaxProductRegistrationsResourceCountryOptionsUnitedStatesBuilder,
    }

    impl Visitor for Place<TaxProductRegistrationsResourceCountryOptionsUnitedStates> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    TaxProductRegistrationsResourceCountryOptionsUnitedStatesBuilder::deser_default(
                    ),
            }))
        }
    }

    impl MapBuilder for TaxProductRegistrationsResourceCountryOptionsUnitedStatesBuilder {
        type Out = TaxProductRegistrationsResourceCountryOptionsUnitedStates;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "local_amusement_tax" => Deserialize::begin(&mut self.local_amusement_tax),
                "local_lease_tax" => Deserialize::begin(&mut self.local_lease_tax),
                "mass_transit_parking_tax" => {
                    Deserialize::begin(&mut self.mass_transit_parking_tax)
                }
                "parking_tax" => Deserialize::begin(&mut self.parking_tax),
                "state" => Deserialize::begin(&mut self.state),
                "state_sales_tax" => Deserialize::begin(&mut self.state_sales_tax),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                local_amusement_tax: Some(None),
                local_lease_tax: Some(None),
                mass_transit_parking_tax: Some(None),
                parking_tax: Some(None),
                state: None,
                state_sales_tax: Some(None),
                type_: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(local_amusement_tax),
                Some(local_lease_tax),
                Some(mass_transit_parking_tax),
                Some(parking_tax),
                Some(state),
                Some(state_sales_tax),
                Some(type_),
            ) = (
                self.local_amusement_tax.take(),
                self.local_lease_tax.take(),
                self.mass_transit_parking_tax.take(),
                self.parking_tax.take(),
                self.state.take(),
                self.state_sales_tax.take(),
                self.type_.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                local_amusement_tax,
                local_lease_tax,
                mass_transit_parking_tax,
                parking_tax,
                state,
                state_sales_tax,
                type_,
            })
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

    impl ObjectDeser for TaxProductRegistrationsResourceCountryOptionsUnitedStates {
        type Builder = TaxProductRegistrationsResourceCountryOptionsUnitedStatesBuilder;
    }

    impl FromValueOpt for TaxProductRegistrationsResourceCountryOptionsUnitedStates {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                TaxProductRegistrationsResourceCountryOptionsUnitedStatesBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "local_amusement_tax" => b.local_amusement_tax = FromValueOpt::from_value(v),
                    "local_lease_tax" => b.local_lease_tax = FromValueOpt::from_value(v),
                    "mass_transit_parking_tax" => {
                        b.mass_transit_parking_tax = FromValueOpt::from_value(v)
                    }
                    "parking_tax" => b.parking_tax = FromValueOpt::from_value(v),
                    "state" => b.state = FromValueOpt::from_value(v),
                    "state_sales_tax" => b.state_sales_tax = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Type of registration in the US.
pub type TaxProductRegistrationsResourceCountryOptionsUnitedStatesType =
    stripe_shared::CreateTaxRegistrationCountryOptionsUsType;
