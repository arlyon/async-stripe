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
            Self { is_revision: None, quote: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self { is_revision: Some(is_revision), quote: Some(quote) } => {
                    Some(Self::Out { is_revision, quote })
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

    impl ObjectDeser for QuotesResourceFromQuote {
        type Builder = QuotesResourceFromQuoteBuilder;
    }

    impl FromValueOpt for QuotesResourceFromQuote {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
