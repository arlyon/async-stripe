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
    pub type_: TaxProductRegistrationsResourceCountryOptionsUnitedStatesType,
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
    local_amusement_tax:
        Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsLocalAmusementTax>,
    local_lease_tax:
        Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsLocalLeaseTax>,
    mass_transit_parking_tax:
        Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsMassTransitParkingTax>,
    parking_tax: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsParkingTax>,
    state: Option<String>,
    state_sales_tax:
        Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsStateSalesTax>,
    type_: Option<TaxProductRegistrationsResourceCountryOptionsUnitedStatesType>,
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
                "local_amusement_tax" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.local_amusement_tax)
                }
                "local_lease_tax" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.local_lease_tax)
                }
                "mass_transit_parking_tax" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.mass_transit_parking_tax,
                ),
                "parking_tax" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.parking_tax)
                }
                "state" => Deserialize::begin(&mut self.state),
                "state_sales_tax" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.state_sales_tax)
                }
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                local_amusement_tax: None,
                local_lease_tax: None,
                mass_transit_parking_tax: None,
                parking_tax: None,
                state: None,
                state_sales_tax: None,
                type_: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    local_amusement_tax,
                    local_lease_tax,
                    mass_transit_parking_tax,
                    parking_tax,
                    state: Some(state),
                    state_sales_tax,
                    type_: Some(type_),
                } => Some(Self::Out {
                    local_amusement_tax,
                    local_lease_tax,
                    mass_transit_parking_tax,
                    parking_tax,
                    state,
                    state_sales_tax,
                    type_,
                }),
                builder => {
                    *self = builder;
                    None
                }
            }
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
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
/// Type of registration in the US.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    LocalAmusementTax,
    LocalLeaseTax,
    MassTransitParkingTax,
    ParkingTax,
    StateCommunicationsTax,
    StateRetailDeliveryFee,
    StateSalesTax,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    pub fn as_str(&self) -> &str {
        use TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::*;
        match self {
            LocalAmusementTax => "local_amusement_tax",
            LocalLeaseTax => "local_lease_tax",
            MassTransitParkingTax => "mass_transit_parking_tax",
            ParkingTax => "parking_tax",
            StateCommunicationsTax => "state_communications_tax",
            StateRetailDeliveryFee => "state_retail_delivery_fee",
            StateSalesTax => "state_sales_tax",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::*;
        match s {
            "local_amusement_tax" => Ok(LocalAmusementTax),
            "local_lease_tax" => Ok(LocalLeaseTax),
            "mass_transit_parking_tax" => Ok(MassTransitParkingTax),
            "parking_tax" => Ok(ParkingTax),
            "state_communications_tax" => Ok(StateCommunicationsTax),
            "state_retail_delivery_fee" => Ok(StateRetailDeliveryFee),
            "state_sales_tax" => Ok(StateSalesTax),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TaxProductRegistrationsResourceCountryOptionsUnitedStatesType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TaxProductRegistrationsResourceCountryOptionsUnitedStatesType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<TaxProductRegistrationsResourceCountryOptionsUnitedStatesType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    TaxProductRegistrationsResourceCountryOptionsUnitedStatesType
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
