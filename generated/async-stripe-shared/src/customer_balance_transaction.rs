/// Each customer has a [Balance](https://stripe.com/docs/api/customers/object#customer_object-balance) value,.
/// which denotes a debit or credit that's automatically applied to their next invoice upon finalization.
/// You may modify the value directly by using the [update customer API](https://stripe.com/docs/api/customers/update),.
/// or by creating a Customer Balance Transaction, which increments or decrements the customer's `balance` by the specified `amount`.
///
/// Related guide: [Customer balance](https://stripe.com/docs/billing/customer/balance)
///
/// For more details see <<https://stripe.com/docs/api/customer_balance_transactions/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerBalanceTransaction {
    /// The amount of the transaction.
    /// A negative value is a credit for the customer's balance, and a positive value is a debit to the customer's `balance`.
    pub amount: i64,
    /// The ID of the checkout session (if any) that created the transaction.
    pub checkout_session: Option<stripe_types::Expandable<stripe_shared::CheckoutSession>>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The ID of the credit note (if any) related to the transaction.
    pub credit_note: Option<stripe_types::Expandable<stripe_shared::CreditNote>>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the customer the transaction belongs to.
    pub customer: stripe_types::Expandable<stripe_shared::Customer>,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    /// The customer's `balance` after the transaction was applied.
    /// A negative value decreases the amount due on the customer's next invoice.
    /// A positive value increases the amount due on the customer's next invoice.
    pub ending_balance: i64,
    /// Unique identifier for the object.
    pub id: stripe_shared::CustomerBalanceTransactionId,
    /// The ID of the invoice (if any) related to the transaction.
    pub invoice: Option<stripe_types::Expandable<stripe_shared::Invoice>>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Transaction type: `adjustment`, `applied_to_invoice`, `credit_note`, `initial`, `invoice_overpaid`, `invoice_too_large`, `invoice_too_small`, `unspent_receiver_credit`, `unapplied_from_invoice`, `checkout_session_subscription_payment`, or `checkout_session_subscription_payment_canceled`.
    /// See the [Customer Balance page](https://stripe.com/docs/billing/customer/balance#types) to learn more about transaction types.
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: CustomerBalanceTransactionType,
}
#[doc(hidden)]
pub struct CustomerBalanceTransactionBuilder {
    amount: Option<i64>,
    checkout_session: Option<Option<stripe_types::Expandable<stripe_shared::CheckoutSession>>>,
    created: Option<stripe_types::Timestamp>,
    credit_note: Option<Option<stripe_types::Expandable<stripe_shared::CreditNote>>>,
    currency: Option<stripe_types::Currency>,
    customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    description: Option<Option<String>>,
    ending_balance: Option<i64>,
    id: Option<stripe_shared::CustomerBalanceTransactionId>,
    invoice: Option<Option<stripe_types::Expandable<stripe_shared::Invoice>>>,
    livemode: Option<bool>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    type_: Option<CustomerBalanceTransactionType>,
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
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CustomerBalanceTransaction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CustomerBalanceTransaction>,
        builder: CustomerBalanceTransactionBuilder,
    }

    impl Visitor for Place<CustomerBalanceTransaction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CustomerBalanceTransactionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CustomerBalanceTransactionBuilder {
        type Out = CustomerBalanceTransaction;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "checkout_session" => Deserialize::begin(&mut self.checkout_session),
                "created" => Deserialize::begin(&mut self.created),
                "credit_note" => Deserialize::begin(&mut self.credit_note),
                "currency" => Deserialize::begin(&mut self.currency),
                "customer" => Deserialize::begin(&mut self.customer),
                "description" => Deserialize::begin(&mut self.description),
                "ending_balance" => Deserialize::begin(&mut self.ending_balance),
                "id" => Deserialize::begin(&mut self.id),
                "invoice" => Deserialize::begin(&mut self.invoice),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                checkout_session: Deserialize::default(),
                created: Deserialize::default(),
                credit_note: Deserialize::default(),
                currency: Deserialize::default(),
                customer: Deserialize::default(),
                description: Deserialize::default(),
                ending_balance: Deserialize::default(),
                id: Deserialize::default(),
                invoice: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(amount),
                Some(checkout_session),
                Some(created),
                Some(credit_note),
                Some(currency),
                Some(customer),
                Some(description),
                Some(ending_balance),
                Some(id),
                Some(invoice),
                Some(livemode),
                Some(metadata),
                Some(type_),
            ) = (
                self.amount,
                self.checkout_session.take(),
                self.created,
                self.credit_note.take(),
                self.currency.take(),
                self.customer.take(),
                self.description.take(),
                self.ending_balance,
                self.id.take(),
                self.invoice.take(),
                self.livemode,
                self.metadata.take(),
                self.type_,
            )
            else {
                return None;
            };
            Some(Self::Out {
                amount,
                checkout_session,
                created,
                credit_note,
                currency,
                customer,
                description,
                ending_balance,
                id,
                invoice,
                livemode,
                metadata,
                type_,
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

    impl ObjectDeser for CustomerBalanceTransaction {
        type Builder = CustomerBalanceTransactionBuilder;
    }

    impl FromValueOpt for CustomerBalanceTransaction {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CustomerBalanceTransactionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "checkout_session" => b.checkout_session = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "credit_note" => b.credit_note = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "customer" => b.customer = FromValueOpt::from_value(v),
                    "description" => b.description = FromValueOpt::from_value(v),
                    "ending_balance" => b.ending_balance = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "invoice" => b.invoice = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for CustomerBalanceTransaction {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("CustomerBalanceTransaction", 14)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("checkout_session", &self.checkout_session)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("credit_note", &self.credit_note)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("customer", &self.customer)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("ending_balance", &self.ending_balance)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("invoice", &self.invoice)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("type", &self.type_)?;

        s.serialize_field("object", "customer_balance_transaction")?;
        s.end()
    }
}
/// Transaction type: `adjustment`, `applied_to_invoice`, `credit_note`, `initial`, `invoice_overpaid`, `invoice_too_large`, `invoice_too_small`, `unspent_receiver_credit`, `unapplied_from_invoice`, `checkout_session_subscription_payment`, or `checkout_session_subscription_payment_canceled`.
/// See the [Customer Balance page](https://stripe.com/docs/billing/customer/balance#types) to learn more about transaction types.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CustomerBalanceTransactionType {
    Adjustment,
    AppliedToInvoice,
    CheckoutSessionSubscriptionPayment,
    CheckoutSessionSubscriptionPaymentCanceled,
    CreditNote,
    Initial,
    InvoiceOverpaid,
    InvoiceTooLarge,
    InvoiceTooSmall,
    Migration,
    UnappliedFromInvoice,
    UnspentReceiverCredit,
}
impl CustomerBalanceTransactionType {
    pub fn as_str(self) -> &'static str {
        use CustomerBalanceTransactionType::*;
        match self {
            Adjustment => "adjustment",
            AppliedToInvoice => "applied_to_invoice",
            CheckoutSessionSubscriptionPayment => "checkout_session_subscription_payment",
            CheckoutSessionSubscriptionPaymentCanceled => {
                "checkout_session_subscription_payment_canceled"
            }
            CreditNote => "credit_note",
            Initial => "initial",
            InvoiceOverpaid => "invoice_overpaid",
            InvoiceTooLarge => "invoice_too_large",
            InvoiceTooSmall => "invoice_too_small",
            Migration => "migration",
            UnappliedFromInvoice => "unapplied_from_invoice",
            UnspentReceiverCredit => "unspent_receiver_credit",
        }
    }
}

impl std::str::FromStr for CustomerBalanceTransactionType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CustomerBalanceTransactionType::*;
        match s {
            "adjustment" => Ok(Adjustment),
            "applied_to_invoice" => Ok(AppliedToInvoice),
            "checkout_session_subscription_payment" => Ok(CheckoutSessionSubscriptionPayment),
            "checkout_session_subscription_payment_canceled" => {
                Ok(CheckoutSessionSubscriptionPaymentCanceled)
            }
            "credit_note" => Ok(CreditNote),
            "initial" => Ok(Initial),
            "invoice_overpaid" => Ok(InvoiceOverpaid),
            "invoice_too_large" => Ok(InvoiceTooLarge),
            "invoice_too_small" => Ok(InvoiceTooSmall),
            "migration" => Ok(Migration),
            "unapplied_from_invoice" => Ok(UnappliedFromInvoice),
            "unspent_receiver_credit" => Ok(UnspentReceiverCredit),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CustomerBalanceTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CustomerBalanceTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CustomerBalanceTransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CustomerBalanceTransactionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CustomerBalanceTransactionType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CustomerBalanceTransactionType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CustomerBalanceTransactionType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CustomerBalanceTransactionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CustomerBalanceTransactionType")
        })
    }
}
impl stripe_types::Object for CustomerBalanceTransaction {
    type Id = stripe_shared::CustomerBalanceTransactionId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(CustomerBalanceTransactionId);
