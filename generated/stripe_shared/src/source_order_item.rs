#[derive(Clone, Debug, Default)]
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
#[doc(hidden)]
pub struct SourceOrderItemBuilder {
    amount: Option<Option<i64>>,
    currency: Option<Option<stripe_types::Currency>>,
    description: Option<Option<String>>,
    parent: Option<Option<String>>,
    quantity: Option<Option<u64>>,
    type_: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: SourceOrderItemBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SourceOrderItemBuilder {
        type Out = SourceOrderItem;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "currency" => Deserialize::begin(&mut self.currency),
                "description" => Deserialize::begin(&mut self.description),
                "parent" => Deserialize::begin(&mut self.parent),
                "quantity" => Deserialize::begin(&mut self.quantity),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                currency: Deserialize::default(),
                description: Deserialize::default(),
                parent: Deserialize::default(),
                quantity: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                amount: self.amount?,
                currency: self.currency?,
                description: self.description.take()?,
                parent: self.parent.take()?,
                quantity: self.quantity?,
                type_: self.type_.take()?,
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

    impl ObjectDeser for SourceOrderItem {
        type Builder = SourceOrderItemBuilder;
    }

    impl FromValueOpt for SourceOrderItem {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SourceOrderItemBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = Some(FromValueOpt::from_value(v)?),
                    "currency" => b.currency = Some(FromValueOpt::from_value(v)?),
                    "description" => b.description = Some(FromValueOpt::from_value(v)?),
                    "parent" => b.parent = Some(FromValueOpt::from_value(v)?),
                    "quantity" => b.quantity = Some(FromValueOpt::from_value(v)?),
                    "type" => b.type_ = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
