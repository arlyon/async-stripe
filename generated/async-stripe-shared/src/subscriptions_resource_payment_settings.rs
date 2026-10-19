#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionsResourcePaymentSettings {
    /// Payment-method-specific configuration to provide to invoices created by the subscription.
    pub payment_method_options: Option<stripe_shared::SubscriptionsResourcePaymentMethodOptions>,
    /// The list of payment method types to provide to every invoice created by the subscription.
    /// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
    pub payment_method_types: Option<Vec<stripe_shared::PaymentMethodTypes>>,
    /// Configure whether Stripe updates `subscription.default_payment_method` when payment succeeds.
    /// Defaults to `off`.
    pub save_default_payment_method: Option<stripe_shared::SaveDefaultPaymentMethod>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionsResourcePaymentSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionsResourcePaymentSettings").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionsResourcePaymentSettingsBuilder {
    payment_method_options:
        Option<Option<stripe_shared::SubscriptionsResourcePaymentMethodOptions>>,
    payment_method_types: Option<Option<Vec<stripe_shared::PaymentMethodTypes>>>,
    save_default_payment_method: Option<Option<stripe_shared::SaveDefaultPaymentMethod>>,
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

    impl Deserialize for SubscriptionsResourcePaymentSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionsResourcePaymentSettings>,
        builder: SubscriptionsResourcePaymentSettingsBuilder,
    }

    impl Visitor for Place<SubscriptionsResourcePaymentSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionsResourcePaymentSettingsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionsResourcePaymentSettingsBuilder {
        type Out = SubscriptionsResourcePaymentSettings;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "payment_method_options" => Deserialize::begin(&mut self.payment_method_options),
                "payment_method_types" => Deserialize::begin(&mut self.payment_method_types),
                "save_default_payment_method" => {
                    Deserialize::begin(&mut self.save_default_payment_method)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                payment_method_options: Some(None),
                payment_method_types: Some(None),
                save_default_payment_method: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(payment_method_options),
                Some(payment_method_types),
                Some(save_default_payment_method),
            ) = (
                self.payment_method_options.take(),
                self.payment_method_types.take(),
                self.save_default_payment_method.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                payment_method_options,
                payment_method_types,
                save_default_payment_method,
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

    impl ObjectDeser for SubscriptionsResourcePaymentSettings {
        type Builder = SubscriptionsResourcePaymentSettingsBuilder;
    }

    impl FromValueOpt for SubscriptionsResourcePaymentSettings {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionsResourcePaymentSettingsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "payment_method_options" => {
                        b.payment_method_options = FromValueOpt::from_value(v)
                    }
                    "payment_method_types" => b.payment_method_types = FromValueOpt::from_value(v),
                    "save_default_payment_method" => {
                        b.save_default_payment_method = FromValueOpt::from_value(v)
                    }
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The list of payment method types to provide to every invoice created by the subscription.
/// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
pub type SubscriptionsResourcePaymentSettingsPaymentMethodTypes = stripe_shared::PaymentMethodTypes;
/// Configure whether Stripe updates `subscription.default_payment_method` when payment succeeds.
/// Defaults to `off`.
pub type SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod =
    stripe_shared::SaveDefaultPaymentMethod;
