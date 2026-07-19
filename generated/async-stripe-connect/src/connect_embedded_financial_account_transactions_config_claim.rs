#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConnectEmbeddedFinancialAccountTransactionsConfigClaim {
    /// Whether the embedded component is enabled.
    pub enabled: bool,
    pub features: stripe_connect::ConnectEmbeddedFinancialAccountTransactionsFeatures,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConnectEmbeddedFinancialAccountTransactionsConfigClaim {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConnectEmbeddedFinancialAccountTransactionsConfigClaim")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ConnectEmbeddedFinancialAccountTransactionsConfigClaimBuilder {
    enabled: Option<bool>,
    features: Option<stripe_connect::ConnectEmbeddedFinancialAccountTransactionsFeatures>,
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

    impl Deserialize for ConnectEmbeddedFinancialAccountTransactionsConfigClaim {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ConnectEmbeddedFinancialAccountTransactionsConfigClaim>,
        builder: ConnectEmbeddedFinancialAccountTransactionsConfigClaimBuilder,
    }

    impl Visitor for Place<ConnectEmbeddedFinancialAccountTransactionsConfigClaim> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ConnectEmbeddedFinancialAccountTransactionsConfigClaimBuilder {
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
            *self.out =
                Some(ConnectEmbeddedFinancialAccountTransactionsConfigClaim { enabled, features });
            Ok(())
        }
    }
};
