/// This hash contains the features the customer sheet supports.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerSessionResourceComponentsResourceCustomerSheetResourceFeatures {
    /// A list of [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) values that controls which saved payment methods the customer sheet displays by filtering to only show payment methods with an `allow_redisplay` value that is present in this list.
    ///
    /// If not specified, defaults to ["always"].
    /// In order to display all saved payment methods, specify ["always", "limited", "unspecified"].
    pub payment_method_allow_redisplay_filters: Option<Vec<stripe_shared::CardAllowRedisplay>>,
    /// Controls whether the customer sheet displays the option to remove a saved payment method."
    ///
    /// Allowing buyers to remove their saved payment methods impacts subscriptions that depend on that payment method.
    /// Removing the payment method detaches the [`customer` object](https://docs.stripe.com/api/payment_methods/object#payment_method_object-customer) from that [PaymentMethod](https://docs.stripe.com/api/payment_methods).
    pub payment_method_remove: Option<stripe_shared::BalanceSettingsResourcePayoutsStatus>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CustomerSessionResourceComponentsResourceCustomerSheetResourceFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CustomerSessionResourceComponentsResourceCustomerSheetResourceFeatures")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesBuilder {
    payment_method_allow_redisplay_filters: Option<Option<Vec<stripe_shared::CardAllowRedisplay>>>,
    payment_method_remove: Option<Option<stripe_shared::BalanceSettingsResourcePayoutsStatus>>,
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

    impl Deserialize for CustomerSessionResourceComponentsResourceCustomerSheetResourceFeatures {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CustomerSessionResourceComponentsResourceCustomerSheetResourceFeatures>,
        builder: CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesBuilder,
    }

    impl Visitor for Place<CustomerSessionResourceComponentsResourceCustomerSheetResourceFeatures> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesBuilder {
        type Out = CustomerSessionResourceComponentsResourceCustomerSheetResourceFeatures;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "payment_method_allow_redisplay_filters" => {
                    Deserialize::begin(&mut self.payment_method_allow_redisplay_filters)
                }
                "payment_method_remove" => Deserialize::begin(&mut self.payment_method_remove),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                payment_method_allow_redisplay_filters: Some(None),
                payment_method_remove: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(payment_method_allow_redisplay_filters), Some(payment_method_remove)) = (
                self.payment_method_allow_redisplay_filters.take(),
                self.payment_method_remove.take(),
            ) else {
                return None;
            };
            Some(Self::Out { payment_method_allow_redisplay_filters, payment_method_remove })
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

    impl ObjectDeser for CustomerSessionResourceComponentsResourceCustomerSheetResourceFeatures {
        type Builder =
            CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesBuilder;
    }

    impl FromValueOpt for CustomerSessionResourceComponentsResourceCustomerSheetResourceFeatures {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "payment_method_allow_redisplay_filters" => {
                        b.payment_method_allow_redisplay_filters = FromValueOpt::from_value(v)
                    }
                    "payment_method_remove" => {
                        b.payment_method_remove = FromValueOpt::from_value(v)
                    }
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// A list of [`allow_redisplay`](https://docs.stripe.com/api/payment_methods/object#payment_method_object-allow_redisplay) values that controls which saved payment methods the customer sheet displays by filtering to only show payment methods with an `allow_redisplay` value that is present in this list.
///
/// If not specified, defaults to ["always"].
/// In order to display all saved payment methods, specify ["always", "limited", "unspecified"].
pub type CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodAllowRedisplayFilters =
    stripe_shared::CardAllowRedisplay;
/// Controls whether the customer sheet displays the option to remove a saved payment method."
///
/// Allowing buyers to remove their saved payment methods impacts subscriptions that depend on that payment method.
/// Removing the payment method detaches the [`customer` object](https://docs.stripe.com/api/payment_methods/object#payment_method_object-customer) from that [PaymentMethod](https://docs.stripe.com/api/payment_methods).
pub type CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove =
    stripe_shared::BalanceSettingsResourcePayoutsStatus;
