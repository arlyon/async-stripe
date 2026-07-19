#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingCardLifecycleControls {
    pub cancel_after: stripe_shared::IssuingCardLifecycleConditions,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingCardLifecycleControls {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingCardLifecycleControls").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingCardLifecycleControlsBuilder {
    cancel_after: Option<stripe_shared::IssuingCardLifecycleConditions>,
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

    impl Deserialize for IssuingCardLifecycleControls {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingCardLifecycleControls>,
        builder: IssuingCardLifecycleControlsBuilder,
    }

    impl Visitor for Place<IssuingCardLifecycleControls> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingCardLifecycleControlsBuilder {
                    cancel_after: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "cancel_after" => Deserialize::begin(&mut self.builder.cancel_after),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(cancel_after),) = (self.builder.cancel_after,) else {
                return Ok(());
            };
            *self.out = Some(IssuingCardLifecycleControls { cancel_after });
            Ok(())
        }
    }
};
