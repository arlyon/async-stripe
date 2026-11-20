#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicle {
    /// Make of the vehicle.
    pub make: Option<String>,
    /// Model of the vehicle.
    pub model: Option<String>,
    /// Odometer reading.
    pub odometer: Option<i64>,
    /// Type of the vehicle.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_:
        Option<PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleType>,
    /// Class of the vehicle.
    pub vehicle_class: Option<
        PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleVehicleClass,
    >,
    /// Vehicle identification number.
    pub vehicle_identification_number: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicle
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(
            "PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicle",
        )
        .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleBuilder {
    make: Option<Option<String>>,
model: Option<Option<String>>,
odometer: Option<Option<i64>>,
type_: Option<Option<PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleType>>,
vehicle_class: Option<Option<PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleVehicleClass>>,
vehicle_identification_number: Option<Option<String>>,

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

    impl Deserialize
        for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicle
    {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<
            PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicle,
        >,
        builder:
            PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleBuilder,
    }

    impl Visitor
        for Place<PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicle>
    {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder
        for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleBuilder
    {
        type Out = PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicle;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "make" => Deserialize::begin(&mut self.make),
                "model" => Deserialize::begin(&mut self.model),
                "odometer" => Deserialize::begin(&mut self.odometer),
                "type" => Deserialize::begin(&mut self.type_),
                "vehicle_class" => Deserialize::begin(&mut self.vehicle_class),
                "vehicle_identification_number" => {
                    Deserialize::begin(&mut self.vehicle_identification_number)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                make: Some(None),
                model: Some(None),
                odometer: Some(None),
                type_: Some(None),
                vehicle_class: Some(None),
                vehicle_identification_number: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(make),
                Some(model),
                Some(odometer),
                Some(type_),
                Some(vehicle_class),
                Some(vehicle_identification_number),
            ) = (
                self.make.take(),
                self.model.take(),
                self.odometer,
                self.type_.take(),
                self.vehicle_class.take(),
                self.vehicle_identification_number.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                make,
                model,
                odometer,
                type_,
                vehicle_class,
                vehicle_identification_number,
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

    impl ObjectDeser
        for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicle
    {
        type Builder =
            PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleBuilder;
    }

    impl FromValueOpt
        for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicle
    {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "make" => b.make = FromValueOpt::from_value(v),
                    "model" => b.model = FromValueOpt::from_value(v),
                    "odometer" => b.odometer = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    "vehicle_class" => b.vehicle_class = FromValueOpt::from_value(v),
                    "vehicle_identification_number" => {
                        b.vehicle_identification_number = FromValueOpt::from_value(v)
                    }
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Type of the vehicle.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleType {
    CargoVan,
    Compact,
    Economy,
    Exotic,
    ExoticSuv,
    FifteenPassengerVan,
    FourWheelDrive,
    FullSize,
    Intermediate,
    LargeSuv,
    LargeTruck,
    Luxury,
    MediumSuv,
    Midsize,
    Mini,
    Minivan,
    Miscellaneous,
    Moped,
    MovingVan,
    Premium,
    Regular,
    SmallMediumTruck,
    SmallSuv,
    Special,
    Standard,
    Stretch,
    Subcompact,
    Taxi,
    TwelveFootTruck,
    TwelvePassengerVan,
    TwentyFootTruck,
    TwentyFourFootTruck,
    TwentySixFootTruck,
    Unique,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleType {
    pub fn as_str(&self) -> &str {
        use PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleType::*;
        match self {
            CargoVan => "cargo_van",
            Compact => "compact",
            Economy => "economy",
            Exotic => "exotic",
            ExoticSuv => "exotic_suv",
            FifteenPassengerVan => "fifteen_passenger_van",
            FourWheelDrive => "four_wheel_drive",
            FullSize => "full_size",
            Intermediate => "intermediate",
            LargeSuv => "large_suv",
            LargeTruck => "large_truck",
            Luxury => "luxury",
            MediumSuv => "medium_suv",
            Midsize => "midsize",
            Mini => "mini",
            Minivan => "minivan",
            Miscellaneous => "miscellaneous",
            Moped => "moped",
            MovingVan => "moving_van",
            Premium => "premium",
            Regular => "regular",
            SmallMediumTruck => "small_medium_truck",
            SmallSuv => "small_suv",
            Special => "special",
            Standard => "standard",
            Stretch => "stretch",
            Subcompact => "subcompact",
            Taxi => "taxi",
            TwelveFootTruck => "twelve_foot_truck",
            TwelvePassengerVan => "twelve_passenger_van",
            TwentyFootTruck => "twenty_foot_truck",
            TwentyFourFootTruck => "twenty_four_foot_truck",
            TwentySixFootTruck => "twenty_six_foot_truck",
            Unique => "unique",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleType
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleType::*;
        match s {
            "cargo_van" => Ok(CargoVan),
            "compact" => Ok(Compact),
            "economy" => Ok(Economy),
            "exotic" => Ok(Exotic),
            "exotic_suv" => Ok(ExoticSuv),
            "fifteen_passenger_van" => Ok(FifteenPassengerVan),
            "four_wheel_drive" => Ok(FourWheelDrive),
            "full_size" => Ok(FullSize),
            "intermediate" => Ok(Intermediate),
            "large_suv" => Ok(LargeSuv),
            "large_truck" => Ok(LargeTruck),
            "luxury" => Ok(Luxury),
            "medium_suv" => Ok(MediumSuv),
            "midsize" => Ok(Midsize),
            "mini" => Ok(Mini),
            "minivan" => Ok(Minivan),
            "miscellaneous" => Ok(Miscellaneous),
            "moped" => Ok(Moped),
            "moving_van" => Ok(MovingVan),
            "premium" => Ok(Premium),
            "regular" => Ok(Regular),
            "small_medium_truck" => Ok(SmallMediumTruck),
            "small_suv" => Ok(SmallSuv),
            "special" => Ok(Special),
            "standard" => Ok(Standard),
            "stretch" => Ok(Stretch),
            "subcompact" => Ok(Subcompact),
            "taxi" => Ok(Taxi),
            "twelve_foot_truck" => Ok(TwelveFootTruck),
            "twelve_passenger_van" => Ok(TwelvePassengerVan),
            "twenty_foot_truck" => Ok(TwentyFootTruck),
            "twenty_four_foot_truck" => Ok(TwentyFourFootTruck),
            "twenty_six_foot_truck" => Ok(TwentySixFootTruck),
            "unique" => Ok(Unique),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display
    for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug
    for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleType
        ))
        .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize
    for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleType
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleType
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<
        PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleType,
    >
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleType
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Class of the vehicle.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleVehicleClass
{
    Business,
    Economy,
    FirstClass,
    PremiumEconomy,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleVehicleClass {
    pub fn as_str(&self) -> &str {
        use PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleVehicleClass::*;
        match self {
            Business => "business",
            Economy => "economy",
            FirstClass => "first_class",
            PremiumEconomy => "premium_economy",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleVehicleClass
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleVehicleClass::*;
        match s {
            "business" => Ok(Business),
            "economy" => Ok(Economy),
            "first_class" => Ok(FirstClass),
            "premium_economy" => Ok(PremiumEconomy),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleVehicleClass"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display
    for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleVehicleClass
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug
    for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleVehicleClass
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleVehicleClass
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleVehicleClass)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize
    for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleVehicleClass
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleVehicleClass
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<
        PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleVehicleClass,
    >
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleVehicleClass::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleVehicleClass
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryVehicleVehicleClass
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
