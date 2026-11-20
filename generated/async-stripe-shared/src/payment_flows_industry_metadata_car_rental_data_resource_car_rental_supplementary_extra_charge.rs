#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryExtraCharge {
    /// Amount of the extra charge.
    pub amount: Option<i64>,
    /// Type of extra charge.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: Option<
        PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryExtraChargeType,
    >,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryExtraCharge
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(
            "PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryExtraCharge",
        )
        .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryExtraChargeBuilder
{
    amount: Option<Option<i64>>,
    type_: Option<
        Option<
            PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryExtraChargeType,
        >,
    >,
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
        for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryExtraCharge
    {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
    out: &'a mut Option<PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryExtraCharge>,
    builder: PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryExtraChargeBuilder,
}

    impl Visitor
        for Place<
            PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryExtraCharge,
        >
    {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryExtraChargeBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryExtraChargeBuilder {
    type Out = PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryExtraCharge;
    fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
        Ok(match k {
            "amount" => Deserialize::begin(&mut self.amount),
            "type" => Deserialize::begin(&mut self.type_),
            _ => <dyn Visitor>::ignore(),
        })
    }

    fn deser_default() -> Self {
        Self { amount: Some(None),
type_: Some(None),
 }
    }

    fn take_out(&mut self) -> Option<Self::Out> {
        let (Some(amount),
Some(type_),
) = (self.amount,
self.type_.take(),
) else {
            return None;
        };
        Some(Self::Out { amount,type_ })
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
        for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryExtraCharge
    {
        type Builder = PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryExtraChargeBuilder;
    }

    impl FromValueOpt
        for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryExtraCharge
    {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryExtraChargeBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Type of extra charge.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryExtraChargeType {
    ExtraMileage,
    Gas,
    Gps,
    LateCharge,
    OneWayDropOff,
    Other,
    Parking,
    Phone,
    RegularMileage,
    Towing,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryExtraChargeType {
    pub fn as_str(&self) -> &str {
        use PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryExtraChargeType::*;
        match self {
            ExtraMileage => "extra_mileage",
            Gas => "gas",
            Gps => "gps",
            LateCharge => "late_charge",
            OneWayDropOff => "one_way_drop_off",
            Other => "other",
            Parking => "parking",
            Phone => "phone",
            RegularMileage => "regular_mileage",
            Towing => "towing",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryExtraChargeType
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryExtraChargeType::*;
        match s {
            "extra_mileage" => Ok(ExtraMileage),
            "gas" => Ok(Gas),
            "gps" => Ok(Gps),
            "late_charge" => Ok(LateCharge),
            "one_way_drop_off" => Ok(OneWayDropOff),
            "other" => Ok(Other),
            "parking" => Ok(Parking),
            "phone" => Ok(Phone),
            "regular_mileage" => Ok(RegularMileage),
            "towing" => Ok(Towing),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryExtraChargeType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display
    for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryExtraChargeType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug
    for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryExtraChargeType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryExtraChargeType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryExtraChargeType
        ))
        .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize
    for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryExtraChargeType
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryExtraChargeType
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<
        PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryExtraChargeType,
    >
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryExtraChargeType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryExtraChargeType
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryExtraChargeType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
