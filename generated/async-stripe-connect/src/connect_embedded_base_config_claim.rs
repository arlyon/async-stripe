#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConnectEmbeddedBaseConfigClaim {
    /// Whether the embedded component is enabled.
    pub enabled: bool,
    pub features: stripe_connect::ConnectEmbeddedBaseFeatures,
}
#[doc(hidden)]
pub struct ConnectEmbeddedBaseConfigClaimBuilder {
    enabled: Option<bool>,
    features: Option<stripe_connect::ConnectEmbeddedBaseFeatures>,
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

    impl Deserialize for ConnectEmbeddedBaseConfigClaim {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ConnectEmbeddedBaseConfigClaim>,
        builder: ConnectEmbeddedBaseConfigClaimBuilder,
    }

    impl Visitor for Place<ConnectEmbeddedBaseConfigClaim> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ConnectEmbeddedBaseConfigClaimBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ConnectEmbeddedBaseConfigClaimBuilder {
        type Out = ConnectEmbeddedBaseConfigClaim;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "enabled" => Deserialize::begin(&mut self.enabled),
                "features" => Deserialize::begin(&mut self.features),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { enabled: Deserialize::default(), features: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(enabled), Some(features)) = (self.enabled, self.features) else {
                return None;
            };
            Some(Self::Out { enabled, features })
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

    impl ObjectDeser for ConnectEmbeddedBaseConfigClaim {
        type Builder = ConnectEmbeddedBaseConfigClaimBuilder;
    }

    impl FromValueOpt for ConnectEmbeddedBaseConfigClaim {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ConnectEmbeddedBaseConfigClaimBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "enabled" => b.enabled = FromValueOpt::from_value(v),
                    "features" => b.features = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
