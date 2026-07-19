#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConnectEmbeddedFinancialAccountTransactionsFeatures {
    /// Whether to allow card spend dispute management features.
    pub card_spend_dispute_management: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConnectEmbeddedFinancialAccountTransactionsFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConnectEmbeddedFinancialAccountTransactionsFeatures")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ConnectEmbeddedFinancialAccountTransactionsFeaturesBuilder {
    card_spend_dispute_management: Option<bool>,
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

    impl Deserialize for ConnectEmbeddedFinancialAccountTransactionsFeatures {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ConnectEmbeddedFinancialAccountTransactionsFeatures>,
        builder: ConnectEmbeddedFinancialAccountTransactionsFeaturesBuilder,
    }

    impl Visitor for Place<ConnectEmbeddedFinancialAccountTransactionsFeatures> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ConnectEmbeddedFinancialAccountTransactionsFeaturesBuilder {
                    card_spend_dispute_management: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "card_spend_dispute_management" => {
                    Deserialize::begin(&mut self.builder.card_spend_dispute_management)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(card_spend_dispute_management),) =
                (self.builder.card_spend_dispute_management,)
            else {
                return Ok(());
            };
            *self.out = Some(ConnectEmbeddedFinancialAccountTransactionsFeatures {
                card_spend_dispute_management,
            });
            Ok(())
        }
    }
};
