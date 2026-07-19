#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingTransactionTreasury {
    /// The Treasury [ReceivedCredit](https://docs.stripe.com/api/treasury/received_credits) representing this Issuing transaction if it is a refund.
    pub received_credit: Option<String>,
    /// The Treasury [ReceivedDebit](https://docs.stripe.com/api/treasury/received_debits) representing this Issuing transaction if it is a capture.
    pub received_debit: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingTransactionTreasury {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingTransactionTreasury").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingTransactionTreasuryBuilder {
    received_credit: Option<Option<String>>,
    received_debit: Option<Option<String>>,
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

    impl Deserialize for IssuingTransactionTreasury {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingTransactionTreasury>,
        builder: IssuingTransactionTreasuryBuilder,
    }

    impl Visitor for Place<IssuingTransactionTreasury> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingTransactionTreasuryBuilder {
                    received_credit: Deserialize::default(),
                    received_debit: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "received_credit" => Deserialize::begin(&mut self.builder.received_credit),
                "received_debit" => Deserialize::begin(&mut self.builder.received_debit),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(received_credit), Some(received_debit)) =
                (self.builder.received_credit.take(), self.builder.received_debit.take())
            else {
                return Ok(());
            };
            *self.out = Some(IssuingTransactionTreasury { received_credit, received_debit });
            Ok(())
        }
    }
};
