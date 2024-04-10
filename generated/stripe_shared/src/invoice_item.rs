/// Invoice Items represent the component lines of an [invoice](https://stripe.com/docs/api/invoices).
/// An invoice item is added to an.
/// invoice by creating or updating it with an `invoice` field, at which point it will be included as
/// [an invoice line item](https://stripe.com/docs/api/invoices/line_item) within
/// [invoice.lines](https://stripe.com/docs/api/invoices/object#invoice_object-lines).
///
/// Invoice Items can be created before you are ready to actually send the invoice.
/// This can be particularly useful when combined.
/// with a [subscription](https://stripe.com/docs/api/subscriptions).
/// Sometimes you want to add a charge or credit to a customer, but actually charge.
/// or credit the customer’s card only at the end of a regular billing cycle.
/// This is useful for combining several charges.
/// (to minimize per-transaction fees), or for having Stripe tabulate your usage-based billing totals.
///
/// Related guides: [Integrate with the Invoicing API](https://stripe.com/docs/invoicing/integration), [Subscription Invoices](https://stripe.com/docs/billing/invoices/subscription#adding-upcoming-invoice-items).
///
/// For more details see <<https://stripe.com/docs/api/invoiceitems/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoiceItem {
    /// Amount (in the `currency` specified) of the invoice item.
    /// This should always be equal to `unit_amount * quantity`.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the customer who will be billed when this invoice item is billed.
    pub customer: stripe_types::Expandable<stripe_shared::Customer>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub date: stripe_types::Timestamp,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    /// If true, discounts will apply to this invoice item. Always false for prorations.
    pub discountable: bool,
    /// The discounts which apply to the invoice item.
    /// Item discounts are applied before invoice discounts.
    /// Use `expand[]=discounts` to expand each discount.
    pub discounts: Option<Vec<stripe_types::Expandable<stripe_shared::Discount>>>,
    /// Unique identifier for the object.
    pub id: stripe_shared::InvoiceItemId,
    /// The ID of the invoice this invoice item belongs to.
    pub invoice: Option<stripe_types::Expandable<stripe_shared::Invoice>>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    pub period: stripe_shared::InvoiceLineItemPeriod,
    /// If the invoice item is a proration, the plan of the subscription that the proration was computed for.
    pub plan: Option<stripe_shared::Plan>,
    /// The price of the invoice item.
    pub price: Option<stripe_shared::Price>,
    /// Whether the invoice item was created automatically as a proration adjustment when the customer switched plans.
    pub proration: bool,
    /// Quantity of units for the invoice item.
    /// If the invoice item is a proration, the quantity of the subscription that the proration was computed for.
    pub quantity: u64,
    /// The subscription that this invoice item has been created for, if any.
    pub subscription: Option<stripe_types::Expandable<stripe_shared::Subscription>>,
    /// The subscription item that this invoice item has been created for, if any.
    pub subscription_item: Option<String>,
    /// The tax rates which apply to the invoice item.
    /// When set, the `default_tax_rates` on the invoice do not apply to this invoice item.
    pub tax_rates: Option<Vec<stripe_shared::TaxRate>>,
    /// ID of the test clock this invoice item belongs to.
    pub test_clock: Option<stripe_types::Expandable<stripe_shared::TestHelpersTestClock>>,
    /// Unit amount (in the `currency` specified) of the invoice item.
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but contains a decimal value with at most 12 decimal places.
    pub unit_amount_decimal: Option<String>,
}
#[doc(hidden)]
pub struct InvoiceItemBuilder {
    amount: Option<i64>,
    currency: Option<stripe_types::Currency>,
    customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    date: Option<stripe_types::Timestamp>,
    description: Option<Option<String>>,
    discountable: Option<bool>,
    discounts: Option<Option<Vec<stripe_types::Expandable<stripe_shared::Discount>>>>,
    id: Option<stripe_shared::InvoiceItemId>,
    invoice: Option<Option<stripe_types::Expandable<stripe_shared::Invoice>>>,
    livemode: Option<bool>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    period: Option<stripe_shared::InvoiceLineItemPeriod>,
    plan: Option<Option<stripe_shared::Plan>>,
    price: Option<Option<stripe_shared::Price>>,
    proration: Option<bool>,
    quantity: Option<u64>,
    subscription: Option<Option<stripe_types::Expandable<stripe_shared::Subscription>>>,
    subscription_item: Option<Option<String>>,
    tax_rates: Option<Option<Vec<stripe_shared::TaxRate>>>,
    test_clock: Option<Option<stripe_types::Expandable<stripe_shared::TestHelpersTestClock>>>,
    unit_amount: Option<Option<i64>>,
    unit_amount_decimal: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for InvoiceItem {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoiceItem>,
        builder: InvoiceItemBuilder,
    }

    impl Visitor for Place<InvoiceItem> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoiceItemBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InvoiceItemBuilder {
        type Out = InvoiceItem;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "currency" => Deserialize::begin(&mut self.currency),
                "customer" => Deserialize::begin(&mut self.customer),
                "date" => Deserialize::begin(&mut self.date),
                "description" => Deserialize::begin(&mut self.description),
                "discountable" => Deserialize::begin(&mut self.discountable),
                "discounts" => Deserialize::begin(&mut self.discounts),
                "id" => Deserialize::begin(&mut self.id),
                "invoice" => Deserialize::begin(&mut self.invoice),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "period" => Deserialize::begin(&mut self.period),
                "plan" => Deserialize::begin(&mut self.plan),
                "price" => Deserialize::begin(&mut self.price),
                "proration" => Deserialize::begin(&mut self.proration),
                "quantity" => Deserialize::begin(&mut self.quantity),
                "subscription" => Deserialize::begin(&mut self.subscription),
                "subscription_item" => Deserialize::begin(&mut self.subscription_item),
                "tax_rates" => Deserialize::begin(&mut self.tax_rates),
                "test_clock" => Deserialize::begin(&mut self.test_clock),
                "unit_amount" => Deserialize::begin(&mut self.unit_amount),
                "unit_amount_decimal" => Deserialize::begin(&mut self.unit_amount_decimal),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                currency: Deserialize::default(),
                customer: Deserialize::default(),
                date: Deserialize::default(),
                description: Deserialize::default(),
                discountable: Deserialize::default(),
                discounts: Deserialize::default(),
                id: Deserialize::default(),
                invoice: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                period: Deserialize::default(),
                plan: Deserialize::default(),
                price: Deserialize::default(),
                proration: Deserialize::default(),
                quantity: Deserialize::default(),
                subscription: Deserialize::default(),
                subscription_item: Deserialize::default(),
                tax_rates: Deserialize::default(),
                test_clock: Deserialize::default(),
                unit_amount: Deserialize::default(),
                unit_amount_decimal: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                amount: self.amount?,
                currency: self.currency?,
                customer: self.customer.take()?,
                date: self.date?,
                description: self.description.take()?,
                discountable: self.discountable?,
                discounts: self.discounts.take()?,
                id: self.id.take()?,
                invoice: self.invoice.take()?,
                livemode: self.livemode?,
                metadata: self.metadata.take()?,
                period: self.period?,
                plan: self.plan.take()?,
                price: self.price.take()?,
                proration: self.proration?,
                quantity: self.quantity?,
                subscription: self.subscription.take()?,
                subscription_item: self.subscription_item.take()?,
                tax_rates: self.tax_rates.take()?,
                test_clock: self.test_clock.take()?,
                unit_amount: self.unit_amount?,
                unit_amount_decimal: self.unit_amount_decimal.take()?,
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

    impl ObjectDeser for InvoiceItem {
        type Builder = InvoiceItemBuilder;
    }

    impl FromValueOpt for InvoiceItem {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InvoiceItemBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = Some(FromValueOpt::from_value(v)?),
                    "currency" => b.currency = Some(FromValueOpt::from_value(v)?),
                    "customer" => b.customer = Some(FromValueOpt::from_value(v)?),
                    "date" => b.date = Some(FromValueOpt::from_value(v)?),
                    "description" => b.description = Some(FromValueOpt::from_value(v)?),
                    "discountable" => b.discountable = Some(FromValueOpt::from_value(v)?),
                    "discounts" => b.discounts = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "invoice" => b.invoice = Some(FromValueOpt::from_value(v)?),
                    "livemode" => b.livemode = Some(FromValueOpt::from_value(v)?),
                    "metadata" => b.metadata = Some(FromValueOpt::from_value(v)?),
                    "period" => b.period = Some(FromValueOpt::from_value(v)?),
                    "plan" => b.plan = Some(FromValueOpt::from_value(v)?),
                    "price" => b.price = Some(FromValueOpt::from_value(v)?),
                    "proration" => b.proration = Some(FromValueOpt::from_value(v)?),
                    "quantity" => b.quantity = Some(FromValueOpt::from_value(v)?),
                    "subscription" => b.subscription = Some(FromValueOpt::from_value(v)?),
                    "subscription_item" => b.subscription_item = Some(FromValueOpt::from_value(v)?),
                    "tax_rates" => b.tax_rates = Some(FromValueOpt::from_value(v)?),
                    "test_clock" => b.test_clock = Some(FromValueOpt::from_value(v)?),
                    "unit_amount" => b.unit_amount = Some(FromValueOpt::from_value(v)?),
                    "unit_amount_decimal" => {
                        b.unit_amount_decimal = Some(FromValueOpt::from_value(v)?)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for InvoiceItem {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("InvoiceItem", 23)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("customer", &self.customer)?;
        s.serialize_field("date", &self.date)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("discountable", &self.discountable)?;
        s.serialize_field("discounts", &self.discounts)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("invoice", &self.invoice)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("period", &self.period)?;
        s.serialize_field("plan", &self.plan)?;
        s.serialize_field("price", &self.price)?;
        s.serialize_field("proration", &self.proration)?;
        s.serialize_field("quantity", &self.quantity)?;
        s.serialize_field("subscription", &self.subscription)?;
        s.serialize_field("subscription_item", &self.subscription_item)?;
        s.serialize_field("tax_rates", &self.tax_rates)?;
        s.serialize_field("test_clock", &self.test_clock)?;
        s.serialize_field("unit_amount", &self.unit_amount)?;
        s.serialize_field("unit_amount_decimal", &self.unit_amount_decimal)?;

        s.serialize_field("object", "invoiceitem")?;
        s.end()
    }
}
impl stripe_types::Object for InvoiceItem {
    type Id = stripe_shared::InvoiceItemId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(InvoiceItemId);
