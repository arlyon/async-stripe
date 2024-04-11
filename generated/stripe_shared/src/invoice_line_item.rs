#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoiceLineItem {
    /// The amount, in cents (or local equivalent).
    pub amount: i64,
    /// The integer amount in cents (or local equivalent) representing the amount for this line item, excluding all tax and discounts.
    pub amount_excluding_tax: Option<i64>,
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
    pub discounts: Option<Vec<stripe_types::Expandable<stripe_shared::Discount>>>,
    /// Unique identifier for the object.
    pub id: stripe_shared::InvoiceLineItemId,
    /// The ID of the [invoice item](https://stripe.com/docs/api/invoiceitems) associated with this line item if any.
    pub invoice_item: Option<stripe_types::Expandable<stripe_shared::InvoiceItem>>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Note that for line items with `type=subscription` this will reflect the metadata of the subscription that caused the line item to be created.
    pub metadata: std::collections::HashMap<String, String>,
    pub period: stripe_shared::InvoiceLineItemPeriod,
    /// The plan of the subscription, if the line item is a subscription or a proration.
    pub plan: Option<stripe_shared::Plan>,
    /// The price of the line item.
    pub price: Option<stripe_shared::Price>,
    /// Whether this is a proration.
    pub proration: bool,
    /// Additional details for proration line items
    pub proration_details: Option<stripe_shared::InvoicesResourceLineItemsProrationDetails>,
    /// The quantity of the subscription, if the line item is a subscription or a proration.
    pub quantity: Option<u64>,
    /// The subscription that the invoice item pertains to, if any.
    pub subscription: Option<stripe_types::Expandable<stripe_shared::Subscription>>,
    /// The subscription item that generated this line item.
    /// Left empty if the line item is not an explicit result of a subscription.
    pub subscription_item: Option<stripe_types::Expandable<stripe_shared::SubscriptionItem>>,
    /// The amount of tax calculated per tax rate for this line item
    pub tax_amounts: Option<Vec<stripe_shared::InvoiceTaxAmount>>,
    /// The tax rates which apply to the line item.
    pub tax_rates: Option<Vec<stripe_shared::TaxRate>>,
    /// A string identifying the type of the source of this line item, either an `invoiceitem` or a `subscription`.
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: InvoiceLineItemType,
    /// The amount in cents (or local equivalent) representing the unit amount for this line item, excluding all tax and discounts.
    pub unit_amount_excluding_tax: Option<String>,
}
#[doc(hidden)]
pub struct InvoiceLineItemBuilder {
    amount: Option<i64>,
    amount_excluding_tax: Option<Option<i64>>,
    currency: Option<stripe_types::Currency>,
    description: Option<Option<String>>,
    discount_amounts: Option<Option<Vec<stripe_shared::DiscountsResourceDiscountAmount>>>,
    discountable: Option<bool>,
    discounts: Option<Option<Vec<stripe_types::Expandable<stripe_shared::Discount>>>>,
    id: Option<stripe_shared::InvoiceLineItemId>,
    invoice_item: Option<Option<stripe_types::Expandable<stripe_shared::InvoiceItem>>>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    period: Option<stripe_shared::InvoiceLineItemPeriod>,
    plan: Option<Option<stripe_shared::Plan>>,
    price: Option<Option<stripe_shared::Price>>,
    proration: Option<bool>,
    proration_details: Option<Option<stripe_shared::InvoicesResourceLineItemsProrationDetails>>,
    quantity: Option<Option<u64>>,
    subscription: Option<Option<stripe_types::Expandable<stripe_shared::Subscription>>>,
    subscription_item: Option<Option<stripe_types::Expandable<stripe_shared::SubscriptionItem>>>,
    tax_amounts: Option<Option<Vec<stripe_shared::InvoiceTaxAmount>>>,
    tax_rates: Option<Option<Vec<stripe_shared::TaxRate>>>,
    type_: Option<InvoiceLineItemType>,
    unit_amount_excluding_tax: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
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
                "amount_excluding_tax" => Deserialize::begin(&mut self.amount_excluding_tax),
                "currency" => Deserialize::begin(&mut self.currency),
                "description" => Deserialize::begin(&mut self.description),
                "discount_amounts" => Deserialize::begin(&mut self.discount_amounts),
                "discountable" => Deserialize::begin(&mut self.discountable),
                "discounts" => Deserialize::begin(&mut self.discounts),
                "id" => Deserialize::begin(&mut self.id),
                "invoice_item" => Deserialize::begin(&mut self.invoice_item),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "period" => Deserialize::begin(&mut self.period),
                "plan" => Deserialize::begin(&mut self.plan),
                "price" => Deserialize::begin(&mut self.price),
                "proration" => Deserialize::begin(&mut self.proration),
                "proration_details" => Deserialize::begin(&mut self.proration_details),
                "quantity" => Deserialize::begin(&mut self.quantity),
                "subscription" => Deserialize::begin(&mut self.subscription),
                "subscription_item" => Deserialize::begin(&mut self.subscription_item),
                "tax_amounts" => Deserialize::begin(&mut self.tax_amounts),
                "tax_rates" => Deserialize::begin(&mut self.tax_rates),
                "type" => Deserialize::begin(&mut self.type_),
                "unit_amount_excluding_tax" => {
                    Deserialize::begin(&mut self.unit_amount_excluding_tax)
                }

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                amount_excluding_tax: Deserialize::default(),
                currency: Deserialize::default(),
                description: Deserialize::default(),
                discount_amounts: Deserialize::default(),
                discountable: Deserialize::default(),
                discounts: Deserialize::default(),
                id: Deserialize::default(),
                invoice_item: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                period: Deserialize::default(),
                plan: Deserialize::default(),
                price: Deserialize::default(),
                proration: Deserialize::default(),
                proration_details: Deserialize::default(),
                quantity: Deserialize::default(),
                subscription: Deserialize::default(),
                subscription_item: Deserialize::default(),
                tax_amounts: Deserialize::default(),
                tax_rates: Deserialize::default(),
                type_: Deserialize::default(),
                unit_amount_excluding_tax: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                amount: self.amount?,
                amount_excluding_tax: self.amount_excluding_tax?,
                currency: self.currency?,
                description: self.description.take()?,
                discount_amounts: self.discount_amounts.take()?,
                discountable: self.discountable?,
                discounts: self.discounts.take()?,
                id: self.id.take()?,
                invoice_item: self.invoice_item.take()?,
                livemode: self.livemode?,
                metadata: self.metadata.take()?,
                period: self.period?,
                plan: self.plan.take()?,
                price: self.price.take()?,
                proration: self.proration?,
                proration_details: self.proration_details.take()?,
                quantity: self.quantity?,
                subscription: self.subscription.take()?,
                subscription_item: self.subscription_item.take()?,
                tax_amounts: self.tax_amounts.take()?,
                tax_rates: self.tax_rates.take()?,
                type_: self.type_?,
                unit_amount_excluding_tax: self.unit_amount_excluding_tax.take()?,
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
                    "amount" => b.amount = Some(FromValueOpt::from_value(v)?),
                    "amount_excluding_tax" => {
                        b.amount_excluding_tax = Some(FromValueOpt::from_value(v)?)
                    }
                    "currency" => b.currency = Some(FromValueOpt::from_value(v)?),
                    "description" => b.description = Some(FromValueOpt::from_value(v)?),
                    "discount_amounts" => b.discount_amounts = Some(FromValueOpt::from_value(v)?),
                    "discountable" => b.discountable = Some(FromValueOpt::from_value(v)?),
                    "discounts" => b.discounts = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "invoice_item" => b.invoice_item = Some(FromValueOpt::from_value(v)?),
                    "livemode" => b.livemode = Some(FromValueOpt::from_value(v)?),
                    "metadata" => b.metadata = Some(FromValueOpt::from_value(v)?),
                    "period" => b.period = Some(FromValueOpt::from_value(v)?),
                    "plan" => b.plan = Some(FromValueOpt::from_value(v)?),
                    "price" => b.price = Some(FromValueOpt::from_value(v)?),
                    "proration" => b.proration = Some(FromValueOpt::from_value(v)?),
                    "proration_details" => b.proration_details = Some(FromValueOpt::from_value(v)?),
                    "quantity" => b.quantity = Some(FromValueOpt::from_value(v)?),
                    "subscription" => b.subscription = Some(FromValueOpt::from_value(v)?),
                    "subscription_item" => b.subscription_item = Some(FromValueOpt::from_value(v)?),
                    "tax_amounts" => b.tax_amounts = Some(FromValueOpt::from_value(v)?),
                    "tax_rates" => b.tax_rates = Some(FromValueOpt::from_value(v)?),
                    "type" => b.type_ = Some(FromValueOpt::from_value(v)?),
                    "unit_amount_excluding_tax" => {
                        b.unit_amount_excluding_tax = Some(FromValueOpt::from_value(v)?)
                    }

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
        let mut s = s.serialize_struct("InvoiceLineItem", 24)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("amount_excluding_tax", &self.amount_excluding_tax)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("discount_amounts", &self.discount_amounts)?;
        s.serialize_field("discountable", &self.discountable)?;
        s.serialize_field("discounts", &self.discounts)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("invoice_item", &self.invoice_item)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("period", &self.period)?;
        s.serialize_field("plan", &self.plan)?;
        s.serialize_field("price", &self.price)?;
        s.serialize_field("proration", &self.proration)?;
        s.serialize_field("proration_details", &self.proration_details)?;
        s.serialize_field("quantity", &self.quantity)?;
        s.serialize_field("subscription", &self.subscription)?;
        s.serialize_field("subscription_item", &self.subscription_item)?;
        s.serialize_field("tax_amounts", &self.tax_amounts)?;
        s.serialize_field("tax_rates", &self.tax_rates)?;
        s.serialize_field("type", &self.type_)?;
        s.serialize_field("unit_amount_excluding_tax", &self.unit_amount_excluding_tax)?;

        s.serialize_field("object", "line_item")?;
        s.end()
    }
}
/// A string identifying the type of the source of this line item, either an `invoiceitem` or a `subscription`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InvoiceLineItemType {
    Invoiceitem,
    Subscription,
}
impl InvoiceLineItemType {
    pub fn as_str(self) -> &'static str {
        use InvoiceLineItemType::*;
        match self {
            Invoiceitem => "invoiceitem",
            Subscription => "subscription",
        }
    }
}

impl std::str::FromStr for InvoiceLineItemType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoiceLineItemType::*;
        match s {
            "invoiceitem" => Ok(Invoiceitem),
            "subscription" => Ok(Subscription),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for InvoiceLineItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InvoiceLineItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InvoiceLineItemType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for InvoiceLineItemType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<InvoiceLineItemType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(InvoiceLineItemType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(InvoiceLineItemType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InvoiceLineItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for InvoiceLineItemType"))
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
