#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentNextActionDisplayOxxoDetails {
    /// The timestamp after which the OXXO voucher expires.
    pub expires_after: Option<stripe_types::Timestamp>,
    /// The URL for the hosted OXXO voucher page, which allows customers to view and print an OXXO voucher.
    pub hosted_voucher_url: Option<String>,
    /// OXXO reference number.
    pub number: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentIntentNextActionDisplayOxxoDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentIntentNextActionDisplayOxxoDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentIntentNextActionDisplayOxxoDetailsBuilder {
    expires_after: Option<Option<stripe_types::Timestamp>>,
    hosted_voucher_url: Option<Option<String>>,
    number: Option<Option<String>>,
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

    impl Deserialize for PaymentIntentNextActionDisplayOxxoDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionDisplayOxxoDetails>,
        builder: PaymentIntentNextActionDisplayOxxoDetailsBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionDisplayOxxoDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentNextActionDisplayOxxoDetailsBuilder {
                    expires_after: Deserialize::default(),
                    hosted_voucher_url: Deserialize::default(),
                    number: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "expires_after" => Deserialize::begin(&mut self.builder.expires_after),
                "hosted_voucher_url" => Deserialize::begin(&mut self.builder.hosted_voucher_url),
                "number" => Deserialize::begin(&mut self.builder.number),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(expires_after), Some(hosted_voucher_url), Some(number)) = (
                self.builder.expires_after,
                self.builder.hosted_voucher_url.take(),
                self.builder.number.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(PaymentIntentNextActionDisplayOxxoDetails {
                expires_after,
                hosted_voucher_url,
                number,
            });
            Ok(())
        }
    }
};
