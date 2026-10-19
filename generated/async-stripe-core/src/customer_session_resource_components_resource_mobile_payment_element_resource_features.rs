/// This hash contains the features the mobile payment element supports.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeatures {
    /// A list of [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) values that controls which saved payment methods the mobile payment element displays by filtering to only show payment methods with an `allow_redisplay` value that is present in this list.
    ///
    /// If not specified, defaults to ["always"].
    /// In order to display all saved payment methods, specify ["always", "limited", "unspecified"].
    pub payment_method_allow_redisplay_filters: Option<Vec<stripe_shared::CardAllowRedisplay>>,
    /// Controls whether or not the mobile payment element shows saved payment methods.
    pub payment_method_redisplay: Option<stripe_shared::BalanceSettingsResourcePayoutsStatus>,
    /// Controls whether the mobile payment element displays the option to remove a saved payment method."
    ///
    /// Allowing buyers to remove their saved payment methods impacts subscriptions that depend on that payment method.
    /// Removing the payment method detaches the [`customer` object](https://docs.stripe.com/api/payment_methods/object#payment_method_object-customer) from that [PaymentMethod](https://docs.stripe.com/api/payment_methods).
    pub payment_method_remove: Option<stripe_shared::BalanceSettingsResourcePayoutsStatus>,
    /// Controls whether the mobile payment element displays a checkbox offering to save a new payment method.
    ///
    /// If a customer checks the box, the [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) value on the PaymentMethod is set to `'always'` at confirmation time.
    /// For PaymentIntents, the [`setup_future_usage`](https://docs.stripe.com/api/payment_intents/object#payment_intent_object-setup_future_usage) value is also set to the value defined in `payment_method_save_usage`.
    pub payment_method_save: Option<stripe_shared::BalanceSettingsResourcePayoutsStatus>,
    /// Allows overriding the value of allow_override when saving a new payment method when payment_method_save is set to disabled.
    /// Use values: "always", "limited", or "unspecified".
    ///
    /// If not specified, defaults to `nil` (no override value).
    pub payment_method_save_allow_redisplay_override: Option<stripe_shared::CardAllowRedisplay>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeatures
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(
            "CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeatures",
        )
        .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesBuilder {
    payment_method_allow_redisplay_filters: Option<Option<Vec<stripe_shared::CardAllowRedisplay>>>,
    payment_method_redisplay: Option<Option<stripe_shared::BalanceSettingsResourcePayoutsStatus>>,
    payment_method_remove: Option<Option<stripe_shared::BalanceSettingsResourcePayoutsStatus>>,
    payment_method_save: Option<Option<stripe_shared::BalanceSettingsResourcePayoutsStatus>>,
    payment_method_save_allow_redisplay_override: Option<Option<stripe_shared::CardAllowRedisplay>>,
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

    impl Deserialize for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeatures {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<
            CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeatures,
        >,
        builder:
            CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesBuilder,
    }

    impl Visitor
        for Place<CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeatures>
    {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder
        for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesBuilder
    {
        type Out = CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeatures;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "payment_method_allow_redisplay_filters" => {
                    Deserialize::begin(&mut self.payment_method_allow_redisplay_filters)
                }
                "payment_method_redisplay" => {
                    Deserialize::begin(&mut self.payment_method_redisplay)
                }
                "payment_method_remove" => Deserialize::begin(&mut self.payment_method_remove),
                "payment_method_save" => Deserialize::begin(&mut self.payment_method_save),
                "payment_method_save_allow_redisplay_override" => {
                    Deserialize::begin(&mut self.payment_method_save_allow_redisplay_override)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                payment_method_allow_redisplay_filters: Some(None),
                payment_method_redisplay: Some(None),
                payment_method_remove: Some(None),
                payment_method_save: Some(None),
                payment_method_save_allow_redisplay_override: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(payment_method_allow_redisplay_filters),
                Some(payment_method_redisplay),
                Some(payment_method_remove),
                Some(payment_method_save),
                Some(payment_method_save_allow_redisplay_override),
            ) = (
                self.payment_method_allow_redisplay_filters.take(),
                self.payment_method_redisplay.take(),
                self.payment_method_remove.take(),
                self.payment_method_save.take(),
                self.payment_method_save_allow_redisplay_override.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                payment_method_allow_redisplay_filters,
                payment_method_redisplay,
                payment_method_remove,
                payment_method_save,
                payment_method_save_allow_redisplay_override,
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

    impl ObjectDeser for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeatures {
        type Builder =
            CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesBuilder;
    }

    impl FromValueOpt
        for CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeatures
    {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "payment_method_allow_redisplay_filters" => {
                        b.payment_method_allow_redisplay_filters = FromValueOpt::from_value(v)
                    }
                    "payment_method_redisplay" => {
                        b.payment_method_redisplay = FromValueOpt::from_value(v)
                    }
                    "payment_method_remove" => {
                        b.payment_method_remove = FromValueOpt::from_value(v)
                    }
                    "payment_method_save" => b.payment_method_save = FromValueOpt::from_value(v),
                    "payment_method_save_allow_redisplay_override" => {
                        b.payment_method_save_allow_redisplay_override = FromValueOpt::from_value(v)
                    }
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// A list of [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) values that controls which saved payment methods the mobile payment element displays by filtering to only show payment methods with an `allow_redisplay` value that is present in this list.
///
/// If not specified, defaults to ["always"].
/// In order to display all saved payment methods, specify ["always", "limited", "unspecified"].
pub type CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodAllowRedisplayFilters =
    stripe_shared::CardAllowRedisplay;
/// Controls whether or not the mobile payment element shows saved payment methods.
pub type CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodRedisplay =
    stripe_shared::BalanceSettingsResourcePayoutsStatus;
/// Controls whether the mobile payment element displays the option to remove a saved payment method."
///
/// Allowing buyers to remove their saved payment methods impacts subscriptions that depend on that payment method.
/// Removing the payment method detaches the [`customer` object](https://docs.stripe.com/api/payment_methods/object#payment_method_object-customer) from that [PaymentMethod](https://docs.stripe.com/api/payment_methods).
pub type CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodRemove =
    stripe_shared::BalanceSettingsResourcePayoutsStatus;
/// Controls whether the mobile payment element displays a checkbox offering to save a new payment method.
///
/// If a customer checks the box, the [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) value on the PaymentMethod is set to `'always'` at confirmation time.
/// For PaymentIntents, the [`setup_future_usage`](https://docs.stripe.com/api/payment_intents/object#payment_intent_object-setup_future_usage) value is also set to the value defined in `payment_method_save_usage`.
pub type CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodSave =
    stripe_shared::BalanceSettingsResourcePayoutsStatus;
/// Allows overriding the value of allow_override when saving a new payment method when payment_method_save is set to disabled.
/// Use values: "always", "limited", or "unspecified".
///
/// If not specified, defaults to `nil` (no override value).
pub type CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeaturesPaymentMethodSaveAllowRedisplayOverride =
    stripe_shared::CardAllowRedisplay;
