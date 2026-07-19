#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductResourceTaxAssociationTransactionAttempts {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxProductResourceTaxAssociationTransactionAttempts")
            .finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: TaxProductResourceTaxAssociationTransactionAttemptsBuilder {
                    committed: Deserialize::default(),
                    errored: Deserialize::default(),
                    source: Deserialize::default(),
                    status: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "committed" => Deserialize::begin(&mut self.builder.committed),
                "errored" => Deserialize::begin(&mut self.builder.errored),
                "source" => Deserialize::begin(&mut self.builder.source),
                "status" => Deserialize::begin(&mut self.builder.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(committed), Some(errored), Some(source), Some(status)) = (
                self.builder.committed.take(),
                self.builder.errored.take(),
                self.builder.source.take(),
                self.builder.status.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(TaxProductResourceTaxAssociationTransactionAttempts {
                committed,
                errored,
                source,
                status,
            });
            Ok(())
        }
    }
};
