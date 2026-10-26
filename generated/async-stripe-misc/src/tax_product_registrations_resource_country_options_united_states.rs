#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductRegistrationsResourceCountryOptionsUnitedStates {
    pub admissions_tax:
        Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsAdmissionsTax>,
    pub attendance_tax:
        Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsAttendanceTax>,
    pub entertainment_tax:
        Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsEntertainmentTax>,
    pub gross_receipts_tax:
        Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsGrossReceiptsTax>,
    pub hospitality_tax:
        Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsHospitalityTax>,
    pub local_amusement_tax:
        Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsLocalAmusementTax>,
    pub local_lease_tax:
        Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsLocalLeaseTax>,
    pub luxury_tax: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsLuxuryTax>,
    pub mass_transit_parking_tax:
        Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsMassTransitParkingTax>,
    pub parking_tax: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsParkingTax>,
    pub resort_tax: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsResortTax>,
    /// Two-letter US state code ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    pub state: String,
    pub state_sales_tax:
        Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsStateSalesTax>,
    pub tourism_tax: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsTourismTax>,
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
    admissions_tax:
        Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsAdmissionsTax>>,
    attendance_tax:
        Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsAttendanceTax>>,
    entertainment_tax: Option<
        Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsEntertainmentTax>,
    >,
    gross_receipts_tax: Option<
        Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsGrossReceiptsTax>,
    >,
    hospitality_tax:
        Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsHospitalityTax>>,
    local_amusement_tax: Option<
        Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsLocalAmusementTax>,
    >,
    local_lease_tax:
        Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsLocalLeaseTax>>,
    luxury_tax:
        Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsLuxuryTax>>,
    mass_transit_parking_tax: Option<
        Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsMassTransitParkingTax>,
    >,
    parking_tax:
        Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsParkingTax>>,
    resort_tax:
        Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsResortTax>>,
    state: Option<String>,
    state_sales_tax:
        Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsStateSalesTax>>,
    tourism_tax:
        Option<Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsUsTourismTax>>,
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
                "admissions_tax" => Deserialize::begin(&mut self.admissions_tax),
                "attendance_tax" => Deserialize::begin(&mut self.attendance_tax),
                "entertainment_tax" => Deserialize::begin(&mut self.entertainment_tax),
                "gross_receipts_tax" => Deserialize::begin(&mut self.gross_receipts_tax),
                "hospitality_tax" => Deserialize::begin(&mut self.hospitality_tax),
                "local_amusement_tax" => Deserialize::begin(&mut self.local_amusement_tax),
                "local_lease_tax" => Deserialize::begin(&mut self.local_lease_tax),
                "luxury_tax" => Deserialize::begin(&mut self.luxury_tax),
                "mass_transit_parking_tax" => {
                    Deserialize::begin(&mut self.mass_transit_parking_tax)
                }
                "parking_tax" => Deserialize::begin(&mut self.parking_tax),
                "resort_tax" => Deserialize::begin(&mut self.resort_tax),
                "state" => Deserialize::begin(&mut self.state),
                "state_sales_tax" => Deserialize::begin(&mut self.state_sales_tax),
                "tourism_tax" => Deserialize::begin(&mut self.tourism_tax),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                admissions_tax: Some(None),
                attendance_tax: Some(None),
                entertainment_tax: Some(None),
                gross_receipts_tax: Some(None),
                hospitality_tax: Some(None),
                local_amusement_tax: Some(None),
                local_lease_tax: Some(None),
                luxury_tax: Some(None),
                mass_transit_parking_tax: Some(None),
                parking_tax: Some(None),
                resort_tax: Some(None),
                state: None,
                state_sales_tax: Some(None),
                tourism_tax: Some(None),
                type_: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(admissions_tax),
                Some(attendance_tax),
                Some(entertainment_tax),
                Some(gross_receipts_tax),
                Some(hospitality_tax),
                Some(local_amusement_tax),
                Some(local_lease_tax),
                Some(luxury_tax),
                Some(mass_transit_parking_tax),
                Some(parking_tax),
                Some(resort_tax),
                Some(state),
                Some(state_sales_tax),
                Some(tourism_tax),
                Some(type_),
            ) = (
                self.admissions_tax.take(),
                self.attendance_tax.take(),
                self.entertainment_tax.take(),
                self.gross_receipts_tax.take(),
                self.hospitality_tax.take(),
                self.local_amusement_tax.take(),
                self.local_lease_tax.take(),
                self.luxury_tax.take(),
                self.mass_transit_parking_tax.take(),
                self.parking_tax.take(),
                self.resort_tax.take(),
                self.state.take(),
                self.state_sales_tax.take(),
                self.tourism_tax.take(),
                self.type_.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                admissions_tax,
                attendance_tax,
                entertainment_tax,
                gross_receipts_tax,
                hospitality_tax,
                local_amusement_tax,
                local_lease_tax,
                luxury_tax,
                mass_transit_parking_tax,
                parking_tax,
                resort_tax,
                state,
                state_sales_tax,
                tourism_tax,
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
                    "admissions_tax" => b.admissions_tax = FromValueOpt::from_value(v),
                    "attendance_tax" => b.attendance_tax = FromValueOpt::from_value(v),
                    "entertainment_tax" => b.entertainment_tax = FromValueOpt::from_value(v),
                    "gross_receipts_tax" => b.gross_receipts_tax = FromValueOpt::from_value(v),
                    "hospitality_tax" => b.hospitality_tax = FromValueOpt::from_value(v),
                    "local_amusement_tax" => b.local_amusement_tax = FromValueOpt::from_value(v),
                    "local_lease_tax" => b.local_lease_tax = FromValueOpt::from_value(v),
                    "luxury_tax" => b.luxury_tax = FromValueOpt::from_value(v),
                    "mass_transit_parking_tax" => {
                        b.mass_transit_parking_tax = FromValueOpt::from_value(v)
                    }
                    "parking_tax" => b.parking_tax = FromValueOpt::from_value(v),
                    "resort_tax" => b.resort_tax = FromValueOpt::from_value(v),
                    "state" => b.state = FromValueOpt::from_value(v),
                    "state_sales_tax" => b.state_sales_tax = FromValueOpt::from_value(v),
                    "tourism_tax" => b.tourism_tax = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Type of registration in the US.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    AdmissionsTax,
    AttendanceTax,
    EntertainmentTax,
    GrossReceiptsTax,
    HospitalityTax,
    LocalAmusementTax,
    LocalLeaseTax,
    LuxuryTax,
    MassTransitParkingTax,
    ParkingTax,
    ResortTax,
    StateCommunicationsTax,
    StateRetailDeliveryFee,
    StateSalesTax,
    TourismTax,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    pub fn as_str(&self) -> &str {
        use TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::*;
        match self {
            AdmissionsTax => "admissions_tax",
            AttendanceTax => "attendance_tax",
            EntertainmentTax => "entertainment_tax",
            GrossReceiptsTax => "gross_receipts_tax",
            HospitalityTax => "hospitality_tax",
            LocalAmusementTax => "local_amusement_tax",
            LocalLeaseTax => "local_lease_tax",
            LuxuryTax => "luxury_tax",
            MassTransitParkingTax => "mass_transit_parking_tax",
            ParkingTax => "parking_tax",
            ResortTax => "resort_tax",
            StateCommunicationsTax => "state_communications_tax",
            StateRetailDeliveryFee => "state_retail_delivery_fee",
            StateSalesTax => "state_sales_tax",
            TourismTax => "tourism_tax",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::*;
        match s {
            "admissions_tax" => Ok(AdmissionsTax),
            "attendance_tax" => Ok(AttendanceTax),
            "entertainment_tax" => Ok(EntertainmentTax),
            "gross_receipts_tax" => Ok(GrossReceiptsTax),
            "hospitality_tax" => Ok(HospitalityTax),
            "local_amusement_tax" => Ok(LocalAmusementTax),
            "local_lease_tax" => Ok(LocalLeaseTax),
            "luxury_tax" => Ok(LuxuryTax),
            "mass_transit_parking_tax" => Ok(MassTransitParkingTax),
            "parking_tax" => Ok(ParkingTax),
            "resort_tax" => Ok(ResortTax),
            "state_communications_tax" => Ok(StateCommunicationsTax),
            "state_retail_delivery_fee" => Ok(StateRetailDeliveryFee),
            "state_sales_tax" => Ok(StateSalesTax),
            "tourism_tax" => Ok(TourismTax),
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
