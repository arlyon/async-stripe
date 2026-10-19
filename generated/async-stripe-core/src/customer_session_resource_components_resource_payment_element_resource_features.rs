/// This hash contains the features the Payment Element supports.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerSessionResourceComponentsResourcePaymentElementResourceFeatures {
    /// A list of [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) values that controls which saved payment methods the Payment Element displays by filtering to only show payment methods with an `allow_redisplay` value that is present in this list.
    ///
    /// If not specified, defaults to ["always"].
    /// In order to display all saved payment methods, specify ["always", "limited", "unspecified"].
    pub payment_method_allow_redisplay_filters: Vec<stripe_shared::CardAllowRedisplay>,
    /// Controls whether or not the Payment Element shows saved payment methods.
    /// This parameter defaults to `disabled`.
    pub payment_method_redisplay: stripe_shared::BalanceSettingsResourcePayoutsStatus,
    /// Determines the max number of saved payment methods for the Payment Element to display.
    /// This parameter defaults to `3`.
    /// The maximum redisplay limit is `10`.
    pub payment_method_redisplay_limit: Option<i64>,
    /// Controls whether the Payment Element displays the option to remove a saved payment method.
    /// This parameter defaults to `disabled`.
    ///
    /// Allowing buyers to remove their saved payment methods impacts subscriptions that depend on that payment method.
    /// Removing the payment method detaches the [`customer` object](https://docs.stripe.com/api/payment_methods/object#payment_method_object-customer) from that [PaymentMethod](https://docs.stripe.com/api/payment_methods).
    pub payment_method_remove: stripe_shared::BalanceSettingsResourcePayoutsStatus,
    /// Controls whether the Payment Element displays a checkbox offering to save a new payment method.
    /// This parameter defaults to `disabled`.
    ///
    /// If a customer checks the box, the [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) value on the PaymentMethod is set to `'always'` at confirmation time.
    /// For PaymentIntents, the [`setup_future_usage`](https://docs.stripe.com/api/payment_intents/object#payment_intent_object-setup_future_usage) value is also set to the value defined in `payment_method_save_usage`.
    pub payment_method_save: stripe_shared::BalanceSettingsResourcePayoutsStatus,
    /// When using PaymentIntents and the customer checks the save checkbox, this field determines the [`setup_future_usage`](https://docs.stripe.com/api/payment_intents/object#payment_intent_object-setup_future_usage) value used to confirm the PaymentIntent.
    ///
    /// When using SetupIntents, directly configure the [`usage`](https://docs.stripe.com/api/setup_intents/object#setup_intent_object-usage) value on SetupIntent creation.
    pub payment_method_save_usage: Option<stripe_shared::CreateSetupIntentUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CustomerSessionResourceComponentsResourcePaymentElementResourceFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CustomerSessionResourceComponentsResourcePaymentElementResourceFeatures")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesBuilder {
    payment_method_allow_redisplay_filters: Option<Vec<stripe_shared::CardAllowRedisplay>>,
    payment_method_redisplay: Option<stripe_shared::BalanceSettingsResourcePayoutsStatus>,
    payment_method_redisplay_limit: Option<Option<i64>>,
    payment_method_remove: Option<stripe_shared::BalanceSettingsResourcePayoutsStatus>,
    payment_method_save: Option<stripe_shared::BalanceSettingsResourcePayoutsStatus>,
    payment_method_save_usage: Option<Option<stripe_shared::CreateSetupIntentUsage>>,
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

    impl Deserialize for CustomerSessionResourceComponentsResourcePaymentElementResourceFeatures {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out:
            &'a mut Option<CustomerSessionResourceComponentsResourcePaymentElementResourceFeatures>,
        builder: CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesBuilder,
    }

    impl Visitor for Place<CustomerSessionResourceComponentsResourcePaymentElementResourceFeatures> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesBuilder {
        type Out = CustomerSessionResourceComponentsResourcePaymentElementResourceFeatures;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "payment_method_allow_redisplay_filters" => {
                    Deserialize::begin(&mut self.payment_method_allow_redisplay_filters)
                }
                "payment_method_redisplay" => {
                    Deserialize::begin(&mut self.payment_method_redisplay)
                }
                "payment_method_redisplay_limit" => {
                    Deserialize::begin(&mut self.payment_method_redisplay_limit)
                }
                "payment_method_remove" => Deserialize::begin(&mut self.payment_method_remove),
                "payment_method_save" => Deserialize::begin(&mut self.payment_method_save),
                "payment_method_save_usage" => {
                    Deserialize::begin(&mut self.payment_method_save_usage)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                payment_method_allow_redisplay_filters: None,
                payment_method_redisplay: None,
                payment_method_redisplay_limit: Some(None),
                payment_method_remove: None,
                payment_method_save: None,
                payment_method_save_usage: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(payment_method_allow_redisplay_filters),
                Some(payment_method_redisplay),
                Some(payment_method_redisplay_limit),
                Some(payment_method_remove),
                Some(payment_method_save),
                Some(payment_method_save_usage),
            ) = (
                self.payment_method_allow_redisplay_filters.take(),
                self.payment_method_redisplay.take(),
                self.payment_method_redisplay_limit,
                self.payment_method_remove.take(),
                self.payment_method_save.take(),
                self.payment_method_save_usage.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                payment_method_allow_redisplay_filters,
                payment_method_redisplay,
                payment_method_redisplay_limit,
                payment_method_remove,
                payment_method_save,
                payment_method_save_usage,
            })
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

    impl ObjectDeser for CustomerSessionResourceComponentsResourcePaymentElementResourceFeatures {
        type Builder =
            CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesBuilder;
    }

    impl FromValueOpt for CustomerSessionResourceComponentsResourcePaymentElementResourceFeatures {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "payment_method_allow_redisplay_filters" => {
                        b.payment_method_allow_redisplay_filters = FromValueOpt::from_value(v)
                    }
                    "payment_method_redisplay" => {
                        b.payment_method_redisplay = FromValueOpt::from_value(v)
                    }
                    "payment_method_redisplay_limit" => {
                        b.payment_method_redisplay_limit = FromValueOpt::from_value(v)
                    }
                    "payment_method_remove" => {
                        b.payment_method_remove = FromValueOpt::from_value(v)
                    }
                    "payment_method_save" => b.payment_method_save = FromValueOpt::from_value(v),
                    "payment_method_save_usage" => {
                        b.payment_method_save_usage = FromValueOpt::from_value(v)
                    }
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// A list of [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) values that controls which saved payment methods the Payment Element displays by filtering to only show payment methods with an `allow_redisplay` value that is present in this list.
///
/// If not specified, defaults to ["always"].
/// In order to display all saved payment methods, specify ["always", "limited", "unspecified"].
pub type CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters =
    stripe_shared::CardAllowRedisplay;
/// Controls whether or not the Payment Element shows saved payment methods.
/// This parameter defaults to `disabled`.
pub type CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRedisplay =
    stripe_shared::BalanceSettingsResourcePayoutsStatus;
/// Controls whether the Payment Element displays the option to remove a saved payment method.
/// This parameter defaults to `disabled`.
///
/// Allowing buyers to remove their saved payment methods impacts subscriptions that depend on that payment method.
/// Removing the payment method detaches the [`customer` object](https://docs.stripe.com/api/payment_methods/object#payment_method_object-customer) from that [PaymentMethod](https://docs.stripe.com/api/payment_methods).
pub type CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodRemove =
    stripe_shared::BalanceSettingsResourcePayoutsStatus;
/// Controls whether the Payment Element displays a checkbox offering to save a new payment method.
/// This parameter defaults to `disabled`.
///
/// If a customer checks the box, the [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) value on the PaymentMethod is set to `'always'` at confirmation time.
/// For PaymentIntents, the [`setup_future_usage`](https://docs.stripe.com/api/payment_intents/object#payment_intent_object-setup_future_usage) value is also set to the value defined in `payment_method_save_usage`.
pub type CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSave =
    stripe_shared::BalanceSettingsResourcePayoutsStatus;
/// When using PaymentIntents and the customer checks the save checkbox, this field determines the [`setup_future_usage`](https://docs.stripe.com/api/payment_intents/object#payment_intent_object-setup_future_usage) value used to confirm the PaymentIntent.
///
/// When using SetupIntents, directly configure the [`usage`](https://docs.stripe.com/api/setup_intents/object#setup_intent_object-usage) value on SetupIntent creation.
pub type CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage =
    stripe_shared::CreateSetupIntentUsage;
