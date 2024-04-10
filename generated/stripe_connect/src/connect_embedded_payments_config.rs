#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConnectEmbeddedPaymentsConfig {
    /// Whether the embedded component is enabled.
    pub enabled: bool,
    pub features: stripe_connect::ConnectEmbeddedPaymentsFeatures,
}
#[doc(hidden)]
pub struct ConnectEmbeddedPaymentsConfigBuilder {
    enabled: Option<bool>,
    features: Option<stripe_connect::ConnectEmbeddedPaymentsFeatures>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for ConnectEmbeddedPaymentsConfig {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ConnectEmbeddedPaymentsConfig>,
        builder: ConnectEmbeddedPaymentsConfigBuilder,
    }

    impl Visitor for Place<ConnectEmbeddedPaymentsConfig> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ConnectEmbeddedPaymentsConfigBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ConnectEmbeddedPaymentsConfigBuilder {
        type Out = ConnectEmbeddedPaymentsConfig;
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
            Some(Self::Out { enabled: self.enabled?, features: self.features? })
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

    impl ObjectDeser for ConnectEmbeddedPaymentsConfig {
        type Builder = ConnectEmbeddedPaymentsConfigBuilder;
    }

    impl FromValueOpt for ConnectEmbeddedPaymentsConfig {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ConnectEmbeddedPaymentsConfigBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "enabled" => b.enabled = Some(FromValueOpt::from_value(v)?),
                    "features" => b.features = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
