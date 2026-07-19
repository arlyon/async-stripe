#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentNextActionDisplayMultibancoDetails {
    /// Entity number associated with this Multibanco payment.
    pub entity: Option<String>,
    /// The timestamp at which the Multibanco voucher expires.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// The URL for the hosted Multibanco voucher page, which allows customers to view a Multibanco voucher.
    pub hosted_voucher_url: Option<String>,
    /// Reference number associated with this Multibanco payment.
    pub reference: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentIntentNextActionDisplayMultibancoDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentIntentNextActionDisplayMultibancoDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentIntentNextActionDisplayMultibancoDetailsBuilder {
    entity: Option<Option<String>>,
    expires_at: Option<Option<stripe_types::Timestamp>>,
    hosted_voucher_url: Option<Option<String>>,
    reference: Option<Option<String>>,
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

    impl Deserialize for PaymentIntentNextActionDisplayMultibancoDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionDisplayMultibancoDetails>,
        builder: PaymentIntentNextActionDisplayMultibancoDetailsBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionDisplayMultibancoDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentNextActionDisplayMultibancoDetailsBuilder {
                    entity: Deserialize::default(),
                    expires_at: Deserialize::default(),
                    hosted_voucher_url: Deserialize::default(),
                    reference: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "entity" => Deserialize::begin(&mut self.builder.entity),
                "expires_at" => Deserialize::begin(&mut self.builder.expires_at),
                "hosted_voucher_url" => Deserialize::begin(&mut self.builder.hosted_voucher_url),
                "reference" => Deserialize::begin(&mut self.builder.reference),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(entity), Some(expires_at), Some(hosted_voucher_url), Some(reference)) = (
                self.builder.entity.take(),
                self.builder.expires_at,
                self.builder.hosted_voucher_url.take(),
                self.builder.reference.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(PaymentIntentNextActionDisplayMultibancoDetails {
                entity,
                expires_at,
                hosted_voucher_url,
                reference,
            });
            Ok(())
        }
    }
};
