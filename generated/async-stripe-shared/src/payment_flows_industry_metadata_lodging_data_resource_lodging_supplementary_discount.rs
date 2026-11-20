#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryDiscount {
    /// Corporate client discount code.
    pub corporate_client_code: Option<String>,
    /// Coupon code.
    pub coupon: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryDiscount
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(
            "PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryDiscount",
        )
        .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryDiscountBuilder {
    corporate_client_code: Option<Option<String>>,
    coupon: Option<Option<String>>,
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

    impl Deserialize for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryDiscount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<
            PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryDiscount,
        >,
        builder: PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryDiscountBuilder,
    }

    impl Visitor
        for Place<PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryDiscount>
    {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryDiscountBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder
        for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryDiscountBuilder
    {
        type Out = PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryDiscount;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "corporate_client_code" => Deserialize::begin(&mut self.corporate_client_code),
                "coupon" => Deserialize::begin(&mut self.coupon),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { corporate_client_code: Some(None), coupon: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(corporate_client_code), Some(coupon)) =
                (self.corporate_client_code.take(), self.coupon.take())
            else {
                return None;
            };
            Some(Self::Out { corporate_client_code, coupon })
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

    impl ObjectDeser for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryDiscount {
        type Builder =
            PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryDiscountBuilder;
    }

    impl FromValueOpt for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryDiscount {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryDiscountBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "corporate_client_code" => {
                        b.corporate_client_code = FromValueOpt::from_value(v)
                    }
                    "coupon" => b.coupon = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
