#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingAuthorizationTreasury {
    /// The array of [ReceivedCredits](https://stripe.com/docs/api/treasury/received_credits) associated with this authorization.
    pub received_credits: Vec<String>,
    /// The array of [ReceivedDebits](https://stripe.com/docs/api/treasury/received_debits) associated with this authorization.
    pub received_debits: Vec<String>,
    /// The Treasury [Transaction](https://stripe.com/docs/api/treasury/transactions) associated with this authorization.
    pub transaction: Option<String>,
}
#[doc(hidden)]
pub struct IssuingAuthorizationTreasuryBuilder {
    received_credits: Option<Vec<String>>,
    received_debits: Option<Vec<String>>,
    transaction: Option<Option<String>>,
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

    impl Deserialize for IssuingAuthorizationTreasury {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingAuthorizationTreasury>,
        builder: IssuingAuthorizationTreasuryBuilder,
    }

    impl Visitor for Place<IssuingAuthorizationTreasury> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingAuthorizationTreasuryBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingAuthorizationTreasuryBuilder {
        type Out = IssuingAuthorizationTreasury;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "received_credits" => Deserialize::begin(&mut self.received_credits),
                "received_debits" => Deserialize::begin(&mut self.received_debits),
                "transaction" => Deserialize::begin(&mut self.transaction),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                received_credits: Deserialize::default(),
                received_debits: Deserialize::default(),
                transaction: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(received_credits), Some(received_debits), Some(transaction)) = (
                self.received_credits.take(),
                self.received_debits.take(),
                self.transaction.take(),
            ) else {
                return None;
            };
            Some(Self::Out { received_credits, received_debits, transaction })
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

    impl ObjectDeser for IssuingAuthorizationTreasury {
        type Builder = IssuingAuthorizationTreasuryBuilder;
    }

    impl FromValueOpt for IssuingAuthorizationTreasury {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingAuthorizationTreasuryBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "received_credits" => b.received_credits = FromValueOpt::from_value(v),
                    "received_debits" => b.received_debits = FromValueOpt::from_value(v),
                    "transaction" => b.transaction = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
