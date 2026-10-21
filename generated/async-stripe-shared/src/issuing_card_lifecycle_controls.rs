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
                builder: IssuingCardLifecycleControlsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingCardLifecycleControlsBuilder {
        type Out = IssuingCardLifecycleControls;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "cancel_after" => Deserialize::begin(&mut self.cancel_after),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { cancel_after: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self { cancel_after: Some(cancel_after) } => Some(Self::Out { cancel_after }),
                builder => {
                    *self = builder;
                    None
                }
            }
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

    impl ObjectDeser for IssuingCardLifecycleControls {
        type Builder = IssuingCardLifecycleControlsBuilder;
    }

    impl FromValueOpt for IssuingCardLifecycleControls {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
