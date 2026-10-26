/// Describes changes to the subscription's status.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionsResourceStatusDetailsPublic {
    pub paused: stripe_shared::SubscriptionsResourcePausedStatusDetailsPublic,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionsResourceStatusDetailsPublic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionsResourceStatusDetailsPublic").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionsResourceStatusDetailsPublicBuilder {
    paused: Option<stripe_shared::SubscriptionsResourcePausedStatusDetailsPublic>,
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

    impl Deserialize for SubscriptionsResourceStatusDetailsPublic {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionsResourceStatusDetailsPublic>,
        builder: SubscriptionsResourceStatusDetailsPublicBuilder,
    }

    impl Visitor for Place<SubscriptionsResourceStatusDetailsPublic> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionsResourceStatusDetailsPublicBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionsResourceStatusDetailsPublicBuilder {
        type Out = SubscriptionsResourceStatusDetailsPublic;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "paused" => Deserialize::begin(&mut self.paused),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { paused: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(paused),) = (self.paused.take(),) else {
                return None;
            };
            Some(Self::Out { paused })
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

    impl ObjectDeser for SubscriptionsResourceStatusDetailsPublic {
        type Builder = SubscriptionsResourceStatusDetailsPublicBuilder;
    }

    impl FromValueOpt for SubscriptionsResourceStatusDetailsPublic {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionsResourceStatusDetailsPublicBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "paused" => b.paused = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
