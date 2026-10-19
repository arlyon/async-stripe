#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionSavedPaymentMethodOptions {
    /// Uses the `allow_redisplay` value of each saved payment method to filter the set presented to a returning customer.
    /// By default, only saved payment methods with ’allow_redisplay: ‘always’ are shown in Checkout.
    pub allow_redisplay_filters: Option<Vec<stripe_shared::CardAllowRedisplay>>,
    /// Enable customers to choose if they wish to remove their saved payment methods. Disabled by default.
    pub payment_method_remove: Option<stripe_shared::BalanceSettingsResourcePayoutsStatus>,
    /// Enable customers to choose if they wish to save their payment method for future use.
    /// Disabled by default.
    pub payment_method_save: Option<stripe_shared::BalanceSettingsResourcePayoutsStatus>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentPagesCheckoutSessionSavedPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentPagesCheckoutSessionSavedPaymentMethodOptions")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionSavedPaymentMethodOptionsBuilder {
    allow_redisplay_filters: Option<Option<Vec<stripe_shared::CardAllowRedisplay>>>,
    payment_method_remove: Option<Option<stripe_shared::BalanceSettingsResourcePayoutsStatus>>,
    payment_method_save: Option<Option<stripe_shared::BalanceSettingsResourcePayoutsStatus>>,
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

    impl Deserialize for PaymentPagesCheckoutSessionSavedPaymentMethodOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionSavedPaymentMethodOptions>,
        builder: PaymentPagesCheckoutSessionSavedPaymentMethodOptionsBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionSavedPaymentMethodOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentPagesCheckoutSessionSavedPaymentMethodOptionsBuilder::deser_default(
                ),
            }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsBuilder {
        type Out = PaymentPagesCheckoutSessionSavedPaymentMethodOptions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "allow_redisplay_filters" => Deserialize::begin(&mut self.allow_redisplay_filters),
                "payment_method_remove" => Deserialize::begin(&mut self.payment_method_remove),
                "payment_method_save" => Deserialize::begin(&mut self.payment_method_save),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                allow_redisplay_filters: Some(None),
                payment_method_remove: Some(None),
                payment_method_save: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(allow_redisplay_filters),
                Some(payment_method_remove),
                Some(payment_method_save),
            ) = (
                self.allow_redisplay_filters.take(),
                self.payment_method_remove.take(),
                self.payment_method_save.take(),
            )
            else {
                return None;
            };
            Some(Self::Out { allow_redisplay_filters, payment_method_remove, payment_method_save })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for PaymentPagesCheckoutSessionSavedPaymentMethodOptions {
        type Builder = PaymentPagesCheckoutSessionSavedPaymentMethodOptionsBuilder;
    }

    impl FromValueOpt for PaymentPagesCheckoutSessionSavedPaymentMethodOptions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                PaymentPagesCheckoutSessionSavedPaymentMethodOptionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "allow_redisplay_filters" => {
                        b.allow_redisplay_filters = FromValueOpt::from_value(v)
                    }
                    "payment_method_remove" => {
                        b.payment_method_remove = FromValueOpt::from_value(v)
                    }
                    "payment_method_save" => b.payment_method_save = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Uses the `allow_redisplay` value of each saved payment method to filter the set presented to a returning customer.
/// By default, only saved payment methods with ’allow_redisplay: ‘always’ are shown in Checkout.
pub type PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFilters =
    stripe_shared::CardAllowRedisplay;
/// Enable customers to choose if they wish to remove their saved payment methods. Disabled by default.
pub type PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove =
    stripe_shared::BalanceSettingsResourcePayoutsStatus;
/// Enable customers to choose if they wish to save their payment method for future use.
/// Disabled by default.
pub type PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodSave =
    stripe_shared::BalanceSettingsResourcePayoutsStatus;
