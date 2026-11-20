#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHost {
    pub address: Option<stripe_shared::Address>,
    /// Host's country of domicile.
    pub country_of_domicile: Option<String>,
    /// Reference number for the host.
    pub host_reference: Option<String>,
    /// Type of host.
    pub host_type:
        Option<PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHostHostType>,
    /// Name of the lodging property or host.
    pub name: Option<String>,
    /// Total number of reservations for the host.
    pub number_of_reservations: Option<i64>,
    /// Property phone number.
    pub property_phone_number: Option<String>,
    /// Host's registration date.
    pub registered_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHost {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHost")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHostBuilder {
    address: Option<Option<stripe_shared::Address>>,
    country_of_domicile: Option<Option<String>>,
    host_reference: Option<Option<String>>,
    host_type: Option<
        Option<PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHostHostType>,
    >,
    name: Option<Option<String>>,
    number_of_reservations: Option<Option<i64>>,
    property_phone_number: Option<Option<String>>,
    registered_at: Option<Option<stripe_types::Timestamp>>,
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

    impl Deserialize for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHost {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out:
            &'a mut Option<PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHost>,
        builder: PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHostBuilder,
    }

    impl Visitor for Place<PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHost> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHostBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHostBuilder {
        type Out = PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHost;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address" => Deserialize::begin(&mut self.address),
                "country_of_domicile" => Deserialize::begin(&mut self.country_of_domicile),
                "host_reference" => Deserialize::begin(&mut self.host_reference),
                "host_type" => Deserialize::begin(&mut self.host_type),
                "name" => Deserialize::begin(&mut self.name),
                "number_of_reservations" => Deserialize::begin(&mut self.number_of_reservations),
                "property_phone_number" => Deserialize::begin(&mut self.property_phone_number),
                "registered_at" => Deserialize::begin(&mut self.registered_at),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                address: Some(None),
                country_of_domicile: Some(None),
                host_reference: Some(None),
                host_type: Some(None),
                name: Some(None),
                number_of_reservations: Some(None),
                property_phone_number: Some(None),
                registered_at: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(address),
                Some(country_of_domicile),
                Some(host_reference),
                Some(host_type),
                Some(name),
                Some(number_of_reservations),
                Some(property_phone_number),
                Some(registered_at),
            ) = (
                self.address.take(),
                self.country_of_domicile.take(),
                self.host_reference.take(),
                self.host_type.take(),
                self.name.take(),
                self.number_of_reservations,
                self.property_phone_number.take(),
                self.registered_at,
            )
            else {
                return None;
            };
            Some(Self::Out {
                address,
                country_of_domicile,
                host_reference,
                host_type,
                name,
                number_of_reservations,
                property_phone_number,
                registered_at,
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

    impl ObjectDeser for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHost {
        type Builder =
            PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHostBuilder;
    }

    impl FromValueOpt for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHost {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHostBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "address" => b.address = FromValueOpt::from_value(v),
                    "country_of_domicile" => b.country_of_domicile = FromValueOpt::from_value(v),
                    "host_reference" => b.host_reference = FromValueOpt::from_value(v),
                    "host_type" => b.host_type = FromValueOpt::from_value(v),
                    "name" => b.name = FromValueOpt::from_value(v),
                    "number_of_reservations" => {
                        b.number_of_reservations = FromValueOpt::from_value(v)
                    }
                    "property_phone_number" => {
                        b.property_phone_number = FromValueOpt::from_value(v)
                    }
                    "registered_at" => b.registered_at = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Type of host.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHostHostType {
    Hostel,
    Hotel,
    Owner,
    RentalAgency,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHostHostType {
    pub fn as_str(&self) -> &str {
        use PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHostHostType::*;
        match self {
            Hostel => "hostel",
            Hotel => "hotel",
            Owner => "owner",
            RentalAgency => "rental_agency",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHostHostType
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHostHostType::*;
        match s {
            "hostel" => Ok(Hostel),
            "hotel" => Ok(Hotel),
            "owner" => Ok(Owner),
            "rental_agency" => Ok(RentalAgency),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHostHostType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display
    for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHostHostType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug
    for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHostHostType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHostHostType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHostHostType
        ))
        .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize
    for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHostHostType
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHostHostType
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<
        PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHostHostType,
    >
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHostHostType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHostHostType
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryHostHostType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
