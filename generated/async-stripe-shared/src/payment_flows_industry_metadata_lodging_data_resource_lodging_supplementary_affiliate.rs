#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAffiliate {
    /// Affiliate partner code.
    pub code: Option<String>,
    /// Affiliate partner name.
    pub name: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAffiliate
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(
            "PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAffiliate",
        )
        .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAffiliateBuilder {
    code: Option<Option<String>>,
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

    impl Deserialize for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAffiliate {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<
            PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAffiliate,
        >,
        builder:
            PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAffiliateBuilder,
    }

    impl Visitor
        for Place<PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAffiliate>
    {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAffiliateBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder
        for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAffiliateBuilder
    {
        type Out = PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAffiliate;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "code" => Deserialize::begin(&mut self.code),
                "name" => Deserialize::begin(&mut self.name),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { code: Some(None), name: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(code), Some(name)) = (self.code.take(), self.name.take()) else {
                return None;
            };
            Some(Self::Out { code, name })
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

    impl ObjectDeser for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAffiliate {
        type Builder =
            PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAffiliateBuilder;
    }

    impl FromValueOpt for PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAffiliate {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentFlowsIndustryMetadataLodgingDataResourceLodgingSupplementaryAffiliateBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "code" => b.code = FromValueOpt::from_value(v),
                    "name" => b.name = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
