/// Invoice Payments represent payments made against invoices. Invoice Payments can
/// be accessed in two ways:
/// 1. By expanding the `payments` field on the [Invoice](https://api.stripe.com#invoice) resource.
/// 2. By using the Invoice Payment retrieve and list endpoints.
///
/// Invoice Payments include the mapping between payment objects, such as Payment Intent, and Invoices.
/// This resource and its endpoints allows you to easily track if a payment is associated with a specific invoice and.
/// monitor the allocation details of the payments.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoicePayment {
    /// Amount that was actually paid for this invoice, in cents (or local equivalent).
    /// This field is null until the payment is `paid`.
    /// This amount can be less than the `amount_requested` if the PaymentIntent’s `amount_received` is not sufficient to pay all of the invoices that it is attached to.
    pub amount_paid: Option<i64>,
    /// Amount intended to be paid toward this invoice, in cents (or local equivalent)
    pub amount_requested: i64,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Unique identifier for the object.
    pub id: stripe_shared::InvoicePaymentId,
    /// The invoice that was paid.
    pub invoice: stripe_types::Expandable<stripe_shared::Invoice>,
    /// Stripe automatically creates a default InvoicePayment when the invoice is finalized, and keeps it synchronized with the invoice’s `amount_remaining`.
    /// The PaymentIntent associated with the default payment can’t be edited or canceled directly.
    pub is_default: bool,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    pub payment: stripe_shared::InvoicesPaymentsInvoicePaymentAssociatedPayment,
    /// The status of the payment, one of `open`, `paid`, or `canceled`.
    pub status: String,
    pub status_transitions: stripe_shared::InvoicesPaymentsInvoicePaymentStatusTransitions,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoicePayment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoicePayment").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InvoicePaymentBuilder {
    amount_paid: Option<Option<i64>>,
    amount_requested: Option<i64>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    id: Option<stripe_shared::InvoicePaymentId>,
    invoice: Option<stripe_types::Expandable<stripe_shared::Invoice>>,
    is_default: Option<bool>,
    livemode: Option<bool>,
    payment: Option<stripe_shared::InvoicesPaymentsInvoicePaymentAssociatedPayment>,
    status: Option<String>,
    status_transitions: Option<stripe_shared::InvoicesPaymentsInvoicePaymentStatusTransitions>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

    make_place!(Place);

    impl Deserialize for InvoicePayment {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoicePayment>,
        builder: InvoicePaymentBuilder,
    }

    impl Visitor for Place<InvoicePayment> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoicePaymentBuilder {
                    amount_paid: Deserialize::default(),
                    amount_requested: Deserialize::default(),
                    created: Deserialize::default(),
                    currency: Deserialize::default(),
                    id: Deserialize::default(),
                    invoice: Deserialize::default(),
                    is_default: Deserialize::default(),
                    livemode: Deserialize::default(),
                    payment: Deserialize::default(),
                    status: Deserialize::default(),
                    status_transitions: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_paid" => Deserialize::begin(&mut self.builder.amount_paid),
                "amount_requested" => Deserialize::begin(&mut self.builder.amount_requested),
                "created" => Deserialize::begin(&mut self.builder.created),
                "currency" => Deserialize::begin(&mut self.builder.currency),
                "id" => Deserialize::begin(&mut self.builder.id),
                "invoice" => Deserialize::begin(&mut self.builder.invoice),
                "is_default" => Deserialize::begin(&mut self.builder.is_default),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "payment" => Deserialize::begin(&mut self.builder.payment),
                "status" => Deserialize::begin(&mut self.builder.status),
                "status_transitions" => Deserialize::begin(&mut self.builder.status_transitions),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(amount_paid),
                Some(amount_requested),
                Some(created),
                Some(currency),
                Some(id),
                Some(invoice),
                Some(is_default),
                Some(livemode),
                Some(payment),
                Some(status),
                Some(status_transitions),
            ) = (
                self.builder.amount_paid,
                self.builder.amount_requested,
                self.builder.created,
                self.builder.currency.take(),
                self.builder.id.take(),
                self.builder.invoice.take(),
                self.builder.is_default,
                self.builder.livemode,
                self.builder.payment.take(),
                self.builder.status.take(),
                self.builder.status_transitions,
            )
            else {
                return Ok(());
            };
            *self.out = Some(InvoicePayment {
                amount_paid,
                amount_requested,
                created,
                currency,
                id,
                invoice,
                is_default,
                livemode,
                payment,
                status,
                status_transitions,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for InvoicePayment {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("InvoicePayment", 12)?;
        s.serialize_field("amount_paid", &self.amount_paid)?;
        s.serialize_field("amount_requested", &self.amount_requested)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("invoice", &self.invoice)?;
        s.serialize_field("is_default", &self.is_default)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("payment", &self.payment)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("status_transitions", &self.status_transitions)?;

        s.serialize_field("object", "invoice_payment")?;
        s.end()
    }
}
impl stripe_types::Object for InvoicePayment {
    type Id = stripe_shared::InvoicePaymentId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(InvoicePaymentId);
