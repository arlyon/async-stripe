#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionAfterExpirationRecovery {
    /// Enables user redeemable promotion codes on the recovered Checkout Sessions. Defaults to `false`
    pub allow_promotion_codes: bool,
    /// If `true`, a recovery url will be generated to recover this Checkout Session if it
    /// expires before a transaction is completed. It will be attached to the
    /// Checkout Session object upon expiration.
    pub enabled: bool,
    /// The timestamp at which the recovery URL will expire.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// URL that creates a new Checkout Session when clicked that is a copy of this expired Checkout Session.
    pub url: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentPagesCheckoutSessionAfterExpirationRecovery {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentPagesCheckoutSessionAfterExpirationRecovery").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionAfterExpirationRecoveryBuilder {
    allow_promotion_codes: Option<bool>,
    enabled: Option<bool>,
    expires_at: Option<Option<stripe_types::Timestamp>>,
    url: Option<Option<String>>,
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

    impl Deserialize for PaymentPagesCheckoutSessionAfterExpirationRecovery {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionAfterExpirationRecovery>,
        builder: PaymentPagesCheckoutSessionAfterExpirationRecoveryBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionAfterExpirationRecovery> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentPagesCheckoutSessionAfterExpirationRecoveryBuilder {
                    allow_promotion_codes: Deserialize::default(),
                    enabled: Deserialize::default(),
                    expires_at: Deserialize::default(),
                    url: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "allow_promotion_codes" => {
                    Deserialize::begin(&mut self.builder.allow_promotion_codes)
                }
                "enabled" => Deserialize::begin(&mut self.builder.enabled),
                "expires_at" => Deserialize::begin(&mut self.builder.expires_at),
                "url" => Deserialize::begin(&mut self.builder.url),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(allow_promotion_codes), Some(enabled), Some(expires_at), Some(url)) = (
                self.builder.allow_promotion_codes,
                self.builder.enabled,
                self.builder.expires_at,
                self.builder.url.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(PaymentPagesCheckoutSessionAfterExpirationRecovery {
                allow_promotion_codes,
                enabled,
                expires_at,
                url,
            });
            Ok(())
        }
    }
};
