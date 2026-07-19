#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsPaymentRecordSwish {
    /// Uniquely identifies the payer's Swish account.
    /// You can use this attribute to check whether two Swish transactions were paid for by the same payer.
    pub fingerprint: Option<String>,
    /// Payer bank reference number for the payment
    pub payment_reference: Option<String>,
    /// The last four digits of the Swish account phone number
    pub verified_phone_last4: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDetailsPaymentRecordSwish {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodDetailsPaymentRecordSwish").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodDetailsPaymentRecordSwishBuilder {
    fingerprint: Option<Option<String>>,
    payment_reference: Option<Option<String>>,
    verified_phone_last4: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodDetailsPaymentRecordSwish {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsPaymentRecordSwish>,
        builder: PaymentMethodDetailsPaymentRecordSwishBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsPaymentRecordSwish> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsPaymentRecordSwishBuilder {
                    fingerprint: Deserialize::default(),
                    payment_reference: Deserialize::default(),
                    verified_phone_last4: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "fingerprint" => Deserialize::begin(&mut self.builder.fingerprint),
                "payment_reference" => Deserialize::begin(&mut self.builder.payment_reference),
                "verified_phone_last4" => {
                    Deserialize::begin(&mut self.builder.verified_phone_last4)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(fingerprint), Some(payment_reference), Some(verified_phone_last4)) = (
                self.builder.fingerprint.take(),
                self.builder.payment_reference.take(),
                self.builder.verified_phone_last4.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(PaymentMethodDetailsPaymentRecordSwish {
                fingerprint,
                payment_reference,
                verified_phone_last4,
            });
            Ok(())
        }
    }
};
