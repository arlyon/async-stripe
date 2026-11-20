#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodation {
    /// Type of accommodation.
pub accommodation_type: Option<PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodationAccommodationType>,
    /// Bed type.
pub bed_type: Option<String>,
    /// Daily accommodation rate in cents.
pub daily_rate_amount: Option<i64>,
    /// Number of nights.
pub nights: Option<i64>,
    /// Number of rooms, cabanas, apartments, and so on.
pub number_of_rooms: Option<i64>,
    /// Rate type.
pub rate_type: Option<String>,
    /// Whether smoking is allowed.
pub smoking_indicator: Option<bool>,

}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodation
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(
            "PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodation",
        )
        .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodationBuilder {
    accommodation_type: Option<Option<PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodationAccommodationType>>,
bed_type: Option<Option<String>>,
daily_rate_amount: Option<Option<i64>>,
nights: Option<Option<i64>>,
number_of_rooms: Option<Option<i64>>,
rate_type: Option<Option<String>>,
smoking_indicator: Option<Option<bool>>,

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
        for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodation
    {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<
            PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodation,
        >,
        builder:
            PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodationBuilder,
    }

    impl Visitor
        for Place<PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodation>
    {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodationBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder
        for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodationBuilder
    {
        type Out = PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodation;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "accommodation_type" => Deserialize::begin(&mut self.accommodation_type),
                "bed_type" => Deserialize::begin(&mut self.bed_type),
                "daily_rate_amount" => Deserialize::begin(&mut self.daily_rate_amount),
                "nights" => Deserialize::begin(&mut self.nights),
                "number_of_rooms" => Deserialize::begin(&mut self.number_of_rooms),
                "rate_type" => Deserialize::begin(&mut self.rate_type),
                "smoking_indicator" => Deserialize::begin(&mut self.smoking_indicator),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                accommodation_type: Some(None),
                bed_type: Some(None),
                daily_rate_amount: Some(None),
                nights: Some(None),
                number_of_rooms: Some(None),
                rate_type: Some(None),
                smoking_indicator: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(accommodation_type),
                Some(bed_type),
                Some(daily_rate_amount),
                Some(nights),
                Some(number_of_rooms),
                Some(rate_type),
                Some(smoking_indicator),
            ) = (
                self.accommodation_type.take(),
                self.bed_type.take(),
                self.daily_rate_amount,
                self.nights,
                self.number_of_rooms,
                self.rate_type.take(),
                self.smoking_indicator,
            )
            else {
                return None;
            };
            Some(Self::Out {
                accommodation_type,
                bed_type,
                daily_rate_amount,
                nights,
                number_of_rooms,
                rate_type,
                smoking_indicator,
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
        for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodation
    {
        type Builder =
            PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodationBuilder;
    }

    impl FromValueOpt
        for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodation
    {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodationBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "accommodation_type" => b.accommodation_type = FromValueOpt::from_value(v),
                    "bed_type" => b.bed_type = FromValueOpt::from_value(v),
                    "daily_rate_amount" => b.daily_rate_amount = FromValueOpt::from_value(v),
                    "nights" => b.nights = FromValueOpt::from_value(v),
                    "number_of_rooms" => b.number_of_rooms = FromValueOpt::from_value(v),
                    "rate_type" => b.rate_type = FromValueOpt::from_value(v),
                    "smoking_indicator" => b.smoking_indicator = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Type of accommodation.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodationAccommodationType
{
    Apartment,
    Cabana,
    House,
    Penthouse,
    Room,
    Standard,
    Suite,
    Villa,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodationAccommodationType {
    pub fn as_str(&self) -> &str {
        use PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodationAccommodationType::*;
        match self {
Apartment => "apartment",
Cabana => "cabana",
House => "house",
Penthouse => "penthouse",
Room => "room",
Standard => "standard",
Suite => "suite",
Villa => "villa",
Unknown(v) => v,

        }
    }
}

impl std::str::FromStr for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodationAccommodationType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodationAccommodationType::*;
        match s {
    "apartment" => Ok(Apartment),
"cabana" => Ok(Cabana),
"house" => Ok(House),
"penthouse" => Ok(Penthouse),
"room" => Ok(Room),
"standard" => Ok(Standard),
"suite" => Ok(Suite),
"villa" => Ok(Villa),
v => { tracing::warn!("Unknown value '{}' for enum '{}'", v, "PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodationAccommodationType"); Ok(Unknown(v.to_owned())) }

        }
    }
}
impl std::fmt::Display for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodationAccommodationType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodationAccommodationType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodationAccommodationType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodationAccommodationType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodationAccommodationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodationAccommodationType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodationAccommodationType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodationAccommodationType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodationAccommodationType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAccommodationAccommodationType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
