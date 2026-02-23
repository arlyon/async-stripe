#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingTransactionTreasury {
    /// The Treasury [ReceivedCredit](https://docs.stripe.com/api/treasury/received_credits) representing this Issuing transaction if it is a refund.
    pub received_credit: Option<String>,
    /// The Treasury [ReceivedDebit](https://docs.stripe.com/api/treasury/received_debits) representing this Issuing transaction if it is a capture.
    pub received_debit: Option<String>,
}
#[doc(hidden)]
pub struct IssuingTransactionTreasuryBuilder {
    received_credit: Option<Option<String>>,
    received_debit: Option<Option<String>>,
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
                builder: IssuingTransactionTreasuryBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingTransactionTreasuryBuilder {
        type Out = IssuingTransactionTreasury;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "received_credit" => Deserialize::begin(&mut self.received_credit),
                "received_debit" => Deserialize::begin(&mut self.received_debit),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { received_credit: Deserialize::default(), received_debit: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(received_credit), Some(received_debit)) =
                (self.received_credit.take(), self.received_debit.take())
            else {
                return None;
            };
            Some(Self::Out { received_credit, received_debit })
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

    impl ObjectDeser for IssuingTransactionTreasury {
        type Builder = IssuingTransactionTreasuryBuilder;
    }

    impl FromValueOpt for IssuingTransactionTreasury {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingTransactionTreasuryBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "received_credit" => b.received_credit = FromValueOpt::from_value(v),
                    "received_debit" => b.received_debit = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
