#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDiscounts {
    /// Corporate client discount code.
    pub corporate_client_code: Option<String>,
    /// Coupon code applied.
    pub coupon: Option<String>,
    /// Maximum free miles or kilometers included.
    pub maximum_free_miles_or_kilometers: Option<i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDiscounts
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(
            "PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDiscounts",
        )
        .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDiscountsBuilder {
    corporate_client_code: Option<Option<String>>,
    coupon: Option<Option<String>>,
    maximum_free_miles_or_kilometers: Option<Option<i64>>,
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
        for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDiscounts
    {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<
            PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDiscounts,
        >,
        builder:
            PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDiscountsBuilder,
    }

    impl Visitor
        for Place<PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDiscounts>
    {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDiscountsBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder
        for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDiscountsBuilder
    {
        type Out = PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDiscounts;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "corporate_client_code" => Deserialize::begin(&mut self.corporate_client_code),
                "coupon" => Deserialize::begin(&mut self.coupon),
                "maximum_free_miles_or_kilometers" => {
                    Deserialize::begin(&mut self.maximum_free_miles_or_kilometers)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                corporate_client_code: Some(None),
                coupon: Some(None),
                maximum_free_miles_or_kilometers: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(corporate_client_code), Some(coupon), Some(maximum_free_miles_or_kilometers)) = (
                self.corporate_client_code.take(),
                self.coupon.take(),
                self.maximum_free_miles_or_kilometers,
            ) else {
                return None;
            };
            Some(Self::Out { corporate_client_code, coupon, maximum_free_miles_or_kilometers })
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
        for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDiscounts
    {
        type Builder =
            PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDiscountsBuilder;
    }

    impl FromValueOpt
        for PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDiscounts
    {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentFlowsIndustryMetadataCarRentalDataResourceCarRentalSupplementaryDiscountsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "corporate_client_code" => {
                        b.corporate_client_code = FromValueOpt::from_value(v)
                    }
                    "coupon" => b.coupon = FromValueOpt::from_value(v),
                    "maximum_free_miles_or_kilometers" => {
                        b.maximum_free_miles_or_kilometers = FromValueOpt::from_value(v)
                    }
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
