/// Invoice Line Items represent the individual lines within an [invoice](https://stripe.com/docs/api/invoices) and only exist within the context of an invoice.
///
/// Each line item is backed by either an [invoice item](https://stripe.com/docs/api/invoiceitems) or a [subscription item](https://stripe.com/docs/api/subscription_items).
#[derive(Clone, Debug)]
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
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
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
    /// The quantity of the subscription, if the line item is a subscription or a proration.
    pub quantity: Option<u64>,
    pub subscription: Option<stripe_types::Expandable<stripe_shared::Subscription>>,
    /// The tax information of the line item.
    pub taxes: Option<Vec<stripe_shared::BillingBillResourceInvoicingTaxesTax>>,
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
    subscription: Option<Option<stripe_types::Expandable<stripe_shared::Subscription>>>,
    taxes: Option<Option<Vec<stripe_shared::BillingBillResourceInvoicingTaxesTax>>>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: InvoiceLineItemBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InvoiceLineItemBuilder {
        type Out = InvoiceLineItem;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "currency" => Deserialize::begin(&mut self.currency),
                "description" => Deserialize::begin(&mut self.description),
                "discount_amounts" => Deserialize::begin(&mut self.discount_amounts),
                "discountable" => Deserialize::begin(&mut self.discountable),
                "discounts" => Deserialize::begin(&mut self.discounts),
                "id" => Deserialize::begin(&mut self.id),
                "invoice" => Deserialize::begin(&mut self.invoice),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "parent" => Deserialize::begin(&mut self.parent),
                "period" => Deserialize::begin(&mut self.period),
                "pretax_credit_amounts" => Deserialize::begin(&mut self.pretax_credit_amounts),
                "pricing" => Deserialize::begin(&mut self.pricing),
                "quantity" => Deserialize::begin(&mut self.quantity),
                "subscription" => Deserialize::begin(&mut self.subscription),
                "taxes" => Deserialize::begin(&mut self.taxes),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
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
                subscription: Deserialize::default(),
                taxes: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
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
                Some(subscription),
                Some(taxes),
            ) = (
                self.amount,
                self.currency.take(),
                self.description.take(),
                self.discount_amounts.take(),
                self.discountable,
                self.discounts.take(),
                self.id.take(),
                self.invoice.take(),
                self.livemode,
                self.metadata.take(),
                self.parent.take(),
                self.period,
                self.pretax_credit_amounts.take(),
                self.pricing.take(),
                self.quantity,
                self.subscription.take(),
                self.taxes.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
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
                subscription,
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

    impl ObjectDeser for InvoiceLineItem {
        type Builder = InvoiceLineItemBuilder;
    }

    impl FromValueOpt for InvoiceLineItem {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InvoiceLineItemBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "description" => b.description = FromValueOpt::from_value(v),
                    "discount_amounts" => b.discount_amounts = FromValueOpt::from_value(v),
                    "discountable" => b.discountable = FromValueOpt::from_value(v),
                    "discounts" => b.discounts = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "invoice" => b.invoice = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "parent" => b.parent = FromValueOpt::from_value(v),
                    "period" => b.period = FromValueOpt::from_value(v),
                    "pretax_credit_amounts" => {
                        b.pretax_credit_amounts = FromValueOpt::from_value(v)
                    }
                    "pricing" => b.pricing = FromValueOpt::from_value(v),
                    "quantity" => b.quantity = FromValueOpt::from_value(v),
                    "subscription" => b.subscription = FromValueOpt::from_value(v),
                    "taxes" => b.taxes = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for InvoiceLineItem {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("InvoiceLineItem", 18)?;
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
        s.serialize_field("subscription", &self.subscription)?;
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
