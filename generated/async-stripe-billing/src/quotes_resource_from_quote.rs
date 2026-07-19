#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct QuotesResourceFromQuote {
    /// Whether this quote is a revision of a different quote.
    pub is_revision: bool,
    /// The quote that was cloned.
    pub quote: stripe_types::Expandable<stripe_billing::Quote>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for QuotesResourceFromQuote {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("QuotesResourceFromQuote").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct QuotesResourceFromQuoteBuilder {
    is_revision: Option<bool>,
    quote: Option<stripe_types::Expandable<stripe_billing::Quote>>,
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
                builder: QuotesResourceFromQuoteBuilder {
                    is_revision: Deserialize::default(),
                    quote: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "is_revision" => Deserialize::begin(&mut self.builder.is_revision),
                "quote" => Deserialize::begin(&mut self.builder.quote),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(is_revision), Some(quote)) =
                (self.builder.is_revision, self.builder.quote.take())
            else {
                return Ok(());
            };
            *self.out = Some(QuotesResourceFromQuote { is_revision, quote });
            Ok(())
        }
    }
};
