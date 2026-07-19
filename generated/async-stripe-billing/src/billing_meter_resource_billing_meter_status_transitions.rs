#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingMeterResourceBillingMeterStatusTransitions {
    /// The time the meter was deactivated, if any. Measured in seconds since Unix epoch.
    pub deactivated_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingMeterResourceBillingMeterStatusTransitions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingMeterResourceBillingMeterStatusTransitions").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BillingMeterResourceBillingMeterStatusTransitionsBuilder {
    deactivated_at: Option<Option<stripe_types::Timestamp>>,
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

    impl Deserialize for BillingMeterResourceBillingMeterStatusTransitions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingMeterResourceBillingMeterStatusTransitions>,
        builder: BillingMeterResourceBillingMeterStatusTransitionsBuilder,
    }

    impl Visitor for Place<BillingMeterResourceBillingMeterStatusTransitions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BillingMeterResourceBillingMeterStatusTransitionsBuilder {
                    deactivated_at: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "deactivated_at" => Deserialize::begin(&mut self.builder.deactivated_at),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(deactivated_at),) = (self.builder.deactivated_at,) else {
                return Ok(());
            };
            *self.out = Some(BillingMeterResourceBillingMeterStatusTransitions { deactivated_at });
            Ok(())
        }
    }
};
