/// Represents a line item to be displayed on the reader
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalReaderReaderResourceLineItem {
    /// The amount of the line item.
    /// A positive integer in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal).
    pub amount: i64,
    /// Description of the line item.
    pub description: String,
    /// The quantity of the line item.
    pub quantity: u64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalReaderReaderResourceLineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalReaderReaderResourceLineItem").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TerminalReaderReaderResourceLineItemBuilder {
    amount: Option<i64>,
    description: Option<String>,
    quantity: Option<u64>,
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

    impl Deserialize for TerminalReaderReaderResourceLineItem {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalReaderReaderResourceLineItem>,
        builder: TerminalReaderReaderResourceLineItemBuilder,
    }

    impl Visitor for Place<TerminalReaderReaderResourceLineItem> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TerminalReaderReaderResourceLineItemBuilder {
                    amount: Deserialize::default(),
                    description: Deserialize::default(),
                    quantity: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "description" => Deserialize::begin(&mut self.builder.description),
                "quantity" => Deserialize::begin(&mut self.builder.quantity),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(amount), Some(description), Some(quantity)) =
                (self.builder.amount, self.builder.description.take(), self.builder.quantity)
            else {
                return Ok(());
            };
            *self.out =
                Some(TerminalReaderReaderResourceLineItem { amount, description, quantity });
            Ok(())
        }
    }
};
