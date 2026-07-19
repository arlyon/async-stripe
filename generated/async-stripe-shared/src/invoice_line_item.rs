/// Invoice Line Items represent the individual lines within an [invoice](https://docs.stripe.com/api/invoices) and only exist within the context of an invoice.
///
/// Each line item is backed by either an [invoice item](https://docs.stripe.com/api/invoiceitems) or a [subscription item](https://docs.stripe.com/api/subscription_items).
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoiceLineItem {
    /// The amount, in cents (or local equivalent).
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    /// The amount of discount calculated per discount for this line item.
    pub discount_amounts: Option<Vec<stripe_shared::DiscountsResourceDiscountAmount>>,
    /// If true, discounts will apply to this line item. Always false for prorations.
    pub discountable: bool,
    /// The discounts applied to the invoice line item.
    /// Line item discounts are applied before invoice discounts.
    /// Use `expand[]=discounts` to expand each discount.
    pub discounts: Vec<stripe_types::Expandable<stripe_shared::Discount>>,
    /// Unique identifier for the object.
    pub id: stripe_shared::InvoiceLineItemId,
    /// The ID of the invoice that contains this line item.
    pub invoice: Option<String>,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Note that for line items with `type=subscription`, `metadata` reflects the current metadata from the subscription associated with the line item, unless the invoice line was directly updated with different metadata after creation.
    pub metadata: std::collections::HashMap<String, String>,
    /// The parent that generated this line item.
    pub parent:
        Option<stripe_shared::BillingBillResourceInvoicingLinesParentsInvoiceLineItemParent>,
    pub period: stripe_shared::InvoiceLineItemPeriod,
    /// Contains pretax credit amounts (ex: discount, credit grants, etc) that apply to this line item.
    pub pretax_credit_amounts: Option<Vec<stripe_shared::InvoicesResourcePretaxCreditAmount>>,
    /// The pricing information of the line item.
    pub pricing: Option<stripe_shared::BillingBillResourceInvoicingPricingPricing>,
    /// Quantity of units for the invoice line item in integer format, with any decimal precision truncated.
    /// For the line item's full-precision decimal quantity, use `quantity_decimal`.
    /// This field will be deprecated in favor of `quantity_decimal` in a future version.
    /// If the line item is a proration or subscription, the quantity of the subscription that the proration was computed for.
    pub quantity: Option<u64>,
    /// Non-negative decimal with at most 12 decimal places. The quantity of units for the line item.
    pub quantity_decimal: Option<String>,
    pub subscription: Option<stripe_types::Expandable<stripe_shared::Subscription>>,
    /// The subtotal of the line item, in cents (or local equivalent), before any discounts or taxes.
    pub subtotal: i64,
    /// The tax information of the line item.
    pub taxes: Option<Vec<stripe_shared::BillingBillResourceInvoicingTaxesTax>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoiceLineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoiceLineItem").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InvoiceLineItemBuilder {
    amount: Option<i64>,
    currency: Option<stripe_types::Currency>,
    description: Option<Option<String>>,
    discount_amounts: Option<Option<Vec<stripe_shared::DiscountsResourceDiscountAmount>>>,
    discountable: Option<bool>,
    discounts: Option<Vec<stripe_types::Expandable<stripe_shared::Discount>>>,
    id: Option<stripe_shared::InvoiceLineItemId>,
    invoice: Option<Option<String>>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    parent: Option<
        Option<stripe_shared::BillingBillResourceInvoicingLinesParentsInvoiceLineItemParent>,
    >,
    period: Option<stripe_shared::InvoiceLineItemPeriod>,
    pretax_credit_amounts: Option<Option<Vec<stripe_shared::InvoicesResourcePretaxCreditAmount>>>,
    pricing: Option<Option<stripe_shared::BillingBillResourceInvoicingPricingPricing>>,
    quantity: Option<Option<u64>>,
    quantity_decimal: Option<Option<String>>,
    subscription: Option<Option<stripe_types::Expandable<stripe_shared::Subscription>>>,
    subtotal: Option<i64>,
    taxes: Option<Option<Vec<stripe_shared::BillingBillResourceInvoicingTaxesTax>>>,
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

    impl Deserialize for InvoiceLineItem {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoiceLineItem>,
        builder: InvoiceLineItemBuilder,
    }

    impl Visitor for Place<InvoiceLineItem> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoiceLineItemBuilder {
                    amount: Deserialize::default(),
                    currency: Deserialize::default(),
                    description: Deserialize::default(),
                    discount_amounts: Deserialize::default(),
                    discountable: Deserialize::default(),
                    discounts: Deserialize::default(),
                    id: Deserialize::default(),
                    invoice: Deserialize::default(),
                    livemode: Deserialize::default(),
                    metadata: Deserialize::default(),
                    parent: Deserialize::default(),
                    period: Deserialize::default(),
                    pretax_credit_amounts: Deserialize::default(),
                    pricing: Deserialize::default(),
                    quantity: Deserialize::default(),
                    quantity_decimal: Deserialize::default(),
                    subscription: Deserialize::default(),
                    subtotal: Deserialize::default(),
                    taxes: Deserialize::default(),
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
                "discount_amounts" => Deserialize::begin(&mut self.builder.discount_amounts),
                "discountable" => Deserialize::begin(&mut self.builder.discountable),
                "discounts" => Deserialize::begin(&mut self.builder.discounts),
                "id" => Deserialize::begin(&mut self.builder.id),
                "invoice" => Deserialize::begin(&mut self.builder.invoice),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "parent" => Deserialize::begin(&mut self.builder.parent),
                "period" => Deserialize::begin(&mut self.builder.period),
                "pretax_credit_amounts" => {
                    Deserialize::begin(&mut self.builder.pretax_credit_amounts)
                }
                "pricing" => Deserialize::begin(&mut self.builder.pricing),
                "quantity" => Deserialize::begin(&mut self.builder.quantity),
                "quantity_decimal" => Deserialize::begin(&mut self.builder.quantity_decimal),
                "subscription" => Deserialize::begin(&mut self.builder.subscription),
                "subtotal" => Deserialize::begin(&mut self.builder.subtotal),
                "taxes" => Deserialize::begin(&mut self.builder.taxes),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(amount),
                Some(currency),
                Some(description),
                Some(discount_amounts),
                Some(discountable),
                Some(discounts),
                Some(id),
                Some(invoice),
                Some(livemode),
                Some(metadata),
                Some(parent),
                Some(period),
                Some(pretax_credit_amounts),
                Some(pricing),
                Some(quantity),
                Some(quantity_decimal),
                Some(subscription),
                Some(subtotal),
                Some(taxes),
            ) = (
                self.builder.amount,
                self.builder.currency.take(),
                self.builder.description.take(),
                self.builder.discount_amounts.take(),
                self.builder.discountable,
                self.builder.discounts.take(),
                self.builder.id.take(),
                self.builder.invoice.take(),
                self.builder.livemode,
                self.builder.metadata.take(),
                self.builder.parent.take(),
                self.builder.period,
                self.builder.pretax_credit_amounts.take(),
                self.builder.pricing.take(),
                self.builder.quantity,
                self.builder.quantity_decimal.take(),
                self.builder.subscription.take(),
                self.builder.subtotal,
                self.builder.taxes.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(InvoiceLineItem {
                amount,
                currency,
                description,
                discount_amounts,
                discountable,
                discounts,
                id,
                invoice,
                livemode,
                metadata,
                parent,
                period,
                pretax_credit_amounts,
                pricing,
                quantity,
                quantity_decimal,
                subscription,
                subtotal,
                taxes,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for InvoiceLineItem {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("InvoiceLineItem", 20)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("discount_amounts", &self.discount_amounts)?;
        s.serialize_field("discountable", &self.discountable)?;
        s.serialize_field("discounts", &self.discounts)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("invoice", &self.invoice)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("parent", &self.parent)?;
        s.serialize_field("period", &self.period)?;
        s.serialize_field("pretax_credit_amounts", &self.pretax_credit_amounts)?;
        s.serialize_field("pricing", &self.pricing)?;
        s.serialize_field("quantity", &self.quantity)?;
        s.serialize_field("quantity_decimal", &self.quantity_decimal)?;
        s.serialize_field("subscription", &self.subscription)?;
        s.serialize_field("subtotal", &self.subtotal)?;
        s.serialize_field("taxes", &self.taxes)?;

        s.serialize_field("object", "line_item")?;
        s.end()
    }
}
impl stripe_types::Object for InvoiceLineItem {
    type Id = stripe_shared::InvoiceLineItemId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(InvoiceLineItemId);
