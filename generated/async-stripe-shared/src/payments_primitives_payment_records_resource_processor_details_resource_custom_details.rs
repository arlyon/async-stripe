/// Custom processors represent payment processors not modeled directly in
/// the Stripe API. This resource consists of details about the custom processor
/// used for this payment attempt.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsResourceCustomDetails {
    /// An opaque string for manual reconciliation of this payment, for example a check number or a payment processor ID.
    pub payment_reference: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsResourceCustomDetails
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(
            "PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsResourceCustomDetails",
        )
        .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsResourceCustomDetailsBuilder {
    payment_reference: Option<Option<String>>,
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

    impl Deserialize for PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsResourceCustomDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<
            PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsResourceCustomDetails,
        >,
        builder:
            PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsResourceCustomDetailsBuilder,
    }

    impl Visitor
        for Place<PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsResourceCustomDetails>
    {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsResourceCustomDetailsBuilder { payment_reference: Deserialize::default(),
 },
        }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "payment_reference" => Deserialize::begin(&mut self.builder.payment_reference),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(payment_reference),) = (self.builder.payment_reference.take(),) else {
                return Ok(());
            };
            *self.out = Some(
                PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsResourceCustomDetails {
                    payment_reference,
                },
            );
            Ok(())
        }
    }
};
