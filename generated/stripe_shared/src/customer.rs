/// This object represents a customer of your business.
/// Use it to create recurring charges and track payments that belong to the same customer.
///
/// Related guide: [Save a card during payment](https://stripe.com/docs/payments/save-during-payment)
///
/// For more details see <<https://stripe.com/docs/api/customers/object>>.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Customer {
    /// The customer's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_shared::Address>,
    /// The current balance, if any, that's stored on the customer.
    /// If negative, the customer has credit to apply to their next invoice.
    /// If positive, the customer has an amount owed that's added to their next invoice.
    /// The balance only considers amounts that Stripe hasn't successfully applied to any invoice.
    /// It doesn't reflect unpaid invoices.
    /// This balance is only taken into account after invoices finalize.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<i64>,
    /// The current funds being held by Stripe on behalf of the customer.
    /// You can apply these funds towards payment intents when the source is "cash_balance".
    /// The `settings[reconciliation_mode]` field describes if these funds apply to these payment intents manually or automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_balance: Option<stripe_shared::CashBalance>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO code for the currency](https://stripe.com/docs/currencies) the customer can be charged in for recurring billing purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// ID of the default payment source for the customer.
    ///
    /// If you use payment methods created through the PaymentMethods API, see the [invoice_settings.default_payment_method](https://stripe.com/docs/api/customers/object#customer_object-invoice_settings-default_payment_method) field instead.
    pub default_source: Option<stripe_types::Expandable<stripe_shared::PaymentSource>>,
    /// Tracks the most recent state change on any invoice belonging to the customer.
    /// Paying an invoice or marking it uncollectible via the API will set this field to false.
    /// An automatic payment failure or passing the `invoice.due_date` will set this field to `true`.
    ///
    /// If an invoice becomes uncollectible by [dunning](https://stripe.com/docs/billing/automatic-collection), `delinquent` doesn't reset to `false`.
    ///
    /// If you care whether the customer has paid their most recent subscription invoice, use `subscription.status` instead.
    /// Paying or marking uncollectible any customer invoice regardless of whether it is the latest invoice for a subscription will always set this field to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delinquent: Option<bool>,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    /// Describes the current discount active on the customer, if there is one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<stripe_shared::Discount>,
    /// The customer's email address.
    pub email: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_shared::CustomerId,
    /// The current multi-currency balances, if any, that's stored on the customer.
    /// If positive in a currency, the customer has a credit to apply to their next invoice denominated in that currency.
    /// If negative, the customer has an amount owed that's added to their next invoice denominated in that currency.
    /// These balances don't apply to unpaid invoices.
    /// They solely track amounts that Stripe hasn't successfully applied to any invoice.
    /// Stripe only applies a balance in a specific currency to an invoice after that invoice (which is in the same currency) finalizes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_credit_balance: Option<std::collections::HashMap<String, i64>>,
    /// The prefix for the customer used to generate unique invoice numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<stripe_shared::InvoiceSettingCustomerSetting>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The customer's full name or business name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The suffix of the customer's next invoice number (for example, 0001).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_invoice_sequence: Option<i64>,
    /// The customer's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// The customer's preferred locales (languages), ordered by preference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locales: Option<Vec<String>>,
    /// Mailing and shipping address for the customer. Appears on invoices emailed to this customer.
    pub shipping: Option<stripe_shared::Shipping>,
    /// The customer's payment sources, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<stripe_types::List<stripe_shared::PaymentSource>>,
    /// The customer's current subscriptions, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<stripe_types::List<stripe_shared::Subscription>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<stripe_shared::CustomerTax>,
    /// Describes the customer's tax exemption status, which is `none`, `exempt`, or `reverse`.
    /// When set to `reverse`, invoice and receipt PDFs include the following text: **"Reverse charge"**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<stripe_shared::CustomerTaxExempt>,
    /// The customer's tax IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_ids: Option<stripe_types::List<stripe_shared::TaxId>>,
    /// ID of the test clock that this customer belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<stripe_types::Expandable<stripe_shared::TestHelpersTestClock>>,
}
impl stripe_types::Object for Customer {
    type Id = stripe_shared::CustomerId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(CustomerId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CustomerTaxExempt {
    Exempt,
    None,
    Reverse,
}
impl CustomerTaxExempt {
    pub fn as_str(self) -> &'static str {
        use CustomerTaxExempt::*;
        match self {
            Exempt => "exempt",
            None => "none",
            Reverse => "reverse",
        }
    }
}

impl std::str::FromStr for CustomerTaxExempt {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerTaxExempt::*;
        match s {
            "exempt" => Ok(Exempt),
            "none" => Ok(None),
            "reverse" => Ok(Reverse),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CustomerTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CustomerTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CustomerTaxExempt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CustomerTaxExempt {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CustomerTaxExempt"))
    }
}
