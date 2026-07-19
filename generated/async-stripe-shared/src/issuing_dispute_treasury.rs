#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingDisputeTreasury {
    /// The Treasury [DebitReversal](https://docs.stripe.com/api/treasury/debit_reversals) representing this Issuing dispute.
    pub debit_reversal: Option<String>,
    /// The Treasury [ReceivedDebit](https://docs.stripe.com/api/treasury/received_debits) that is being disputed.
    pub received_debit: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingDisputeTreasury {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingDisputeTreasury").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingDisputeTreasuryBuilder {
    debit_reversal: Option<Option<String>>,
    received_debit: Option<String>,
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

    impl Deserialize for IssuingDisputeTreasury {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingDisputeTreasury>,
        builder: IssuingDisputeTreasuryBuilder,
    }

    impl Visitor for Place<IssuingDisputeTreasury> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingDisputeTreasuryBuilder {
                    debit_reversal: Deserialize::default(),
                    received_debit: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "debit_reversal" => Deserialize::begin(&mut self.builder.debit_reversal),
                "received_debit" => Deserialize::begin(&mut self.builder.received_debit),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(debit_reversal), Some(received_debit)) =
                (self.builder.debit_reversal.take(), self.builder.received_debit.take())
            else {
                return Ok(());
            };
            *self.out = Some(IssuingDisputeTreasury { debit_reversal, received_debit });
            Ok(())
        }
    }
};
