/// A line item.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CheckoutSessionItem {
    pub adjustable_quantity: Option<stripe_shared::LineItemsAdjustableQuantity>,
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
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The price used to generate the line item.
    pub price: Option<stripe_shared::Price>,
    /// The quantity of products being purchased.
    pub quantity: Option<u64>,
    /// The taxes applied to the line item.
    pub taxes: Option<Vec<stripe_shared::LineItemsTaxAmount>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CheckoutSessionItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CheckoutSessionItem").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CheckoutSessionItemBuilder {
    adjustable_quantity: Option<Option<stripe_shared::LineItemsAdjustableQuantity>>,
    amount_discount: Option<i64>,
    amount_subtotal: Option<i64>,
    amount_tax: Option<i64>,
    amount_total: Option<i64>,
    currency: Option<stripe_types::Currency>,
    description: Option<Option<String>>,
    discounts: Option<Option<Vec<stripe_shared::LineItemsDiscountAmount>>>,
    id: Option<stripe_shared::CheckoutSessionItemId>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    price: Option<Option<stripe_shared::Price>>,
    quantity: Option<Option<u64>>,
    taxes: Option<Option<Vec<stripe_shared::LineItemsTaxAmount>>>,
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
                builder: CheckoutSessionItemBuilder {
                    adjustable_quantity: Deserialize::default(),
                    amount_discount: Deserialize::default(),
                    amount_subtotal: Deserialize::default(),
                    amount_tax: Deserialize::default(),
                    amount_total: Deserialize::default(),
                    currency: Deserialize::default(),
                    description: Deserialize::default(),
                    discounts: Deserialize::default(),
                    id: Deserialize::default(),
                    metadata: Deserialize::default(),
                    price: Deserialize::default(),
                    quantity: Deserialize::default(),
                    taxes: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "adjustable_quantity" => Deserialize::begin(&mut self.builder.adjustable_quantity),
                "amount_discount" => Deserialize::begin(&mut self.builder.amount_discount),
                "amount_subtotal" => Deserialize::begin(&mut self.builder.amount_subtotal),
                "amount_tax" => Deserialize::begin(&mut self.builder.amount_tax),
                "amount_total" => Deserialize::begin(&mut self.builder.amount_total),
                "currency" => Deserialize::begin(&mut self.builder.currency),
                "description" => Deserialize::begin(&mut self.builder.description),
                "discounts" => Deserialize::begin(&mut self.builder.discounts),
                "id" => Deserialize::begin(&mut self.builder.id),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "price" => Deserialize::begin(&mut self.builder.price),
                "quantity" => Deserialize::begin(&mut self.builder.quantity),
                "taxes" => Deserialize::begin(&mut self.builder.taxes),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(adjustable_quantity),
                Some(amount_discount),
                Some(amount_subtotal),
                Some(amount_tax),
                Some(amount_total),
                Some(currency),
                Some(description),
                Some(discounts),
                Some(id),
                Some(metadata),
                Some(price),
                Some(quantity),
                Some(taxes),
            ) = (
                self.builder.adjustable_quantity,
                self.builder.amount_discount,
                self.builder.amount_subtotal,
                self.builder.amount_tax,
                self.builder.amount_total,
                self.builder.currency.take(),
                self.builder.description.take(),
                self.builder.discounts.take(),
                self.builder.id.take(),
                self.builder.metadata.take(),
                self.builder.price.take(),
                self.builder.quantity,
                self.builder.taxes.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(CheckoutSessionItem {
                adjustable_quantity,
                amount_discount,
                amount_subtotal,
                amount_tax,
                amount_total,
                currency,
                description,
                discounts,
                id,
                metadata,
                price,
                quantity,
                taxes,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for CheckoutSessionItem {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("CheckoutSessionItem", 14)?;
        s.serialize_field("adjustable_quantity", &self.adjustable_quantity)?;
        s.serialize_field("amount_discount", &self.amount_discount)?;
        s.serialize_field("amount_subtotal", &self.amount_subtotal)?;
        s.serialize_field("amount_tax", &self.amount_tax)?;
        s.serialize_field("amount_total", &self.amount_total)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("discounts", &self.discounts)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("metadata", &self.metadata)?;
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
