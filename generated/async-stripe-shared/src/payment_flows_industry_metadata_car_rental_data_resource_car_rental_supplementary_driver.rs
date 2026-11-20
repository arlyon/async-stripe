#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDriver {
pub date_of_birth: Option<stripe_shared::PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDateOfBirth>,
    /// Driver's identification number.
pub driver_identification_number: Option<String>,
    /// Driver's tax number.
pub driver_tax_number: Option<String>,
    /// Full name of the driver.
pub name: Option<String>,

}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDriver
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(
            "PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDriver",
        )
        .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDriverBuilder {
    date_of_birth: Option<Option<stripe_shared::PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDateOfBirth>>,
driver_identification_number: Option<Option<String>>,
driver_tax_number: Option<Option<String>>,
name: Option<Option<String>>,

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

    impl Deserialize for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDriver {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<
            PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDriver,
        >,
        builder:
            PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDriverBuilder,
    }

    impl Visitor
        for Place<PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDriver>
    {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDriverBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder
        for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDriverBuilder
    {
        type Out = PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDriver;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "date_of_birth" => Deserialize::begin(&mut self.date_of_birth),
                "driver_identification_number" => {
                    Deserialize::begin(&mut self.driver_identification_number)
                }
                "driver_tax_number" => Deserialize::begin(&mut self.driver_tax_number),
                "name" => Deserialize::begin(&mut self.name),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                date_of_birth: Some(None),
                driver_identification_number: Some(None),
                driver_tax_number: Some(None),
                name: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(date_of_birth),
                Some(driver_identification_number),
                Some(driver_tax_number),
                Some(name),
            ) = (
                self.date_of_birth,
                self.driver_identification_number.take(),
                self.driver_tax_number.take(),
                self.name.take(),
            )
            else {
                return None;
            };
            Some(Self::Out { date_of_birth, driver_identification_number, driver_tax_number, name })
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

    impl ObjectDeser for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDriver {
        type Builder =
            PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDriverBuilder;
    }

    impl FromValueOpt
        for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDriver
    {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDriverBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "date_of_birth" => b.date_of_birth = FromValueOpt::from_value(v),
                    "driver_identification_number" => {
                        b.driver_identification_number = FromValueOpt::from_value(v)
                    }
                    "driver_tax_number" => b.driver_tax_number = FromValueOpt::from_value(v),
                    "name" => b.name = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
