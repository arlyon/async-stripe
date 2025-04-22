#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingCreditGrantsResourceApplicabilityConfig {
    pub scope: stripe_shared::BillingCreditGrantsResourceScope,
}
#[doc(hidden)]
pub struct BillingCreditGrantsResourceApplicabilityConfigBuilder {
    scope: Option<stripe_shared::BillingCreditGrantsResourceScope>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for BillingCreditGrantsResourceApplicabilityConfig {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingCreditGrantsResourceApplicabilityConfig>,
        builder: BillingCreditGrantsResourceApplicabilityConfigBuilder,
    }

    impl Visitor for Place<BillingCreditGrantsResourceApplicabilityConfig> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BillingCreditGrantsResourceApplicabilityConfigBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BillingCreditGrantsResourceApplicabilityConfigBuilder {
        type Out = BillingCreditGrantsResourceApplicabilityConfig;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "scope" => Deserialize::begin(&mut self.scope),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { scope: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(scope),) = (self.scope.take(),) else {
                return None;
            };
            Some(Self::Out { scope })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for BillingCreditGrantsResourceApplicabilityConfig {
        type Builder = BillingCreditGrantsResourceApplicabilityConfigBuilder;
    }

    impl FromValueOpt for BillingCreditGrantsResourceApplicabilityConfig {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BillingCreditGrantsResourceApplicabilityConfigBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "scope" => b.scope = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
