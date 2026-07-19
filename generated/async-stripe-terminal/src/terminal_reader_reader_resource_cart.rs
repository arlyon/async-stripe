/// Represents a cart to be displayed on the reader
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalReaderReaderResourceCart {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// List of line items in the cart.
    pub line_items: Vec<stripe_terminal::TerminalReaderReaderResourceLineItem>,
    /// Tax amount for the entire cart.
    /// A positive integer in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal).
    pub tax: Option<i64>,
    /// Total amount for the entire cart, including tax.
    /// A positive integer in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal).
    pub total: i64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalReaderReaderResourceCart {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalReaderReaderResourceCart").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TerminalReaderReaderResourceCartBuilder {
    currency: Option<stripe_types::Currency>,
    line_items: Option<Vec<stripe_terminal::TerminalReaderReaderResourceLineItem>>,
    tax: Option<Option<i64>>,
    total: Option<i64>,
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

    impl Deserialize for TerminalReaderReaderResourceCart {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalReaderReaderResourceCart>,
        builder: TerminalReaderReaderResourceCartBuilder,
    }

    impl Visitor for Place<TerminalReaderReaderResourceCart> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TerminalReaderReaderResourceCartBuilder {
                    currency: Deserialize::default(),
                    line_items: Deserialize::default(),
                    tax: Deserialize::default(),
                    total: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "currency" => Deserialize::begin(&mut self.builder.currency),
                "line_items" => Deserialize::begin(&mut self.builder.line_items),
                "tax" => Deserialize::begin(&mut self.builder.tax),
                "total" => Deserialize::begin(&mut self.builder.total),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(currency), Some(line_items), Some(tax), Some(total)) = (
                self.builder.currency.take(),
                self.builder.line_items.take(),
                self.builder.tax,
                self.builder.total,
            ) else {
                return Ok(());
            };
            *self.out = Some(TerminalReaderReaderResourceCart { currency, line_items, tax, total });
            Ok(())
        }
    }
};
