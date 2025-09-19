#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingDisputeTreasury {
    /// The Treasury [DebitReversal](https://stripe.com/docs/api/treasury/debit_reversals) representing this Issuing dispute.
    pub debit_reversal: Option<String>,
    /// The Treasury [ReceivedDebit](https://stripe.com/docs/api/treasury/received_debits) that is being disputed.
    pub received_debit: String,
}
#[doc(hidden)]
pub struct IssuingDisputeTreasuryBuilder {
    debit_reversal: Option<Option<String>>,
    received_debit: Option<String>,
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
                builder: IssuingDisputeTreasuryBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingDisputeTreasuryBuilder {
        type Out = IssuingDisputeTreasury;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "debit_reversal" => Deserialize::begin(&mut self.debit_reversal),
                "received_debit" => Deserialize::begin(&mut self.received_debit),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { debit_reversal: Deserialize::default(), received_debit: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(debit_reversal), Some(received_debit)) =
                (self.debit_reversal.take(), self.received_debit.take())
            else {
                return None;
            };
            Some(Self::Out { debit_reversal, received_debit })
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

    impl ObjectDeser for IssuingDisputeTreasury {
        type Builder = IssuingDisputeTreasuryBuilder;
    }

    impl FromValueOpt for IssuingDisputeTreasury {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingDisputeTreasuryBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "debit_reversal" => b.debit_reversal = FromValueOpt::from_value(v),
                    "received_debit" => b.received_debit = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
