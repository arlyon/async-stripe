#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductResourceTaxAssociationTransactionAttempts {
    pub committed:
        Option<stripe_misc::TaxProductResourceTaxAssociationTransactionAttemptsResourceCommitted>,
    pub errored:
        Option<stripe_misc::TaxProductResourceTaxAssociationTransactionAttemptsResourceErrored>,
    /// The source of the tax transaction attempt. This is either a refund or a payment intent.
    pub source: String,
    /// The status of the transaction attempt. This can be `errored` or `committed`.
    pub status: String,
}
#[doc(hidden)]
pub struct TaxProductResourceTaxAssociationTransactionAttemptsBuilder {
    committed: Option<
        Option<stripe_misc::TaxProductResourceTaxAssociationTransactionAttemptsResourceCommitted>,
    >,
    errored: Option<
        Option<stripe_misc::TaxProductResourceTaxAssociationTransactionAttemptsResourceErrored>,
    >,
    source: Option<String>,
    status: Option<String>,
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

    impl Deserialize for TaxProductResourceTaxAssociationTransactionAttempts {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductResourceTaxAssociationTransactionAttempts>,
        builder: TaxProductResourceTaxAssociationTransactionAttemptsBuilder,
    }

    impl Visitor for Place<TaxProductResourceTaxAssociationTransactionAttempts> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxProductResourceTaxAssociationTransactionAttemptsBuilder::deser_default(
                ),
            }))
        }
    }

    impl MapBuilder for TaxProductResourceTaxAssociationTransactionAttemptsBuilder {
        type Out = TaxProductResourceTaxAssociationTransactionAttempts;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "committed" => Deserialize::begin(&mut self.committed),
                "errored" => Deserialize::begin(&mut self.errored),
                "source" => Deserialize::begin(&mut self.source),
                "status" => Deserialize::begin(&mut self.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                committed: Deserialize::default(),
                errored: Deserialize::default(),
                source: Deserialize::default(),
                status: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(committed), Some(errored), Some(source), Some(status)) = (
                self.committed.take(),
                self.errored.take(),
                self.source.take(),
                self.status.take(),
            ) else {
                return None;
            };
            Some(Self::Out { committed, errored, source, status })
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

    impl ObjectDeser for TaxProductResourceTaxAssociationTransactionAttempts {
        type Builder = TaxProductResourceTaxAssociationTransactionAttemptsBuilder;
    }

    impl FromValueOpt for TaxProductResourceTaxAssociationTransactionAttempts {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxProductResourceTaxAssociationTransactionAttemptsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "committed" => b.committed = FromValueOpt::from_value(v),
                    "errored" => b.errored = FromValueOpt::from_value(v),
                    "source" => b.source = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
