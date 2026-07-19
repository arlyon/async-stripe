#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingAuthorizationTreasury {
    /// The array of [ReceivedCredits](https://docs.stripe.com/api/treasury/received_credits) associated with this authorization.
    pub received_credits: Vec<String>,
    /// The array of [ReceivedDebits](https://docs.stripe.com/api/treasury/received_debits) associated with this authorization.
    pub received_debits: Vec<String>,
    /// The Treasury [Transaction](https://docs.stripe.com/api/treasury/transactions) associated with this authorization.
    pub transaction: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingAuthorizationTreasury {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingAuthorizationTreasury").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: IssuingAuthorizationTreasuryBuilder {
                    received_credits: Deserialize::default(),
                    received_debits: Deserialize::default(),
                    transaction: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "received_credits" => Deserialize::begin(&mut self.builder.received_credits),
                "received_debits" => Deserialize::begin(&mut self.builder.received_debits),
                "transaction" => Deserialize::begin(&mut self.builder.transaction),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(received_credits), Some(received_debits), Some(transaction)) = (
                self.builder.received_credits.take(),
                self.builder.received_debits.take(),
                self.builder.transaction.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(IssuingAuthorizationTreasury {
                received_credits,
                received_debits,
                transaction,
            });
            Ok(())
        }
    }
};
