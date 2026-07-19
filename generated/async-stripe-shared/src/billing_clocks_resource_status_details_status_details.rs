#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingClocksResourceStatusDetailsStatusDetails {
    pub advancing: Option<stripe_shared::BillingClocksResourceStatusDetailsAdvancingStatusDetails>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingClocksResourceStatusDetailsStatusDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingClocksResourceStatusDetailsStatusDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BillingClocksResourceStatusDetailsStatusDetailsBuilder {
    advancing:
        Option<Option<stripe_shared::BillingClocksResourceStatusDetailsAdvancingStatusDetails>>,
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

    impl Deserialize for BillingClocksResourceStatusDetailsStatusDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingClocksResourceStatusDetailsStatusDetails>,
        builder: BillingClocksResourceStatusDetailsStatusDetailsBuilder,
    }

    impl Visitor for Place<BillingClocksResourceStatusDetailsStatusDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BillingClocksResourceStatusDetailsStatusDetailsBuilder {
                    advancing: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "advancing" => Deserialize::begin(&mut self.builder.advancing),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(advancing),) = (self.builder.advancing,) else {
                return Ok(());
            };
            *self.out = Some(BillingClocksResourceStatusDetailsStatusDetails { advancing });
            Ok(())
        }
    }
};
