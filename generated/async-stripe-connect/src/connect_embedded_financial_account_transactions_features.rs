#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConnectEmbeddedFinancialAccountTransactionsFeatures {
    /// Whether to allow card spend dispute management features.
    pub card_spend_dispute_management: bool,
}
#[doc(hidden)]
pub struct ConnectEmbeddedFinancialAccountTransactionsFeaturesBuilder {
    card_spend_dispute_management: Option<bool>,
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
                builder: ConnectEmbeddedFinancialAccountTransactionsFeaturesBuilder::deser_default(
                ),
            }))
        }
    }

    impl MapBuilder for ConnectEmbeddedFinancialAccountTransactionsFeaturesBuilder {
        type Out = ConnectEmbeddedFinancialAccountTransactionsFeatures;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "card_spend_dispute_management" => {
                    Deserialize::begin(&mut self.card_spend_dispute_management)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { card_spend_dispute_management: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(card_spend_dispute_management),) = (self.card_spend_dispute_management,)
            else {
                return None;
            };
            Some(Self::Out { card_spend_dispute_management })
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

    impl ObjectDeser for ConnectEmbeddedFinancialAccountTransactionsFeatures {
        type Builder = ConnectEmbeddedFinancialAccountTransactionsFeaturesBuilder;
    }

    impl FromValueOpt for ConnectEmbeddedFinancialAccountTransactionsFeatures {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ConnectEmbeddedFinancialAccountTransactionsFeaturesBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "card_spend_dispute_management" => {
                        b.card_spend_dispute_management = FromValueOpt::from_value(v)
                    }
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
