#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingClocksResourceStatusDetailsAdvancingStatusDetails {
    /// The `frozen_time` that the Test Clock is advancing towards.
    pub target_frozen_time: stripe_types::Timestamp,
}
#[doc(hidden)]
pub struct BillingClocksResourceStatusDetailsAdvancingStatusDetailsBuilder {
    target_frozen_time: Option<stripe_types::Timestamp>,
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

    impl Deserialize for BillingClocksResourceStatusDetailsAdvancingStatusDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingClocksResourceStatusDetailsAdvancingStatusDetails>,
        builder: BillingClocksResourceStatusDetailsAdvancingStatusDetailsBuilder,
    }

    impl Visitor for Place<BillingClocksResourceStatusDetailsAdvancingStatusDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    BillingClocksResourceStatusDetailsAdvancingStatusDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BillingClocksResourceStatusDetailsAdvancingStatusDetailsBuilder {
        type Out = BillingClocksResourceStatusDetailsAdvancingStatusDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "target_frozen_time" => Deserialize::begin(&mut self.target_frozen_time),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { target_frozen_time: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(target_frozen_time),) = (self.target_frozen_time,) else {
                return None;
            };
            Some(Self::Out { target_frozen_time })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for BillingClocksResourceStatusDetailsAdvancingStatusDetails {
        type Builder = BillingClocksResourceStatusDetailsAdvancingStatusDetailsBuilder;
    }

    impl FromValueOpt for BillingClocksResourceStatusDetailsAdvancingStatusDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                BillingClocksResourceStatusDetailsAdvancingStatusDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "target_frozen_time" => b.target_frozen_time = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
