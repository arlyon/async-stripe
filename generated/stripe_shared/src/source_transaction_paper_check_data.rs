#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceTransactionPaperCheckData {
    /// Time at which the deposited funds will be available for use.
    /// Measured in seconds since the Unix epoch.
    pub available_at: Option<String>,
    /// Comma-separated list of invoice IDs associated with the paper check.
    pub invoices: Option<String>,
}
#[doc(hidden)]
pub struct SourceTransactionPaperCheckDataBuilder {
    available_at: Option<Option<String>>,
    invoices: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: SourceTransactionPaperCheckDataBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SourceTransactionPaperCheckDataBuilder {
        type Out = SourceTransactionPaperCheckData;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "available_at" => Deserialize::begin(&mut self.available_at),
                "invoices" => Deserialize::begin(&mut self.invoices),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { available_at: Deserialize::default(), invoices: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                available_at: self.available_at.take()?,
                invoices: self.invoices.take()?,
            })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for SourceTransactionPaperCheckData {
        type Builder = SourceTransactionPaperCheckDataBuilder;
    }

    impl FromValueOpt for SourceTransactionPaperCheckData {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SourceTransactionPaperCheckDataBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "available_at" => b.available_at = Some(FromValueOpt::from_value(v)?),
                    "invoices" => b.invoices = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
