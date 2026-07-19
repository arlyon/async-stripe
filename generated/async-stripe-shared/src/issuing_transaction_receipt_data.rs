#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingTransactionReceiptData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingTransactionReceiptData").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: IssuingTransactionReceiptDataBuilder {
                    description: Deserialize::default(),
                    quantity: Deserialize::default(),
                    total: Deserialize::default(),
                    unit_cost: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "description" => Deserialize::begin(&mut self.builder.description),
                "quantity" => Deserialize::begin(&mut self.builder.quantity),
                "total" => Deserialize::begin(&mut self.builder.total),
                "unit_cost" => Deserialize::begin(&mut self.builder.unit_cost),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(description), Some(quantity), Some(total), Some(unit_cost)) = (
                self.builder.description.take(),
                self.builder.quantity,
                self.builder.total,
                self.builder.unit_cost,
            ) else {
                return Ok(());
            };
            *self.out =
                Some(IssuingTransactionReceiptData { description, quantity, total, unit_cost });
            Ok(())
        }
    }
};
