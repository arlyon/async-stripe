/// A Quote is a way to model prices that you'd like to provide to a customer.
/// Once accepted, it will automatically create an invoice, subscription or subscription schedule.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Quote {
    /// Total before any discounts or taxes are applied.
    pub amount_subtotal: i64,
    /// Total after discounts and taxes are applied.
    pub amount_total: i64,
    /// ID of the Connect Application that created the quote.
    pub application: Option<stripe_types::Expandable<stripe_types::application::Application>>,
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    ///
    /// Only applicable if there are no line items with recurring prices on the quote.
    pub application_fee_amount: Option<i64>,
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account.
    /// Only applicable if there are line items with recurring prices on the quote.
    pub application_fee_percent: Option<f64>,
    pub automatic_tax: stripe_core::quote::automatic_tax::AutomaticTax,
    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay invoices at the end of the subscription cycle or on finalization using the default payment method attached to the subscription or customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    /// Defaults to `charge_automatically`.
    pub collection_method: QuoteCollectionMethod,
    pub computed: stripe_core::quote::computed::Computed,
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
    pub customer: Option<stripe_types::Expandable<stripe_core::customer::Customer>>,
    /// The tax rates applied to this quote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<stripe_types::Expandable<stripe_core::tax_rate::TaxRate>>>,
    /// A description that will be displayed on the quote PDF.
    pub description: Option<String>,
    /// The discounts applied to this quote.
    pub discounts: Vec<stripe_types::Expandable<stripe_core::discount::Discount>>,
    /// The date on which the quote will be canceled if in `open` or `draft` status.
    ///
    /// Measured in seconds since the Unix epoch.
    pub expires_at: stripe_types::Timestamp,
    /// A footer that will be displayed on the quote PDF.
    pub footer: Option<String>,
    /// Details of the quote that was cloned.
    ///
    /// See the [cloning documentation](https://stripe.com/docs/quotes/clone) for more details.
    pub from_quote: Option<stripe_core::quote::from_quote::FromQuote>,
    /// A header that will be displayed on the quote PDF.
    pub header: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_core::quote::QuoteId,
    /// The invoice that was created from this quote.
    pub invoice: Option<stripe_types::Expandable<stripe_core::invoice::Invoice>>,
    /// All invoices will be billed using the specified settings.
    pub invoice_settings: Option<stripe_core::quote::invoice_settings::InvoiceSettings>,
    /// A list of items the customer is being quoted for.
    #[serde(default)]
    pub line_items: stripe_types::List<Option<stripe_core::line_item::LineItem>>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: stripe_types::Metadata,
    /// A unique number that identifies this particular quote.
    ///
    /// This number is assigned once the quote is [finalized](https://stripe.com/docs/quotes/overview#finalize).
    pub number: Option<String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: QuoteObject,
    /// The account on behalf of which to charge.
    ///
    /// See the [Connect documentation](https://support.stripe.com/questions/sending-invoices-on-behalf-of-connected-accounts) for details.
    pub on_behalf_of: Option<stripe_types::Expandable<stripe_core::account::Account>>,
    /// The status of the quote.
    pub status: QuoteStatus,
    pub status_transitions: stripe_core::quote::status_transitions::StatusTransitions,
    /// The subscription that was created or updated from this quote.
    pub subscription: Option<stripe_types::Expandable<stripe_core::subscription::Subscription>>,
    pub subscription_data: stripe_core::quote::subscription_data::SubscriptionData,
    /// The subscription schedule that was created or updated from this quote.
    pub subscription_schedule:
        Option<stripe_types::Expandable<stripe_core::subscription_schedule::SubscriptionSchedule>>,
    /// ID of the test clock this quote belongs to.
    pub test_clock:
        Option<stripe_types::Expandable<stripe_core::test_helpers::test_clock::TestClock>>,
    pub total_details: stripe_core::quote::total_details::TotalDetails,
    /// The account (if any) the payments will be attributed to for tax reporting, and where funds from each payment will be transferred to for each of the invoices.
    pub transfer_data: Option<stripe_core::quote::transfer_data::TransferData>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Quote {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Either `charge_automatically`, or `send_invoice`.
///
/// When charging automatically, Stripe will attempt to pay invoices at the end of the subscription cycle or on finalization using the default payment method attached to the subscription or customer.
/// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
/// Defaults to `charge_automatically`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum QuoteCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl QuoteCollectionMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ChargeAutomatically => "charge_automatically",
            Self::SendInvoice => "send_invoice",
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
        self.as_str().fmt(f)
    }
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum QuoteObject {
    Quote,
}

impl QuoteObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Quote => "quote",
        }
    }
}

impl AsRef<str> for QuoteObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for QuoteObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The status of the quote.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum QuoteStatus {
    Accepted,
    Canceled,
    Draft,
    Open,
}

impl QuoteStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Canceled => "canceled",
            Self::Draft => "draft",
            Self::Open => "open",
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
        self.as_str().fmt(f)
    }
}
impl stripe_types::Object for Quote {
    type Id = stripe_core::quote::QuoteId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(QuoteId, "qt_");
pub mod invoice_settings;
pub mod requests;
pub use invoice_settings::InvoiceSettings;
pub mod automatic_tax;
pub use automatic_tax::AutomaticTax;
pub mod computed;
pub use computed::Computed;
pub mod from_quote;
pub use from_quote::FromQuote;
pub mod recurring;
pub use recurring::Recurring;
pub mod status_transitions;
pub use status_transitions::StatusTransitions;
pub mod subscription_data;
pub use subscription_data::SubscriptionData;
pub mod total_details;
pub use total_details::TotalDetails;
pub mod transfer_data;
pub use transfer_data::TransferData;
pub mod upfront;
pub use upfront::Upfront;
