#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductResourceTaxTransactionResourceReversal {
    /// The `id` of the reversed `Transaction` object.
    pub original_transaction: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductResourceTaxTransactionResourceReversal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxProductResourceTaxTransactionResourceReversal").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TaxProductResourceTaxTransactionResourceReversalBuilder {
    original_transaction: Option<Option<String>>,
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

    impl Deserialize for TaxProductResourceTaxTransactionResourceReversal {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductResourceTaxTransactionResourceReversal>,
        builder: TaxProductResourceTaxTransactionResourceReversalBuilder,
    }

    impl Visitor for Place<TaxProductResourceTaxTransactionResourceReversal> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxProductResourceTaxTransactionResourceReversalBuilder {
                    original_transaction: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "original_transaction" => {
                    Deserialize::begin(&mut self.builder.original_transaction)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(original_transaction),) = (self.builder.original_transaction.take(),) else {
                return Ok(());
            };
            *self.out =
                Some(TaxProductResourceTaxTransactionResourceReversal { original_transaction });
            Ok(())
        }
    }
};
