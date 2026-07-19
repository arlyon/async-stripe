#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoicesPaymentsInvoicePaymentStatusTransitions {
    /// The time that the payment was canceled.
    pub canceled_at: Option<stripe_types::Timestamp>,
    /// The time that the payment succeeded.
    pub paid_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoicesPaymentsInvoicePaymentStatusTransitions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoicesPaymentsInvoicePaymentStatusTransitions").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InvoicesPaymentsInvoicePaymentStatusTransitionsBuilder {
    canceled_at: Option<Option<stripe_types::Timestamp>>,
    paid_at: Option<Option<stripe_types::Timestamp>>,
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
                builder: InvoicesPaymentsInvoicePaymentStatusTransitionsBuilder {
                    canceled_at: Deserialize::default(),
                    paid_at: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "canceled_at" => Deserialize::begin(&mut self.builder.canceled_at),
                "paid_at" => Deserialize::begin(&mut self.builder.paid_at),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(canceled_at), Some(paid_at)) =
                (self.builder.canceled_at, self.builder.paid_at)
            else {
                return Ok(());
            };
            *self.out =
                Some(InvoicesPaymentsInvoicePaymentStatusTransitions { canceled_at, paid_at });
            Ok(())
        }
    }
};
