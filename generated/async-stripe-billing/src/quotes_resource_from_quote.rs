#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct QuotesResourceFromQuote {
    /// Whether this quote is a revision of a different quote.
    pub is_revision: bool,
    /// The quote that was cloned.
    pub quote: stripe_types::Expandable<stripe_billing::Quote>,
}
#[doc(hidden)]
pub struct QuotesResourceFromQuoteBuilder {
    is_revision: Option<bool>,
    quote: Option<stripe_types::Expandable<stripe_billing::Quote>>,
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

    impl Deserialize for QuotesResourceFromQuote {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<QuotesResourceFromQuote>,
        builder: QuotesResourceFromQuoteBuilder,
    }

    impl Visitor for Place<QuotesResourceFromQuote> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: QuotesResourceFromQuoteBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for QuotesResourceFromQuoteBuilder {
        type Out = QuotesResourceFromQuote;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "is_revision" => Deserialize::begin(&mut self.is_revision),
                "quote" => Deserialize::begin(&mut self.quote),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { is_revision: Deserialize::default(), quote: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(is_revision), Some(quote)) = (self.is_revision, self.quote.take()) else {
                return None;
            };
            Some(Self::Out { is_revision, quote })
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

    impl ObjectDeser for QuotesResourceFromQuote {
        type Builder = QuotesResourceFromQuoteBuilder;
    }

    impl FromValueOpt for QuotesResourceFromQuote {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = QuotesResourceFromQuoteBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "is_revision" => b.is_revision = FromValueOpt::from_value(v),
                    "quote" => b.quote = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
