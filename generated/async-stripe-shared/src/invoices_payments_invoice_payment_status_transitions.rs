#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoicesPaymentsInvoicePaymentStatusTransitions {
    /// The time that the payment was canceled.
    pub canceled_at: Option<stripe_types::Timestamp>,
    /// The time that the payment succeeded.
    pub paid_at: Option<stripe_types::Timestamp>,
}
#[doc(hidden)]
pub struct InvoicesPaymentsInvoicePaymentStatusTransitionsBuilder {
    canceled_at: Option<Option<stripe_types::Timestamp>>,
    paid_at: Option<Option<stripe_types::Timestamp>>,
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

    impl Deserialize for InvoicesPaymentsInvoicePaymentStatusTransitions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoicesPaymentsInvoicePaymentStatusTransitions>,
        builder: InvoicesPaymentsInvoicePaymentStatusTransitionsBuilder,
    }

    impl Visitor for Place<InvoicesPaymentsInvoicePaymentStatusTransitions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoicesPaymentsInvoicePaymentStatusTransitionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InvoicesPaymentsInvoicePaymentStatusTransitionsBuilder {
        type Out = InvoicesPaymentsInvoicePaymentStatusTransitions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "canceled_at" => Deserialize::begin(&mut self.canceled_at),
                "paid_at" => Deserialize::begin(&mut self.paid_at),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { canceled_at: Deserialize::default(), paid_at: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(canceled_at), Some(paid_at)) = (self.canceled_at, self.paid_at) else {
                return None;
            };
            Some(Self::Out { canceled_at, paid_at })
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

    impl ObjectDeser for InvoicesPaymentsInvoicePaymentStatusTransitions {
        type Builder = InvoicesPaymentsInvoicePaymentStatusTransitionsBuilder;
    }

    impl FromValueOpt for InvoicesPaymentsInvoicePaymentStatusTransitions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InvoicesPaymentsInvoicePaymentStatusTransitionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "canceled_at" => b.canceled_at = FromValueOpt::from_value(v),
                    "paid_at" => b.paid_at = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
