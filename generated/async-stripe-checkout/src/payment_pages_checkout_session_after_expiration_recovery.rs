#[derive(Clone, Debug)]
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
                builder: PaymentPagesCheckoutSessionAfterExpirationRecoveryBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionAfterExpirationRecoveryBuilder {
        type Out = PaymentPagesCheckoutSessionAfterExpirationRecovery;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "allow_promotion_codes" => Deserialize::begin(&mut self.allow_promotion_codes),
                "enabled" => Deserialize::begin(&mut self.enabled),
                "expires_at" => Deserialize::begin(&mut self.expires_at),
                "url" => Deserialize::begin(&mut self.url),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                allow_promotion_codes: Deserialize::default(),
                enabled: Deserialize::default(),
                expires_at: Deserialize::default(),
                url: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(allow_promotion_codes), Some(enabled), Some(expires_at), Some(url)) =
                (self.allow_promotion_codes, self.enabled, self.expires_at, self.url.take())
            else {
                return None;
            };
            Some(Self::Out { allow_promotion_codes, enabled, expires_at, url })
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

    impl ObjectDeser for PaymentPagesCheckoutSessionAfterExpirationRecovery {
        type Builder = PaymentPagesCheckoutSessionAfterExpirationRecoveryBuilder;
    }

    impl FromValueOpt for PaymentPagesCheckoutSessionAfterExpirationRecovery {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentPagesCheckoutSessionAfterExpirationRecoveryBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "allow_promotion_codes" => {
                        b.allow_promotion_codes = FromValueOpt::from_value(v)
                    }
                    "enabled" => b.enabled = FromValueOpt::from_value(v),
                    "expires_at" => b.expires_at = FromValueOpt::from_value(v),
                    "url" => b.url = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
