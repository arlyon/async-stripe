/// A line item.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CheckoutSessionItem {
    /// Total discount amount applied. If no discounts were applied, defaults to 0.
    pub amount_discount: i64,
    /// Total before any discounts or taxes are applied.
    pub amount_subtotal: i64,
    /// Total tax amount applied. If no tax was applied, defaults to 0.
    pub amount_tax: i64,
    /// Total after discounts and taxes.
    pub amount_total: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object.
    /// Often useful for displaying to users.
    /// Defaults to product name.
    pub description: Option<String>,
    /// The discounts applied to the line item.
    pub discounts: Option<Vec<stripe_shared::LineItemsDiscountAmount>>,
    /// Unique identifier for the object.
    pub id: stripe_shared::CheckoutSessionItemId,
    /// The price used to generate the line item.
    pub price: Option<stripe_shared::Price>,
    /// The quantity of products being purchased.
    pub quantity: Option<u64>,
    /// The taxes applied to the line item.
    pub taxes: Option<Vec<stripe_shared::LineItemsTaxAmount>>,
}
#[doc(hidden)]
pub struct CheckoutSessionItemBuilder {
    amount_discount: Option<i64>,
    amount_subtotal: Option<i64>,
    amount_tax: Option<i64>,
    amount_total: Option<i64>,
    currency: Option<stripe_types::Currency>,
    description: Option<Option<String>>,
    discounts: Option<Option<Vec<stripe_shared::LineItemsDiscountAmount>>>,
    id: Option<stripe_shared::CheckoutSessionItemId>,
    price: Option<Option<stripe_shared::Price>>,
    quantity: Option<Option<u64>>,
    taxes: Option<Option<Vec<stripe_shared::LineItemsTaxAmount>>>,
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

    impl Deserialize for CheckoutSessionItem {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CheckoutSessionItem>,
        builder: CheckoutSessionItemBuilder,
    }

    impl Visitor for Place<CheckoutSessionItem> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CheckoutSessionItemBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CheckoutSessionItemBuilder {
        type Out = CheckoutSessionItem;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_discount" => Deserialize::begin(&mut self.amount_discount),
                "amount_subtotal" => Deserialize::begin(&mut self.amount_subtotal),
                "amount_tax" => Deserialize::begin(&mut self.amount_tax),
                "amount_total" => Deserialize::begin(&mut self.amount_total),
                "currency" => Deserialize::begin(&mut self.currency),
                "description" => Deserialize::begin(&mut self.description),
                "discounts" => Deserialize::begin(&mut self.discounts),
                "id" => Deserialize::begin(&mut self.id),
                "price" => Deserialize::begin(&mut self.price),
                "quantity" => Deserialize::begin(&mut self.quantity),
                "taxes" => Deserialize::begin(&mut self.taxes),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount_discount: Deserialize::default(),
                amount_subtotal: Deserialize::default(),
                amount_tax: Deserialize::default(),
                amount_total: Deserialize::default(),
                currency: Deserialize::default(),
                description: Deserialize::default(),
                discounts: Deserialize::default(),
                id: Deserialize::default(),
                price: Deserialize::default(),
                quantity: Deserialize::default(),
                taxes: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(amount_discount),
                Some(amount_subtotal),
                Some(amount_tax),
                Some(amount_total),
                Some(currency),
                Some(description),
                Some(discounts),
                Some(id),
                Some(price),
                Some(quantity),
                Some(taxes),
            ) = (
                self.amount_discount,
                self.amount_subtotal,
                self.amount_tax,
                self.amount_total,
                self.currency.take(),
                self.description.take(),
                self.discounts.take(),
                self.id.take(),
                self.price.take(),
                self.quantity,
                self.taxes.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                amount_discount,
                amount_subtotal,
                amount_tax,
                amount_total,
                currency,
                description,
                discounts,
                id,
                price,
                quantity,
                taxes,
            })
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

    impl ObjectDeser for CheckoutSessionItem {
        type Builder = CheckoutSessionItemBuilder;
    }

    impl FromValueOpt for CheckoutSessionItem {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CheckoutSessionItemBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount_discount" => b.amount_discount = FromValueOpt::from_value(v),
                    "amount_subtotal" => b.amount_subtotal = FromValueOpt::from_value(v),
                    "amount_tax" => b.amount_tax = FromValueOpt::from_value(v),
                    "amount_total" => b.amount_total = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "description" => b.description = FromValueOpt::from_value(v),
                    "discounts" => b.discounts = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "price" => b.price = FromValueOpt::from_value(v),
                    "quantity" => b.quantity = FromValueOpt::from_value(v),
                    "taxes" => b.taxes = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for CheckoutSessionItem {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("CheckoutSessionItem", 12)?;
        s.serialize_field("amount_discount", &self.amount_discount)?;
        s.serialize_field("amount_subtotal", &self.amount_subtotal)?;
        s.serialize_field("amount_tax", &self.amount_tax)?;
        s.serialize_field("amount_total", &self.amount_total)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("discounts", &self.discounts)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("price", &self.price)?;
        s.serialize_field("quantity", &self.quantity)?;
        s.serialize_field("taxes", &self.taxes)?;

        s.serialize_field("object", "item")?;
        s.end()
    }
}
impl stripe_types::Object for CheckoutSessionItem {
    type Id = stripe_shared::CheckoutSessionItemId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(CheckoutSessionItemId);
