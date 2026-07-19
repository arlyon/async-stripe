#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceOrderItem {
    /// The amount (price) for this order item.
    pub amount: Option<i64>,
    /// This currency of this order item. Required when `amount` is present.
    pub currency: Option<stripe_types::Currency>,
    /// Human-readable description for this order item.
    pub description: Option<String>,
    /// The ID of the associated object for this line item.
    /// Expandable if not null (e.g., expandable to a SKU).
    pub parent: Option<String>,
    /// The quantity of this order item.
    /// When type is `sku`, this is the number of instances of the SKU to be ordered.
    pub quantity: Option<u64>,
    /// The type of this order item. Must be `sku`, `tax`, or `shipping`.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SourceOrderItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SourceOrderItem").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SourceOrderItemBuilder {
    amount: Option<Option<i64>>,
    currency: Option<Option<stripe_types::Currency>>,
    description: Option<Option<String>>,
    parent: Option<Option<String>>,
    quantity: Option<Option<u64>>,
    type_: Option<Option<String>>,
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

    impl Deserialize for SourceOrderItem {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceOrderItem>,
        builder: SourceOrderItemBuilder,
    }

    impl Visitor for Place<SourceOrderItem> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceOrderItemBuilder {
                    amount: Deserialize::default(),
                    currency: Deserialize::default(),
                    description: Deserialize::default(),
                    parent: Deserialize::default(),
                    quantity: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "currency" => Deserialize::begin(&mut self.builder.currency),
                "description" => Deserialize::begin(&mut self.builder.description),
                "parent" => Deserialize::begin(&mut self.builder.parent),
                "quantity" => Deserialize::begin(&mut self.builder.quantity),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(amount),
                Some(currency),
                Some(description),
                Some(parent),
                Some(quantity),
                Some(type_),
            ) = (
                self.builder.amount,
                self.builder.currency.take(),
                self.builder.description.take(),
                self.builder.parent.take(),
                self.builder.quantity,
                self.builder.type_.take(),
            )
            else {
                return Ok(());
            };
            *self.out =
                Some(SourceOrderItem { amount, currency, description, parent, quantity, type_ });
            Ok(())
        }
    }
};
