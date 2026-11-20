#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryLocation {
    pub address: Option<stripe_shared::Address>,
    /// Name of the location.
    pub location_name: Option<String>,
    /// Time associated with the location.
    pub time: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryLocation
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(
            "PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryLocation",
        )
        .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryLocationBuilder {
    address: Option<Option<stripe_shared::Address>>,
    location_name: Option<Option<String>>,
    time: Option<Option<stripe_types::Timestamp>>,
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
        for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryLocation
    {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<
            PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryLocation,
        >,
        builder:
            PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryLocationBuilder,
    }

    impl Visitor
        for Place<PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryLocation>
    {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryLocationBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder
        for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryLocationBuilder
    {
        type Out = PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryLocation;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address" => Deserialize::begin(&mut self.address),
                "location_name" => Deserialize::begin(&mut self.location_name),
                "time" => Deserialize::begin(&mut self.time),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { address: Some(None), location_name: Some(None), time: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(address), Some(location_name), Some(time)) =
                (self.address.take(), self.location_name.take(), self.time)
            else {
                return None;
            };
            Some(Self::Out { address, location_name, time })
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
        for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryLocation
    {
        type Builder =
            PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryLocationBuilder;
    }

    impl FromValueOpt
        for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryLocation
    {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryLocationBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "address" => b.address = FromValueOpt::from_value(v),
                    "location_name" => b.location_name = FromValueOpt::from_value(v),
                    "time" => b.time = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
