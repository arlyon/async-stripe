/// This object represents a customer of your business.
/// Use it to [create recurring charges](https://stripe.com/docs/invoicing/customer), [save payment](https://stripe.com/docs/payments/save-during-payment) and contact information,.
/// and track payments that belong to the same customer.
///
/// For more details see <<https://stripe.com/docs/api/customers/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Customer {
    /// The customer's address.
    pub address: Option<stripe_shared::Address>,
    /// The current balance, if any, that's stored on the customer.
    /// If negative, the customer has credit to apply to their next invoice.
    /// If positive, the customer has an amount owed that's added to their next invoice.
    /// The balance only considers amounts that Stripe hasn't successfully applied to any invoice.
    /// It doesn't reflect unpaid invoices.
    /// This balance is only taken into account after invoices finalize.
    pub balance: Option<i64>,
    /// The current funds being held by Stripe on behalf of the customer.
    /// You can apply these funds towards payment intents when the source is "cash_balance".
    /// The `settings[reconciliation_mode]` field describes if these funds apply to these payment intents manually or automatically.
    pub cash_balance: Option<stripe_shared::CashBalance>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO code for the currency](https://stripe.com/docs/currencies) the customer can be charged in for recurring billing purposes.
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
    pub delinquent: Option<bool>,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    /// Describes the current discount active on the customer, if there is one.
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
    pub invoice_credit_balance: Option<std::collections::HashMap<String, i64>>,
    /// The prefix for the customer used to generate unique invoice numbers.
    pub invoice_prefix: Option<String>,
    pub invoice_settings: Option<stripe_shared::InvoiceSettingCustomerSetting>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The customer's full name or business name.
    pub name: Option<String>,
    /// The suffix of the customer's next invoice number (for example, 0001).
    /// When the account uses account level sequencing, this parameter is ignored in API requests and the field omitted in API responses.
    pub next_invoice_sequence: Option<i64>,
    /// The customer's phone number.
    pub phone: Option<String>,
    /// The customer's preferred locales (languages), ordered by preference.
    pub preferred_locales: Option<Vec<String>>,
    /// Mailing and shipping address for the customer. Appears on invoices emailed to this customer.
    pub shipping: Option<stripe_shared::Shipping>,
    /// The customer's payment sources, if any.
    pub sources: Option<stripe_types::List<stripe_shared::PaymentSource>>,
    /// The customer's current subscriptions, if any.
    pub subscriptions: Option<stripe_types::List<stripe_shared::Subscription>>,
    pub tax: Option<stripe_shared::CustomerTax>,
    /// Describes the customer's tax exemption status, which is `none`, `exempt`, or `reverse`.
    /// When set to `reverse`, invoice and receipt PDFs include the following text: **"Reverse charge"**.
    pub tax_exempt: Option<stripe_shared::CustomerTaxExempt>,
    /// The customer's tax IDs.
    pub tax_ids: Option<stripe_types::List<stripe_shared::TaxId>>,
    /// ID of the test clock that this customer belongs to.
    pub test_clock: Option<stripe_types::Expandable<stripe_shared::TestHelpersTestClock>>,
}
#[doc(hidden)]
pub struct CustomerBuilder {
    address: Option<Option<stripe_shared::Address>>,
    balance: Option<Option<i64>>,
    cash_balance: Option<Option<stripe_shared::CashBalance>>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<Option<stripe_types::Currency>>,
    default_source: Option<Option<stripe_types::Expandable<stripe_shared::PaymentSource>>>,
    delinquent: Option<Option<bool>>,
    description: Option<Option<String>>,
    discount: Option<Option<stripe_shared::Discount>>,
    email: Option<Option<String>>,
    id: Option<stripe_shared::CustomerId>,
    invoice_credit_balance: Option<Option<std::collections::HashMap<String, i64>>>,
    invoice_prefix: Option<Option<String>>,
    invoice_settings: Option<Option<stripe_shared::InvoiceSettingCustomerSetting>>,
    livemode: Option<bool>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    name: Option<Option<String>>,
    next_invoice_sequence: Option<Option<i64>>,
    phone: Option<Option<String>>,
    preferred_locales: Option<Option<Vec<String>>>,
    shipping: Option<Option<stripe_shared::Shipping>>,
    sources: Option<Option<stripe_types::List<stripe_shared::PaymentSource>>>,
    subscriptions: Option<Option<stripe_types::List<stripe_shared::Subscription>>>,
    tax: Option<Option<stripe_shared::CustomerTax>>,
    tax_exempt: Option<Option<stripe_shared::CustomerTaxExempt>>,
    tax_ids: Option<Option<stripe_types::List<stripe_shared::TaxId>>>,
    test_clock: Option<Option<stripe_types::Expandable<stripe_shared::TestHelpersTestClock>>>,
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

    impl Deserialize for Customer {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Customer>,
        builder: CustomerBuilder,
    }

    impl Visitor for Place<Customer> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: CustomerBuilder::deser_default() }))
        }
    }

    impl MapBuilder for CustomerBuilder {
        type Out = Customer;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address" => Deserialize::begin(&mut self.address),
                "balance" => Deserialize::begin(&mut self.balance),
                "cash_balance" => Deserialize::begin(&mut self.cash_balance),
                "created" => Deserialize::begin(&mut self.created),
                "currency" => Deserialize::begin(&mut self.currency),
                "default_source" => Deserialize::begin(&mut self.default_source),
                "delinquent" => Deserialize::begin(&mut self.delinquent),
                "description" => Deserialize::begin(&mut self.description),
                "discount" => Deserialize::begin(&mut self.discount),
                "email" => Deserialize::begin(&mut self.email),
                "id" => Deserialize::begin(&mut self.id),
                "invoice_credit_balance" => Deserialize::begin(&mut self.invoice_credit_balance),
                "invoice_prefix" => Deserialize::begin(&mut self.invoice_prefix),
                "invoice_settings" => Deserialize::begin(&mut self.invoice_settings),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "name" => Deserialize::begin(&mut self.name),
                "next_invoice_sequence" => Deserialize::begin(&mut self.next_invoice_sequence),
                "phone" => Deserialize::begin(&mut self.phone),
                "preferred_locales" => Deserialize::begin(&mut self.preferred_locales),
                "shipping" => Deserialize::begin(&mut self.shipping),
                "sources" => Deserialize::begin(&mut self.sources),
                "subscriptions" => Deserialize::begin(&mut self.subscriptions),
                "tax" => Deserialize::begin(&mut self.tax),
                "tax_exempt" => Deserialize::begin(&mut self.tax_exempt),
                "tax_ids" => Deserialize::begin(&mut self.tax_ids),
                "test_clock" => Deserialize::begin(&mut self.test_clock),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                address: Deserialize::default(),
                balance: Deserialize::default(),
                cash_balance: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                default_source: Deserialize::default(),
                delinquent: Deserialize::default(),
                description: Deserialize::default(),
                discount: Deserialize::default(),
                email: Deserialize::default(),
                id: Deserialize::default(),
                invoice_credit_balance: Deserialize::default(),
                invoice_prefix: Deserialize::default(),
                invoice_settings: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                name: Deserialize::default(),
                next_invoice_sequence: Deserialize::default(),
                phone: Deserialize::default(),
                preferred_locales: Deserialize::default(),
                shipping: Deserialize::default(),
                sources: Deserialize::default(),
                subscriptions: Deserialize::default(),
                tax: Deserialize::default(),
                tax_exempt: Deserialize::default(),
                tax_ids: Deserialize::default(),
                test_clock: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(address),
                Some(balance),
                Some(cash_balance),
                Some(created),
                Some(currency),
                Some(default_source),
                Some(delinquent),
                Some(description),
                Some(discount),
                Some(email),
                Some(id),
                Some(invoice_credit_balance),
                Some(invoice_prefix),
                Some(invoice_settings),
                Some(livemode),
                Some(metadata),
                Some(name),
                Some(next_invoice_sequence),
                Some(phone),
                Some(preferred_locales),
                Some(shipping),
                Some(sources),
                Some(subscriptions),
                Some(tax),
                Some(tax_exempt),
                Some(tax_ids),
                Some(test_clock),
            ) = (
                self.address.take(),
                self.balance,
                self.cash_balance.take(),
                self.created,
                self.currency,
                self.default_source.take(),
                self.delinquent,
                self.description.take(),
                self.discount.take(),
                self.email.take(),
                self.id.take(),
                self.invoice_credit_balance.take(),
                self.invoice_prefix.take(),
                self.invoice_settings.take(),
                self.livemode,
                self.metadata.take(),
                self.name.take(),
                self.next_invoice_sequence,
                self.phone.take(),
                self.preferred_locales.take(),
                self.shipping.take(),
                self.sources.take(),
                self.subscriptions.take(),
                self.tax.take(),
                self.tax_exempt,
                self.tax_ids.take(),
                self.test_clock.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                address,
                balance,
                cash_balance,
                created,
                currency,
                default_source,
                delinquent,
                description,
                discount,
                email,
                id,
                invoice_credit_balance,
                invoice_prefix,
                invoice_settings,
                livemode,
                metadata,
                name,
                next_invoice_sequence,
                phone,
                preferred_locales,
                shipping,
                sources,
                subscriptions,
                tax,
                tax_exempt,
                tax_ids,
                test_clock,
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

    impl ObjectDeser for Customer {
        type Builder = CustomerBuilder;
    }

    impl FromValueOpt for Customer {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CustomerBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "address" => b.address = FromValueOpt::from_value(v),
                    "balance" => b.balance = FromValueOpt::from_value(v),
                    "cash_balance" => b.cash_balance = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "default_source" => b.default_source = FromValueOpt::from_value(v),
                    "delinquent" => b.delinquent = FromValueOpt::from_value(v),
                    "description" => b.description = FromValueOpt::from_value(v),
                    "discount" => b.discount = FromValueOpt::from_value(v),
                    "email" => b.email = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "invoice_credit_balance" => {
                        b.invoice_credit_balance = FromValueOpt::from_value(v)
                    }
                    "invoice_prefix" => b.invoice_prefix = FromValueOpt::from_value(v),
                    "invoice_settings" => b.invoice_settings = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "name" => b.name = FromValueOpt::from_value(v),
                    "next_invoice_sequence" => {
                        b.next_invoice_sequence = FromValueOpt::from_value(v)
                    }
                    "phone" => b.phone = FromValueOpt::from_value(v),
                    "preferred_locales" => b.preferred_locales = FromValueOpt::from_value(v),
                    "shipping" => b.shipping = FromValueOpt::from_value(v),
                    "sources" => b.sources = FromValueOpt::from_value(v),
                    "subscriptions" => b.subscriptions = FromValueOpt::from_value(v),
                    "tax" => b.tax = FromValueOpt::from_value(v),
                    "tax_exempt" => b.tax_exempt = FromValueOpt::from_value(v),
                    "tax_ids" => b.tax_ids = FromValueOpt::from_value(v),
                    "test_clock" => b.test_clock = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for Customer {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Customer", 28)?;
        s.serialize_field("address", &self.address)?;
        s.serialize_field("balance", &self.balance)?;
        s.serialize_field("cash_balance", &self.cash_balance)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("default_source", &self.default_source)?;
        s.serialize_field("delinquent", &self.delinquent)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("discount", &self.discount)?;
        s.serialize_field("email", &self.email)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("invoice_credit_balance", &self.invoice_credit_balance)?;
        s.serialize_field("invoice_prefix", &self.invoice_prefix)?;
        s.serialize_field("invoice_settings", &self.invoice_settings)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("next_invoice_sequence", &self.next_invoice_sequence)?;
        s.serialize_field("phone", &self.phone)?;
        s.serialize_field("preferred_locales", &self.preferred_locales)?;
        s.serialize_field("shipping", &self.shipping)?;
        s.serialize_field("sources", &self.sources)?;
        s.serialize_field("subscriptions", &self.subscriptions)?;
        s.serialize_field("tax", &self.tax)?;
        s.serialize_field("tax_exempt", &self.tax_exempt)?;
        s.serialize_field("tax_ids", &self.tax_ids)?;
        s.serialize_field("test_clock", &self.test_clock)?;

        s.serialize_field("object", "customer")?;
        s.end()
    }
}
impl stripe_types::Object for Customer {
    type Id = stripe_shared::CustomerId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerTaxExempt::*;
        match s {
            "exempt" => Ok(Exempt),
            "none" => Ok(None),
            "reverse" => Ok(Reverse),
            _ => Err(stripe_types::StripeParseError),
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
impl miniserde::Deserialize for CustomerTaxExempt {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CustomerTaxExempt> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CustomerTaxExempt::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CustomerTaxExempt);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CustomerTaxExempt {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CustomerTaxExempt"))
    }
}
