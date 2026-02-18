/// Represents a cart to be displayed on the reader
#[derive(Clone, Debug, Eq, PartialEq)]
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
                builder: TerminalReaderReaderResourceCartBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TerminalReaderReaderResourceCartBuilder {
        type Out = TerminalReaderReaderResourceCart;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "currency" => Deserialize::begin(&mut self.currency),
                "line_items" => Deserialize::begin(&mut self.line_items),
                "tax" => Deserialize::begin(&mut self.tax),
                "total" => Deserialize::begin(&mut self.total),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                currency: Deserialize::default(),
                line_items: Deserialize::default(),
                tax: Deserialize::default(),
                total: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(currency), Some(line_items), Some(tax), Some(total)) =
                (self.currency.take(), self.line_items.take(), self.tax, self.total)
            else {
                return None;
            };
            Some(Self::Out { currency, line_items, tax, total })
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

    impl ObjectDeser for TerminalReaderReaderResourceCart {
        type Builder = TerminalReaderReaderResourceCartBuilder;
    }

    impl FromValueOpt for TerminalReaderReaderResourceCart {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TerminalReaderReaderResourceCartBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "line_items" => b.line_items = FromValueOpt::from_value(v),
                    "tax" => b.tax = FromValueOpt::from_value(v),
                    "total" => b.total = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
