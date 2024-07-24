#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxCalculationLineItem {
    /// The line item amount in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    /// If `tax_behavior=inclusive`, then this amount includes taxes.
    /// Otherwise, taxes were calculated on top of this amount.
    pub amount: i64,
    /// The amount of tax calculated for this line item, in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount_tax: i64,
    /// Unique identifier for the object.
    pub id: stripe_misc::TaxCalculationLineItemId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The ID of an existing [Product](https://stripe.com/docs/api/products/object).
    pub product: Option<String>,
    /// The number of units of the item being purchased. For reversals, this is the quantity reversed.
    pub quantity: u64,
    /// A custom identifier for this line item.
    pub reference: Option<String>,
    /// Specifies whether the `amount` includes taxes.
    /// If `tax_behavior=inclusive`, then the amount includes taxes.
    pub tax_behavior: TaxCalculationLineItemTaxBehavior,
    /// Detailed account of taxes relevant to this line item.
    pub tax_breakdown: Option<Vec<stripe_misc::TaxProductResourceLineItemTaxBreakdown>>,
    /// The [tax code](https://stripe.com/docs/tax/tax-categories) ID used for this resource.
    pub tax_code: String,
}
#[doc(hidden)]
pub struct TaxCalculationLineItemBuilder {
    amount: Option<i64>,
    amount_tax: Option<i64>,
    id: Option<stripe_misc::TaxCalculationLineItemId>,
    livemode: Option<bool>,
    product: Option<Option<String>>,
    quantity: Option<u64>,
    reference: Option<Option<String>>,
    tax_behavior: Option<TaxCalculationLineItemTaxBehavior>,
    tax_breakdown: Option<Option<Vec<stripe_misc::TaxProductResourceLineItemTaxBreakdown>>>,
    tax_code: Option<String>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxCalculationLineItem {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxCalculationLineItem>,
        builder: TaxCalculationLineItemBuilder,
    }

    impl Visitor for Place<TaxCalculationLineItem> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxCalculationLineItemBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TaxCalculationLineItemBuilder {
        type Out = TaxCalculationLineItem;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "amount_tax" => Deserialize::begin(&mut self.amount_tax),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "product" => Deserialize::begin(&mut self.product),
                "quantity" => Deserialize::begin(&mut self.quantity),
                "reference" => Deserialize::begin(&mut self.reference),
                "tax_behavior" => Deserialize::begin(&mut self.tax_behavior),
                "tax_breakdown" => Deserialize::begin(&mut self.tax_breakdown),
                "tax_code" => Deserialize::begin(&mut self.tax_code),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                amount_tax: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                product: Deserialize::default(),
                quantity: Deserialize::default(),
                reference: Deserialize::default(),
                tax_behavior: Deserialize::default(),
                tax_breakdown: Deserialize::default(),
                tax_code: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                amount: self.amount?,
                amount_tax: self.amount_tax?,
                id: self.id.take()?,
                livemode: self.livemode?,
                product: self.product.take()?,
                quantity: self.quantity?,
                reference: self.reference.take()?,
                tax_behavior: self.tax_behavior?,
                tax_breakdown: self.tax_breakdown.take()?,
                tax_code: self.tax_code.take()?,
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

    impl ObjectDeser for TaxCalculationLineItem {
        type Builder = TaxCalculationLineItemBuilder;
    }

    impl FromValueOpt for TaxCalculationLineItem {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxCalculationLineItemBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = Some(FromValueOpt::from_value(v)?),
                    "amount_tax" => b.amount_tax = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "livemode" => b.livemode = Some(FromValueOpt::from_value(v)?),
                    "product" => b.product = Some(FromValueOpt::from_value(v)?),
                    "quantity" => b.quantity = Some(FromValueOpt::from_value(v)?),
                    "reference" => b.reference = Some(FromValueOpt::from_value(v)?),
                    "tax_behavior" => b.tax_behavior = Some(FromValueOpt::from_value(v)?),
                    "tax_breakdown" => b.tax_breakdown = Some(FromValueOpt::from_value(v)?),
                    "tax_code" => b.tax_code = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxCalculationLineItem {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("TaxCalculationLineItem", 11)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("amount_tax", &self.amount_tax)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("product", &self.product)?;
        s.serialize_field("quantity", &self.quantity)?;
        s.serialize_field("reference", &self.reference)?;
        s.serialize_field("tax_behavior", &self.tax_behavior)?;
        s.serialize_field("tax_breakdown", &self.tax_breakdown)?;
        s.serialize_field("tax_code", &self.tax_code)?;

        s.serialize_field("object", "tax.calculation_line_item")?;
        s.end()
    }
}
/// Specifies whether the `amount` includes taxes.
/// If `tax_behavior=inclusive`, then the amount includes taxes.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxCalculationLineItemTaxBehavior {
    Exclusive,
    Inclusive,
}
impl TaxCalculationLineItemTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use TaxCalculationLineItemTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
        }
    }
}

impl std::str::FromStr for TaxCalculationLineItemTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxCalculationLineItemTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TaxCalculationLineItemTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxCalculationLineItemTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxCalculationLineItemTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TaxCalculationLineItemTaxBehavior {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TaxCalculationLineItemTaxBehavior> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(TaxCalculationLineItemTaxBehavior::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TaxCalculationLineItemTaxBehavior);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxCalculationLineItemTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for TaxCalculationLineItemTaxBehavior")
        })
    }
}
impl stripe_types::Object for TaxCalculationLineItem {
    type Id = stripe_misc::TaxCalculationLineItemId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(TaxCalculationLineItemId);
