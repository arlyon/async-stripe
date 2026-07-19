#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentNextActionKonbini {
    /// The timestamp at which the pending Konbini payment expires.
    pub expires_at: stripe_types::Timestamp,
    /// The URL for the Konbini payment instructions page, which allows customers to view and print a Konbini voucher.
    pub hosted_voucher_url: Option<String>,
    pub stores: stripe_shared::PaymentIntentNextActionKonbiniStores,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentIntentNextActionKonbini {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentIntentNextActionKonbini").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentIntentNextActionKonbiniBuilder {
    expires_at: Option<stripe_types::Timestamp>,
    hosted_voucher_url: Option<Option<String>>,
    stores: Option<stripe_shared::PaymentIntentNextActionKonbiniStores>,
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

    impl Deserialize for PaymentIntentNextActionKonbini {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionKonbini>,
        builder: PaymentIntentNextActionKonbiniBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionKonbini> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentNextActionKonbiniBuilder {
                    expires_at: Deserialize::default(),
                    hosted_voucher_url: Deserialize::default(),
                    stores: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "expires_at" => Deserialize::begin(&mut self.builder.expires_at),
                "hosted_voucher_url" => Deserialize::begin(&mut self.builder.hosted_voucher_url),
                "stores" => Deserialize::begin(&mut self.builder.stores),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(expires_at), Some(hosted_voucher_url), Some(stores)) = (
                self.builder.expires_at,
                self.builder.hosted_voucher_url.take(),
                self.builder.stores.take(),
            ) else {
                return Ok(());
            };
            *self.out =
                Some(PaymentIntentNextActionKonbini { expires_at, hosted_voucher_url, stores });
            Ok(())
        }
    }
};
