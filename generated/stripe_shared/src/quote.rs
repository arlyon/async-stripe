/// A Quote is a way to model prices that you'd like to provide to a customer.
/// Once accepted, it will automatically create an invoice, subscription or subscription schedule.
///
/// For more details see <<https://stripe.com/docs/api/quotes/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Quote {
    /// Total before any discounts or taxes are applied.
    pub amount_subtotal: i64,
    /// Total after discounts and taxes are applied.
    pub amount_total: i64,
    /// ID of the Connect Application that created the quote.
    pub application: Option<stripe_types::Expandable<stripe_shared::Application>>,
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    /// Only applicable if there are no line items with recurring prices on the quote.
    pub application_fee_amount: Option<i64>,
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    /// Only applicable if there are line items with recurring prices on the quote.
    pub application_fee_percent: Option<f64>,
    pub automatic_tax: stripe_shared::QuotesResourceAutomaticTax,
    /// Either `charge_automatically`, or `send_invoice`.
    /// When charging automatically, Stripe will attempt to pay invoices at the end of the subscription cycle or on finalization using the default payment method attached to the subscription or customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    /// Defaults to `charge_automatically`.
    pub collection_method: stripe_shared::QuoteCollectionMethod,
    pub computed: stripe_shared::QuotesResourceComputed,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Option<stripe_types::Currency>,
    /// The customer which this quote belongs to.
    /// A customer is required before finalizing the quote.
    /// Once specified, it cannot be changed.
    pub customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    /// The tax rates applied to this quote.
    pub default_tax_rates: Option<Vec<stripe_types::Expandable<stripe_shared::TaxRate>>>,
    /// A description that will be displayed on the quote PDF.
    pub description: Option<String>,
    /// The discounts applied to this quote.
    pub discounts: Vec<stripe_types::Expandable<stripe_shared::Discount>>,
    /// The date on which the quote will be canceled if in `open` or `draft` status.
    /// Measured in seconds since the Unix epoch.
    pub expires_at: stripe_types::Timestamp,
    /// A footer that will be displayed on the quote PDF.
    pub footer: Option<String>,
    /// Details of the quote that was cloned.
    /// See the [cloning documentation](https://stripe.com/docs/quotes/clone) for more details.
    pub from_quote: Option<stripe_shared::QuotesResourceFromQuote>,
    /// A header that will be displayed on the quote PDF.
    pub header: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_shared::QuoteId,
    /// The invoice that was created from this quote.
    pub invoice: Option<stripe_types::Expandable<stripe_shared::Invoice>>,
    pub invoice_settings: stripe_shared::InvoiceSettingQuoteSetting,
    /// A list of items the customer is being quoted for.
    pub line_items: Option<stripe_types::List<stripe_shared::CheckoutSessionItem>>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// A unique number that identifies this particular quote.
    /// This number is assigned once the quote is [finalized](https://stripe.com/docs/quotes/overview#finalize).
    pub number: Option<String>,
    /// The account on behalf of which to charge.
    /// See the [Connect documentation](https://support.stripe.com/questions/sending-invoices-on-behalf-of-connected-accounts) for details.
    pub on_behalf_of: Option<stripe_types::Expandable<stripe_shared::Account>>,
    /// The status of the quote.
    pub status: stripe_shared::QuoteStatus,
    pub status_transitions: stripe_shared::QuotesResourceStatusTransitions,
    /// The subscription that was created or updated from this quote.
    pub subscription: Option<stripe_types::Expandable<stripe_shared::Subscription>>,
    pub subscription_data: stripe_shared::QuotesResourceSubscriptionDataSubscriptionData,
    /// The subscription schedule that was created or updated from this quote.
    pub subscription_schedule:
        Option<stripe_types::Expandable<stripe_shared::SubscriptionSchedule>>,
    /// ID of the test clock this quote belongs to.
    pub test_clock: Option<stripe_types::Expandable<stripe_shared::TestHelpersTestClock>>,
    pub total_details: stripe_shared::QuotesResourceTotalDetails,
    /// The account (if any) the payments will be attributed to for tax reporting, and where funds from each payment will be transferred to for each of the invoices.
    pub transfer_data: Option<stripe_shared::QuotesResourceTransferData>,
}
#[doc(hidden)]
pub struct QuoteBuilder {
    amount_subtotal: Option<i64>,
    amount_total: Option<i64>,
    application: Option<Option<stripe_types::Expandable<stripe_shared::Application>>>,
    application_fee_amount: Option<Option<i64>>,
    application_fee_percent: Option<Option<f64>>,
    automatic_tax: Option<stripe_shared::QuotesResourceAutomaticTax>,
    collection_method: Option<stripe_shared::QuoteCollectionMethod>,
    computed: Option<stripe_shared::QuotesResourceComputed>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<Option<stripe_types::Currency>>,
    customer: Option<Option<stripe_types::Expandable<stripe_shared::Customer>>>,
    default_tax_rates: Option<Option<Vec<stripe_types::Expandable<stripe_shared::TaxRate>>>>,
    description: Option<Option<String>>,
    discounts: Option<Vec<stripe_types::Expandable<stripe_shared::Discount>>>,
    expires_at: Option<stripe_types::Timestamp>,
    footer: Option<Option<String>>,
    from_quote: Option<Option<stripe_shared::QuotesResourceFromQuote>>,
    header: Option<Option<String>>,
    id: Option<stripe_shared::QuoteId>,
    invoice: Option<Option<stripe_types::Expandable<stripe_shared::Invoice>>>,
    invoice_settings: Option<stripe_shared::InvoiceSettingQuoteSetting>,
    line_items: Option<Option<stripe_types::List<stripe_shared::CheckoutSessionItem>>>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    number: Option<Option<String>>,
    on_behalf_of: Option<Option<stripe_types::Expandable<stripe_shared::Account>>>,
    status: Option<stripe_shared::QuoteStatus>,
    status_transitions: Option<stripe_shared::QuotesResourceStatusTransitions>,
    subscription: Option<Option<stripe_types::Expandable<stripe_shared::Subscription>>>,
    subscription_data: Option<stripe_shared::QuotesResourceSubscriptionDataSubscriptionData>,
    subscription_schedule:
        Option<Option<stripe_types::Expandable<stripe_shared::SubscriptionSchedule>>>,
    test_clock: Option<Option<stripe_types::Expandable<stripe_shared::TestHelpersTestClock>>>,
    total_details: Option<stripe_shared::QuotesResourceTotalDetails>,
    transfer_data: Option<Option<stripe_shared::QuotesResourceTransferData>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for Quote {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Quote>,
        builder: QuoteBuilder,
    }

    impl Visitor for Place<Quote> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: QuoteBuilder::deser_default() }))
        }
    }

    impl MapBuilder for QuoteBuilder {
        type Out = Quote;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_subtotal" => Deserialize::begin(&mut self.amount_subtotal),
                "amount_total" => Deserialize::begin(&mut self.amount_total),
                "application" => Deserialize::begin(&mut self.application),
                "application_fee_amount" => Deserialize::begin(&mut self.application_fee_amount),
                "application_fee_percent" => Deserialize::begin(&mut self.application_fee_percent),
                "automatic_tax" => Deserialize::begin(&mut self.automatic_tax),
                "collection_method" => Deserialize::begin(&mut self.collection_method),
                "computed" => Deserialize::begin(&mut self.computed),
                "created" => Deserialize::begin(&mut self.created),
                "currency" => Deserialize::begin(&mut self.currency),
                "customer" => Deserialize::begin(&mut self.customer),
                "default_tax_rates" => Deserialize::begin(&mut self.default_tax_rates),
                "description" => Deserialize::begin(&mut self.description),
                "discounts" => Deserialize::begin(&mut self.discounts),
                "expires_at" => Deserialize::begin(&mut self.expires_at),
                "footer" => Deserialize::begin(&mut self.footer),
                "from_quote" => Deserialize::begin(&mut self.from_quote),
                "header" => Deserialize::begin(&mut self.header),
                "id" => Deserialize::begin(&mut self.id),
                "invoice" => Deserialize::begin(&mut self.invoice),
                "invoice_settings" => Deserialize::begin(&mut self.invoice_settings),
                "line_items" => Deserialize::begin(&mut self.line_items),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "number" => Deserialize::begin(&mut self.number),
                "on_behalf_of" => Deserialize::begin(&mut self.on_behalf_of),
                "status" => Deserialize::begin(&mut self.status),
                "status_transitions" => Deserialize::begin(&mut self.status_transitions),
                "subscription" => Deserialize::begin(&mut self.subscription),
                "subscription_data" => Deserialize::begin(&mut self.subscription_data),
                "subscription_schedule" => Deserialize::begin(&mut self.subscription_schedule),
                "test_clock" => Deserialize::begin(&mut self.test_clock),
                "total_details" => Deserialize::begin(&mut self.total_details),
                "transfer_data" => Deserialize::begin(&mut self.transfer_data),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount_subtotal: Deserialize::default(),
                amount_total: Deserialize::default(),
                application: Deserialize::default(),
                application_fee_amount: Deserialize::default(),
                application_fee_percent: Deserialize::default(),
                automatic_tax: Deserialize::default(),
                collection_method: Deserialize::default(),
                computed: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                customer: Deserialize::default(),
                default_tax_rates: Deserialize::default(),
                description: Deserialize::default(),
                discounts: Deserialize::default(),
                expires_at: Deserialize::default(),
                footer: Deserialize::default(),
                from_quote: Deserialize::default(),
                header: Deserialize::default(),
                id: Deserialize::default(),
                invoice: Deserialize::default(),
                invoice_settings: Deserialize::default(),
                line_items: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                number: Deserialize::default(),
                on_behalf_of: Deserialize::default(),
                status: Deserialize::default(),
                status_transitions: Deserialize::default(),
                subscription: Deserialize::default(),
                subscription_data: Deserialize::default(),
                subscription_schedule: Deserialize::default(),
                test_clock: Deserialize::default(),
                total_details: Deserialize::default(),
                transfer_data: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                amount_subtotal: self.amount_subtotal?,
                amount_total: self.amount_total?,
                application: self.application.take()?,
                application_fee_amount: self.application_fee_amount?,
                application_fee_percent: self.application_fee_percent?,
                automatic_tax: self.automatic_tax.take()?,
                collection_method: self.collection_method?,
                computed: self.computed.take()?,
                created: self.created?,
                currency: self.currency?,
                customer: self.customer.take()?,
                default_tax_rates: self.default_tax_rates.take()?,
                description: self.description.take()?,
                discounts: self.discounts.take()?,
                expires_at: self.expires_at?,
                footer: self.footer.take()?,
                from_quote: self.from_quote.take()?,
                header: self.header.take()?,
                id: self.id.take()?,
                invoice: self.invoice.take()?,
                invoice_settings: self.invoice_settings.take()?,
                line_items: self.line_items.take()?,
                livemode: self.livemode?,
                metadata: self.metadata.take()?,
                number: self.number.take()?,
                on_behalf_of: self.on_behalf_of.take()?,
                status: self.status?,
                status_transitions: self.status_transitions?,
                subscription: self.subscription.take()?,
                subscription_data: self.subscription_data.take()?,
                subscription_schedule: self.subscription_schedule.take()?,
                test_clock: self.test_clock.take()?,
                total_details: self.total_details.take()?,
                transfer_data: self.transfer_data.take()?,
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

    impl ObjectDeser for Quote {
        type Builder = QuoteBuilder;
    }

    impl FromValueOpt for Quote {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = QuoteBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount_subtotal" => b.amount_subtotal = Some(FromValueOpt::from_value(v)?),
                    "amount_total" => b.amount_total = Some(FromValueOpt::from_value(v)?),
                    "application" => b.application = Some(FromValueOpt::from_value(v)?),
                    "application_fee_amount" => {
                        b.application_fee_amount = Some(FromValueOpt::from_value(v)?)
                    }
                    "application_fee_percent" => {
                        b.application_fee_percent = Some(FromValueOpt::from_value(v)?)
                    }
                    "automatic_tax" => b.automatic_tax = Some(FromValueOpt::from_value(v)?),
                    "collection_method" => b.collection_method = Some(FromValueOpt::from_value(v)?),
                    "computed" => b.computed = Some(FromValueOpt::from_value(v)?),
                    "created" => b.created = Some(FromValueOpt::from_value(v)?),
                    "currency" => b.currency = Some(FromValueOpt::from_value(v)?),
                    "customer" => b.customer = Some(FromValueOpt::from_value(v)?),
                    "default_tax_rates" => b.default_tax_rates = Some(FromValueOpt::from_value(v)?),
                    "description" => b.description = Some(FromValueOpt::from_value(v)?),
                    "discounts" => b.discounts = Some(FromValueOpt::from_value(v)?),
                    "expires_at" => b.expires_at = Some(FromValueOpt::from_value(v)?),
                    "footer" => b.footer = Some(FromValueOpt::from_value(v)?),
                    "from_quote" => b.from_quote = Some(FromValueOpt::from_value(v)?),
                    "header" => b.header = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "invoice" => b.invoice = Some(FromValueOpt::from_value(v)?),
                    "invoice_settings" => b.invoice_settings = Some(FromValueOpt::from_value(v)?),
                    "line_items" => b.line_items = Some(FromValueOpt::from_value(v)?),
                    "livemode" => b.livemode = Some(FromValueOpt::from_value(v)?),
                    "metadata" => b.metadata = Some(FromValueOpt::from_value(v)?),
                    "number" => b.number = Some(FromValueOpt::from_value(v)?),
                    "on_behalf_of" => b.on_behalf_of = Some(FromValueOpt::from_value(v)?),
                    "status" => b.status = Some(FromValueOpt::from_value(v)?),
                    "status_transitions" => {
                        b.status_transitions = Some(FromValueOpt::from_value(v)?)
                    }
                    "subscription" => b.subscription = Some(FromValueOpt::from_value(v)?),
                    "subscription_data" => b.subscription_data = Some(FromValueOpt::from_value(v)?),
                    "subscription_schedule" => {
                        b.subscription_schedule = Some(FromValueOpt::from_value(v)?)
                    }
                    "test_clock" => b.test_clock = Some(FromValueOpt::from_value(v)?),
                    "total_details" => b.total_details = Some(FromValueOpt::from_value(v)?),
                    "transfer_data" => b.transfer_data = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for Quote {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Quote", 35)?;
        s.serialize_field("amount_subtotal", &self.amount_subtotal)?;
        s.serialize_field("amount_total", &self.amount_total)?;
        s.serialize_field("application", &self.application)?;
        s.serialize_field("application_fee_amount", &self.application_fee_amount)?;
        s.serialize_field("application_fee_percent", &self.application_fee_percent)?;
        s.serialize_field("automatic_tax", &self.automatic_tax)?;
        s.serialize_field("collection_method", &self.collection_method)?;
        s.serialize_field("computed", &self.computed)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("customer", &self.customer)?;
        s.serialize_field("default_tax_rates", &self.default_tax_rates)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("discounts", &self.discounts)?;
        s.serialize_field("expires_at", &self.expires_at)?;
        s.serialize_field("footer", &self.footer)?;
        s.serialize_field("from_quote", &self.from_quote)?;
        s.serialize_field("header", &self.header)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("invoice", &self.invoice)?;
        s.serialize_field("invoice_settings", &self.invoice_settings)?;
        s.serialize_field("line_items", &self.line_items)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("number", &self.number)?;
        s.serialize_field("on_behalf_of", &self.on_behalf_of)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("status_transitions", &self.status_transitions)?;
        s.serialize_field("subscription", &self.subscription)?;
        s.serialize_field("subscription_data", &self.subscription_data)?;
        s.serialize_field("subscription_schedule", &self.subscription_schedule)?;
        s.serialize_field("test_clock", &self.test_clock)?;
        s.serialize_field("total_details", &self.total_details)?;
        s.serialize_field("transfer_data", &self.transfer_data)?;

        s.serialize_field("object", "quote")?;
        s.end()
    }
}
impl stripe_types::Object for Quote {
    type Id = stripe_shared::QuoteId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(QuoteId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum QuoteCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}
impl QuoteCollectionMethod {
    pub fn as_str(self) -> &'static str {
        use QuoteCollectionMethod::*;
        match self {
            ChargeAutomatically => "charge_automatically",
            SendInvoice => "send_invoice",
        }
    }
}

impl std::str::FromStr for QuoteCollectionMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use QuoteCollectionMethod::*;
        match s {
            "charge_automatically" => Ok(ChargeAutomatically),
            "send_invoice" => Ok(SendInvoice),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for QuoteCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for QuoteCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for QuoteCollectionMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for QuoteCollectionMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<QuoteCollectionMethod> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(QuoteCollectionMethod::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(QuoteCollectionMethod);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for QuoteCollectionMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for QuoteCollectionMethod"))
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum QuoteStatus {
    Accepted,
    Canceled,
    Draft,
    Open,
}
impl QuoteStatus {
    pub fn as_str(self) -> &'static str {
        use QuoteStatus::*;
        match self {
            Accepted => "accepted",
            Canceled => "canceled",
            Draft => "draft",
            Open => "open",
        }
    }
}

impl std::str::FromStr for QuoteStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use QuoteStatus::*;
        match s {
            "accepted" => Ok(Accepted),
            "canceled" => Ok(Canceled),
            "draft" => Ok(Draft),
            "open" => Ok(Open),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for QuoteStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for QuoteStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for QuoteStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for QuoteStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<QuoteStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(QuoteStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(QuoteStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for QuoteStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for QuoteStatus"))
    }
}
