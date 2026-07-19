#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingClocksResourceStatusDetailsAdvancingStatusDetails {
    /// The `frozen_time` that the Test Clock is advancing towards.
    pub target_frozen_time: stripe_types::Timestamp,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingClocksResourceStatusDetailsAdvancingStatusDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingClocksResourceStatusDetailsAdvancingStatusDetails")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BillingClocksResourceStatusDetailsAdvancingStatusDetailsBuilder {
    target_frozen_time: Option<stripe_types::Timestamp>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: BillingClocksResourceStatusDetailsAdvancingStatusDetailsBuilder {
                    target_frozen_time: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "target_frozen_time" => Deserialize::begin(&mut self.builder.target_frozen_time),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(target_frozen_time),) = (self.builder.target_frozen_time,) else {
                return Ok(());
            };
            *self.out = Some(BillingClocksResourceStatusDetailsAdvancingStatusDetails {
                target_frozen_time,
            });
            Ok(())
        }
    }
};
