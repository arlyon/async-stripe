#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConnectEmbeddedBaseConfigClaim {
    /// Whether the embedded component is enabled.
    pub enabled: bool,
    pub features: stripe_connect::ConnectEmbeddedBaseFeatures,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConnectEmbeddedBaseConfigClaim {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConnectEmbeddedBaseConfigClaim").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ConnectEmbeddedBaseConfigClaimBuilder {
    enabled: Option<bool>,
    features: Option<stripe_connect::ConnectEmbeddedBaseFeatures>,
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
                builder: ConnectEmbeddedBaseConfigClaimBuilder {
                    enabled: Deserialize::default(),
                    features: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "enabled" => Deserialize::begin(&mut self.builder.enabled),
                "features" => Deserialize::begin(&mut self.builder.features),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(enabled), Some(features)) = (self.builder.enabled, self.builder.features)
            else {
                return Ok(());
            };
            *self.out = Some(ConnectEmbeddedBaseConfigClaim { enabled, features });
            Ok(())
        }
    }
};
