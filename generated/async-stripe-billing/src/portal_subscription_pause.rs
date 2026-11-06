#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalSubscriptionPause {
    pub duration: stripe_billing::PortalSubscriptionPauseDuration,
    /// Whether the feature is enabled.
    /// Customers can only pause subscriptions in the [redesigned customer portal](/customer-management).
    pub enabled: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalSubscriptionPause {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PortalSubscriptionPause").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PortalSubscriptionPauseBuilder {
    duration: Option<stripe_billing::PortalSubscriptionPauseDuration>,
    enabled: Option<bool>,
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

    impl Deserialize for PortalSubscriptionPause {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalSubscriptionPause>,
        builder: PortalSubscriptionPauseBuilder,
    }

    impl Visitor for Place<PortalSubscriptionPause> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PortalSubscriptionPauseBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PortalSubscriptionPauseBuilder {
        type Out = PortalSubscriptionPause;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "duration" => Deserialize::begin(&mut self.duration),
                "enabled" => Deserialize::begin(&mut self.enabled),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { duration: None, enabled: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(duration), Some(enabled)) = (self.duration.take(), self.enabled) else {
                return None;
            };
            Some(Self::Out { duration, enabled })
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

    impl ObjectDeser for PortalSubscriptionPause {
        type Builder = PortalSubscriptionPauseBuilder;
    }

    impl FromValueOpt for PortalSubscriptionPause {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PortalSubscriptionPauseBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "duration" => b.duration = FromValueOpt::from_value(v),
                    "enabled" => b.enabled = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
