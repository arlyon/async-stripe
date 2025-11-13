#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductResourceTaxAssociationTransactionAttemptsResourceCommitted {
    /// The [Tax Transaction](https://stripe.com/docs/api/tax/transaction/object)
    pub transaction: String,
}
#[doc(hidden)]
pub struct TaxProductResourceTaxAssociationTransactionAttemptsResourceCommittedBuilder {
    transaction: Option<String>,
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

    impl Deserialize for TaxProductResourceTaxAssociationTransactionAttemptsResourceCommitted {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductResourceTaxAssociationTransactionAttemptsResourceCommitted>,
        builder: TaxProductResourceTaxAssociationTransactionAttemptsResourceCommittedBuilder,
    }

    impl Visitor for Place<TaxProductResourceTaxAssociationTransactionAttemptsResourceCommitted> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: TaxProductResourceTaxAssociationTransactionAttemptsResourceCommittedBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for TaxProductResourceTaxAssociationTransactionAttemptsResourceCommittedBuilder {
        type Out = TaxProductResourceTaxAssociationTransactionAttemptsResourceCommitted;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "transaction" => Deserialize::begin(&mut self.transaction),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { transaction: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(transaction),) = (self.transaction.take(),) else {
                return None;
            };
            Some(Self::Out { transaction })
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

    impl ObjectDeser for TaxProductResourceTaxAssociationTransactionAttemptsResourceCommitted {
        type Builder = TaxProductResourceTaxAssociationTransactionAttemptsResourceCommittedBuilder;
    }

    impl FromValueOpt for TaxProductResourceTaxAssociationTransactionAttemptsResourceCommitted {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxProductResourceTaxAssociationTransactionAttemptsResourceCommittedBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "transaction" => b.transaction = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
