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
                builder: TaxProductResourceTaxTransactionResourceReversalBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TaxProductResourceTaxTransactionResourceReversalBuilder {
        type Out = TaxProductResourceTaxTransactionResourceReversal;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "original_transaction" => Deserialize::begin(&mut self.original_transaction),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { original_transaction: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self { original_transaction: Some(original_transaction) } => {
                    Some(Self::Out { original_transaction })
                }
                builder => {
                    *self = builder;
                    None
                }
            }
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for TaxProductResourceTaxTransactionResourceReversal {
        type Builder = TaxProductResourceTaxTransactionResourceReversalBuilder;
    }

    impl FromValueOpt for TaxProductResourceTaxTransactionResourceReversal {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
