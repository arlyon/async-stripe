#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ThreeDSecureUsage {
    /// Whether 3D Secure is supported on this card.
    pub supported: bool,
}
#[doc(hidden)]
pub struct ThreeDSecureUsageBuilder {
    supported: Option<bool>,
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

    impl Deserialize for ThreeDSecureUsage {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ThreeDSecureUsage>,
        builder: ThreeDSecureUsageBuilder,
    }

    impl Visitor for Place<ThreeDSecureUsage> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ThreeDSecureUsageBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ThreeDSecureUsageBuilder {
        type Out = ThreeDSecureUsage;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "supported" => Deserialize::begin(&mut self.supported),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { supported: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(supported),) = (self.supported,) else {
                return None;
            };
            Some(Self::Out { supported })
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

    impl ObjectDeser for ThreeDSecureUsage {
        type Builder = ThreeDSecureUsageBuilder;
    }

    impl FromValueOpt for ThreeDSecureUsage {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ThreeDSecureUsageBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "supported" => b.supported = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
