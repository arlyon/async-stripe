/// A Quote is a way to model prices that you'd like to provide to a customer.
/// Once accepted, it will automatically create an invoice, subscription or subscription schedule.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Quote {
    /// Total before any discounts or taxes are applied.
    pub amount_subtotal: i64,
    /// Total after discounts and taxes are applied.
    pub amount_total: i64,
    /// ID of the Connect Application that created the quote.
    pub application: Option<stripe_types::Expandable<stripe_types::Application>>,
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    ///
    /// Only applicable if there are no line items with recurring prices on the quote.
    pub application_fee_amount: Option<i64>,
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    /// Only applicable if there are line items with recurring prices on the quote.
    pub application_fee_percent: Option<f64>,
    pub automatic_tax: stripe_types::QuotesResourceAutomaticTax,
    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay invoices at the end of the subscription cycle or on finalization using the default payment method attached to the subscription or customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    /// Defaults to `charge_automatically`.
    pub collection_method: QuoteCollectionMethod,
    pub computed: stripe_types::QuotesResourceComputed,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Option<stripe_types::Currency>,
    /// The customer which this quote belongs to.
    ///
    /// A customer is required before finalizing the quote.
    /// Once specified, it cannot be changed.
    pub customer: Option<stripe_types::Expandable<stripe_types::Customer>>,
    /// The tax rates applied to this quote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<stripe_types::Expandable<stripe_types::TaxRate>>>,
    /// A description that will be displayed on the quote PDF.
    pub description: Option<String>,
    /// The discounts applied to this quote.
    pub discounts: Vec<stripe_types::Expandable<stripe_types::Discount>>,
    /// The date on which the quote will be canceled if in `open` or `draft` status.
    ///
    /// Measured in seconds since the Unix epoch.
    pub expires_at: stripe_types::Timestamp,
    /// A footer that will be displayed on the quote PDF.
    pub footer: Option<String>,
    /// Details of the quote that was cloned.
    ///
    /// See the [cloning documentation](https://stripe.com/docs/quotes/clone) for more details.
    pub from_quote: Option<stripe_types::QuotesResourceFromQuote>,
    /// A header that will be displayed on the quote PDF.
    pub header: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_types::quote::QuoteId,
    /// The invoice that was created from this quote.
    pub invoice: Option<stripe_types::Expandable<stripe_types::Invoice>>,
    /// All invoices will be billed using the specified settings.
    pub invoice_settings: Option<stripe_types::InvoiceSettingQuoteSetting>,
    /// A list of items the customer is being quoted for.
    #[serde(default)]
    pub line_items: stripe_types::List<stripe_types::LineItem>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// A unique number that identifies this particular quote.
    ///
    /// This number is assigned once the quote is [finalized](https://stripe.com/docs/quotes/overview#finalize).
    pub number: Option<String>,
    /// The account on behalf of which to charge.
    ///
    /// See the [Connect documentation](https://support.stripe.com/questions/sending-invoices-on-behalf-of-connected-accounts) for details.
    pub on_behalf_of: Option<stripe_types::Expandable<stripe_types::Account>>,
    /// The status of the quote.
    pub status: QuoteStatus,
    pub status_transitions: stripe_types::QuotesResourceStatusTransitions,
    /// The subscription that was created or updated from this quote.
    pub subscription: Option<stripe_types::Expandable<stripe_types::Subscription>>,
    pub subscription_data: stripe_types::QuotesResourceSubscriptionDataSubscriptionData,
    /// The subscription schedule that was created or updated from this quote.
    pub subscription_schedule: Option<stripe_types::Expandable<stripe_types::SubscriptionSchedule>>,
    /// ID of the test clock this quote belongs to.
    pub test_clock: Option<stripe_types::Expandable<stripe_types::TestClock>>,
    pub total_details: stripe_types::QuotesResourceTotalDetails,
    /// The account (if any) the payments will be attributed to for tax reporting, and where funds from each payment will be transferred to for each of the invoices.
    pub transfer_data: Option<stripe_types::QuotesResourceTransferData>,
}
/// Either `charge_automatically`, or `send_invoice`.
///
/// When charging automatically, Stripe will attempt to pay invoices at the end of the subscription cycle or on finalization using the default payment method attached to the subscription or customer.
/// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
/// Defaults to `charge_automatically`.
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use QuoteCollectionMethod::*;
        match s {
            "charge_automatically" => Ok(ChargeAutomatically),
            "send_invoice" => Ok(SendInvoice),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for QuoteCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
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
impl<'de> serde::Deserialize<'de> for QuoteCollectionMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for QuoteCollectionMethod"))
    }
}
/// The status of the quote.
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use QuoteStatus::*;
        match s {
            "accepted" => Ok(Accepted),
            "canceled" => Ok(Canceled),
            "draft" => Ok(Draft),
            "open" => Ok(Open),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for QuoteStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
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
impl<'de> serde::Deserialize<'de> for QuoteStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for QuoteStatus"))
    }
}
impl stripe_types::Object for Quote {
    type Id = stripe_types::quote::QuoteId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(QuoteId, "qt_");
