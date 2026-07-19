#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceTransactionPaperCheckData {
    /// Time at which the deposited funds will be available for use.
    /// Measured in seconds since the Unix epoch.
    pub available_at: Option<String>,
    /// Comma-separated list of invoice IDs associated with the paper check.
    pub invoices: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SourceTransactionPaperCheckData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SourceTransactionPaperCheckData").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SourceTransactionPaperCheckDataBuilder {
    available_at: Option<Option<String>>,
    invoices: Option<Option<String>>,
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

    impl Deserialize for SourceTransactionPaperCheckData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTransactionPaperCheckData>,
        builder: SourceTransactionPaperCheckDataBuilder,
    }

    impl Visitor for Place<SourceTransactionPaperCheckData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceTransactionPaperCheckDataBuilder {
                    available_at: Deserialize::default(),
                    invoices: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "available_at" => Deserialize::begin(&mut self.builder.available_at),
                "invoices" => Deserialize::begin(&mut self.builder.invoices),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(available_at), Some(invoices)) =
                (self.builder.available_at.take(), self.builder.invoices.take())
            else {
                return Ok(());
            };
            *self.out = Some(SourceTransactionPaperCheckData { available_at, invoices });
            Ok(())
        }
    }
};
