#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConnectEmbeddedIssuingCardsListConfigClaim {
    /// Whether the embedded component is enabled.
    pub enabled: bool,
    pub features: stripe_connect::ConnectEmbeddedIssuingCardsListFeatures,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConnectEmbeddedIssuingCardsListConfigClaim {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConnectEmbeddedIssuingCardsListConfigClaim").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ConnectEmbeddedIssuingCardsListConfigClaimBuilder {
    enabled: Option<bool>,
    features: Option<stripe_connect::ConnectEmbeddedIssuingCardsListFeatures>,
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

    impl Deserialize for ConnectEmbeddedIssuingCardsListConfigClaim {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ConnectEmbeddedIssuingCardsListConfigClaim>,
        builder: ConnectEmbeddedIssuingCardsListConfigClaimBuilder,
    }

    impl Visitor for Place<ConnectEmbeddedIssuingCardsListConfigClaim> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ConnectEmbeddedIssuingCardsListConfigClaimBuilder {
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
            *self.out = Some(ConnectEmbeddedIssuingCardsListConfigClaim { enabled, features });
            Ok(())
        }
    }
};
