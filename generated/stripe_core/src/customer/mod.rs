/// This object represents a customer of your business.
///
/// It lets you create recurring charges and track payments that belong to the same customer.  Related guide: [Save a card during payment](https://stripe.com/docs/payments/save-during-payment).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Customer {
    /// The customer's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types::address::Address>,
    /// Current balance, if any, being stored on the customer.
    ///
    /// If negative, the customer has credit to apply to their next invoice.
    /// If positive, the customer has an amount owed that will be added to their next invoice.
    /// The balance does not refer to any unpaid invoices; it solely takes into account amounts that have yet to be successfully applied to any invoice.
    /// This balance is only taken into account as invoices are finalized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<i64>,
    /// The current funds being held by Stripe on behalf of the customer.
    ///
    /// These funds can be applied towards payment intents with source "cash_balance".
    /// The settings[reconciliation_mode] field describes whether these funds are applied to such payment intents manually or automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_balance: Option<stripe_core::cash_balance::CashBalance>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO code for the currency](https://stripe.com/docs/currencies) the customer can be charged in for recurring billing purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// ID of the default payment source for the customer.
    ///
    /// If you are using payment methods created via the PaymentMethods API, see the [invoice_settings.default_payment_method](https://stripe.com/docs/api/customers/object#customer_object-invoice_settings-default_payment_method) field instead.
    pub default_source:
        Option<stripe_types::Expandable<stripe_core::payment_source::PaymentSource>>,
    /// When the customer's latest invoice is billed by charging automatically, `delinquent` is `true` if the invoice's latest charge failed.
    ///
    /// When the customer's latest invoice is billed by sending an invoice, `delinquent` is `true` if the invoice isn't paid by its due date.  If an invoice is marked uncollectible by [dunning](https://stripe.com/docs/billing/automatic-collection), `delinquent` doesn't get reset to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delinquent: Option<bool>,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,
    /// Describes the current discount active on the customer, if there is one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<stripe_core::discount::Discount>,
    /// The customer's email address.
    pub email: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_core::customer::CustomerId,
    /// The current multi-currency balances, if any, being stored on the customer.
    ///
    /// If positive in a currency, the customer has a credit to apply to their next invoice denominated in that currency.
    /// If negative, the customer has an amount owed that will be added to their next invoice denominated in that currency.
    /// These balances do not refer to any unpaid invoices.
    /// They solely track amounts that have yet to be successfully applied to any invoice.
    /// A balance in a particular currency is only applied to any invoice as an invoice in that currency is finalized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_credit_balance: Option<i64>,
    /// The prefix for the customer used to generate unique invoice numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<stripe_core::customer::invoice_settings::InvoiceSettings>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<stripe_types::Metadata>,
    /// The customer's full name or business name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The suffix of the customer's next invoice number, e.g., 0001.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_invoice_sequence: Option<i64>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: CustomerObject,
    /// The customer's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// The customer's preferred locales (languages), ordered by preference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locales: Option<Vec<String>>,
    /// Mailing and shipping address for the customer.
    ///
    /// Appears on invoices emailed to this customer.
    pub shipping: Option<stripe_types::shipping_details::ShippingDetails>,
    /// The customer's payment sources, if any.
    #[serde(default)]
    pub sources: stripe_types::List<Option<stripe_core::payment_source::PaymentSource>>,
    /// The customer's current subscriptions, if any.
    #[serde(default)]
    pub subscriptions: stripe_types::List<Option<stripe_core::subscription::Subscription>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<stripe_core::customer::tax::Tax>,
    /// Describes the customer's tax exemption status.
    ///
    /// One of `none`, `exempt`, or `reverse`.
    /// When set to `reverse`, invoice and receipt PDFs include the text **"Reverse charge"**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<CustomerTaxExempt>,
    /// The customer's tax IDs.
    #[serde(default)]
    pub tax_ids: stripe_types::List<Option<stripe_core::tax_id::TaxId>>,
    /// ID of the test clock this customer belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_clock:
        Option<stripe_types::Expandable<stripe_core::test_helpers::test_clock::TestClock>>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Customer {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum CustomerObject {
    Customer,
}

impl CustomerObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Customer => "customer",
        }
    }
}

impl AsRef<str> for CustomerObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Describes the customer's tax exemption status.
///
/// One of `none`, `exempt`, or `reverse`.
/// When set to `reverse`, invoice and receipt PDFs include the text **"Reverse charge"**.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum CustomerTaxExempt {
    Exempt,
    None,
    Reverse,
}

impl CustomerTaxExempt {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exempt => "exempt",
            Self::None => "none",
            Self::Reverse => "reverse",
        }
    }
}

impl AsRef<str> for CustomerTaxExempt {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl stripe_types::Object for Customer {
    type Id = stripe_core::customer::CustomerId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(CustomerId, "cs_");
pub mod deleted;
pub mod requests;
pub use deleted::DeletedCustomer;
pub mod tax;
pub use tax::Tax;
pub mod invoice_settings;
pub use invoice_settings::InvoiceSettings;
