#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxTransactionLineItem {
    /// The line item amount in the [smallest currency unit](https://docs.stripe.com/currencies#minor-units).
    /// If `tax_behavior=inclusive`, then this amount includes taxes.
    /// Otherwise, taxes were calculated on top of this amount.
    pub amount: i64,
    /// The amount of tax calculated for this line item, in the [smallest currency unit](https://docs.stripe.com/currencies#minor-units).
    pub amount_tax: i64,
    /// Unique identifier for the object.
    pub id: stripe_misc::TaxTransactionLineItemId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The ID of an existing [Product](https://docs.stripe.com/api/products/object).
    pub product: Option<String>,
    /// The number of units of the item being purchased. For reversals, this is the quantity reversed.
    pub quantity: u64,
    /// A custom identifier for this line item in the transaction.
    pub reference: String,
    /// If `type=reversal`, contains information about what was reversed.
    pub reversal: Option<stripe_misc::TaxProductResourceTaxTransactionLineItemResourceReversal>,
    /// Specifies whether the `amount` includes taxes.
    /// If `tax_behavior=inclusive`, then the amount includes taxes.
    pub tax_behavior: stripe_shared::TaxBehavior,
    /// The [tax code](https://docs.stripe.com/tax/tax-categories) ID used for this resource.
    pub tax_code: String,
    /// If `reversal`, this line item reverses an earlier transaction.
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: stripe_shared::TaxTransactionType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxTransactionLineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxTransactionLineItem").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TaxTransactionLineItemBuilder {
    amount: Option<i64>,
    amount_tax: Option<i64>,
    id: Option<stripe_misc::TaxTransactionLineItemId>,
    livemode: Option<bool>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    product: Option<Option<String>>,
    quantity: Option<u64>,
    reference: Option<String>,
    reversal: Option<Option<stripe_misc::TaxProductResourceTaxTransactionLineItemResourceReversal>>,
    tax_behavior: Option<stripe_shared::TaxBehavior>,
    tax_code: Option<String>,
    type_: Option<stripe_shared::TaxTransactionType>,
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

    impl Deserialize for TaxTransactionLineItem {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxTransactionLineItem>,
        builder: TaxTransactionLineItemBuilder,
    }

    impl Visitor for Place<TaxTransactionLineItem> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxTransactionLineItemBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TaxTransactionLineItemBuilder {
        type Out = TaxTransactionLineItem;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "amount_tax" => Deserialize::begin(&mut self.amount_tax),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "product" => Deserialize::begin(&mut self.product),
                "quantity" => Deserialize::begin(&mut self.quantity),
                "reference" => Deserialize::begin(&mut self.reference),
                "reversal" => Deserialize::begin(&mut self.reversal),
                "tax_behavior" => Deserialize::begin(&mut self.tax_behavior),
                "tax_code" => Deserialize::begin(&mut self.tax_code),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: None,
                amount_tax: None,
                id: None,
                livemode: None,
                metadata: Some(None),
                product: Some(None),
                quantity: None,
                reference: None,
                reversal: Some(None),
                tax_behavior: None,
                tax_code: None,
                type_: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(amount),
                Some(amount_tax),
                Some(id),
                Some(livemode),
                Some(metadata),
                Some(product),
                Some(quantity),
                Some(reference),
                Some(reversal),
                Some(tax_behavior),
                Some(tax_code),
                Some(type_),
            ) = (
                self.amount,
                self.amount_tax,
                self.id.take(),
                self.livemode,
                self.metadata.take(),
                self.product.take(),
                self.quantity,
                self.reference.take(),
                self.reversal.take(),
                self.tax_behavior.take(),
                self.tax_code.take(),
                self.type_.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                amount,
                amount_tax,
                id,
                livemode,
                metadata,
                product,
                quantity,
                reference,
                reversal,
                tax_behavior,
                tax_code,
                type_,
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for TaxTransactionLineItem {
        type Builder = TaxTransactionLineItemBuilder;
    }

    impl FromValueOpt for TaxTransactionLineItem {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxTransactionLineItemBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "amount_tax" => b.amount_tax = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "product" => b.product = FromValueOpt::from_value(v),
                    "quantity" => b.quantity = FromValueOpt::from_value(v),
                    "reference" => b.reference = FromValueOpt::from_value(v),
                    "reversal" => b.reversal = FromValueOpt::from_value(v),
                    "tax_behavior" => b.tax_behavior = FromValueOpt::from_value(v),
                    "tax_code" => b.tax_code = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxTransactionLineItem {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("TaxTransactionLineItem", 13)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("amount_tax", &self.amount_tax)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("product", &self.product)?;
        s.serialize_field("quantity", &self.quantity)?;
        s.serialize_field("reference", &self.reference)?;
        s.serialize_field("reversal", &self.reversal)?;
        s.serialize_field("tax_behavior", &self.tax_behavior)?;
        s.serialize_field("tax_code", &self.tax_code)?;
        s.serialize_field("type", &self.type_)?;

        s.serialize_field("object", "tax.transaction_line_item")?;
        s.end()
    }
}
impl stripe_types::Object for TaxTransactionLineItem {
    type Id = stripe_misc::TaxTransactionLineItemId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(TaxTransactionLineItemId);
/// Specifies whether the `amount` includes taxes.
/// If `tax_behavior=inclusive`, then the amount includes taxes.
pub type TaxTransactionLineItemTaxBehavior = stripe_shared::TaxBehavior;
/// If `reversal`, this line item reverses an earlier transaction.
pub type TaxTransactionLineItemType = stripe_shared::TaxTransactionType;
