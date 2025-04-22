/// The invoice payment object
#[derive(Clone, Debug)]
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
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    pub payment: stripe_shared::InvoicesPaymentsInvoicePaymentAssociatedPayment,
    /// The status of the payment, one of `open`, `paid`, or `canceled`.
    pub status: String,
    pub status_transitions: stripe_shared::InvoicesPaymentsInvoicePaymentStatusTransitions,
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
                builder: InvoicePaymentBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InvoicePaymentBuilder {
        type Out = InvoicePayment;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_paid" => Deserialize::begin(&mut self.amount_paid),
                "amount_requested" => Deserialize::begin(&mut self.amount_requested),
                "created" => Deserialize::begin(&mut self.created),
                "currency" => Deserialize::begin(&mut self.currency),
                "id" => Deserialize::begin(&mut self.id),
                "invoice" => Deserialize::begin(&mut self.invoice),
                "is_default" => Deserialize::begin(&mut self.is_default),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "payment" => Deserialize::begin(&mut self.payment),
                "status" => Deserialize::begin(&mut self.status),
                "status_transitions" => Deserialize::begin(&mut self.status_transitions),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
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
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
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
                self.amount_paid,
                self.amount_requested,
                self.created,
                self.currency,
                self.id.take(),
                self.invoice.take(),
                self.is_default,
                self.livemode,
                self.payment.take(),
                self.status.take(),
                self.status_transitions,
            )
            else {
                return None;
            };
            Some(Self::Out {
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

    impl ObjectDeser for InvoicePayment {
        type Builder = InvoicePaymentBuilder;
    }

    impl FromValueOpt for InvoicePayment {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InvoicePaymentBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount_paid" => b.amount_paid = FromValueOpt::from_value(v),
                    "amount_requested" => b.amount_requested = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "invoice" => b.invoice = FromValueOpt::from_value(v),
                    "is_default" => b.is_default = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "payment" => b.payment = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    "status_transitions" => b.status_transitions = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
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
