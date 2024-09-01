#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingTransactionReceiptData {
    /// The description of the item. The maximum length of this field is 26 characters.
    pub description: Option<String>,
    /// The quantity of the item.
    pub quantity: Option<f64>,
    /// The total for this line item in cents.
    pub total: Option<i64>,
    /// The unit cost of the item in cents.
    pub unit_cost: Option<i64>,
}
#[doc(hidden)]
pub struct IssuingTransactionReceiptDataBuilder {
    description: Option<Option<String>>,
    quantity: Option<Option<f64>>,
    total: Option<Option<i64>>,
    unit_cost: Option<Option<i64>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingTransactionReceiptData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingTransactionReceiptData>,
        builder: IssuingTransactionReceiptDataBuilder,
    }

    impl Visitor for Place<IssuingTransactionReceiptData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingTransactionReceiptDataBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingTransactionReceiptDataBuilder {
        type Out = IssuingTransactionReceiptData;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "description" => Deserialize::begin(&mut self.description),
                "quantity" => Deserialize::begin(&mut self.quantity),
                "total" => Deserialize::begin(&mut self.total),
                "unit_cost" => Deserialize::begin(&mut self.unit_cost),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                description: Deserialize::default(),
                quantity: Deserialize::default(),
                total: Deserialize::default(),
                unit_cost: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(description), Some(quantity), Some(total), Some(unit_cost)) =
                (self.description.take(), self.quantity, self.total, self.unit_cost)
            else {
                return None;
            };
            Some(Self::Out { description, quantity, total, unit_cost })
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

    impl ObjectDeser for IssuingTransactionReceiptData {
        type Builder = IssuingTransactionReceiptDataBuilder;
    }

    impl FromValueOpt for IssuingTransactionReceiptData {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingTransactionReceiptDataBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "description" => b.description = FromValueOpt::from_value(v),
                    "quantity" => b.quantity = FromValueOpt::from_value(v),
                    "total" => b.total = FromValueOpt::from_value(v),
                    "unit_cost" => b.unit_cost = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
