#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodOptionsWechatPay {
    /// The app ID registered with WeChat Pay. Only required when client is ios, android, or mini_program.
    pub app_id: Option<String>,
    /// The client type that the end customer will pay from
    pub client: Option<stripe_shared::Client>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    pub setup_future_usage: Option<stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodOptionsWechatPay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodOptionsWechatPay").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodOptionsWechatPayBuilder {
    app_id: Option<Option<String>>,
    client: Option<Option<stripe_shared::Client>>,
    setup_future_usage: Option<Option<stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage>>,
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

    impl Deserialize for PaymentMethodOptionsWechatPay {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodOptionsWechatPay>,
        builder: PaymentMethodOptionsWechatPayBuilder,
    }

    impl Visitor for Place<PaymentMethodOptionsWechatPay> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodOptionsWechatPayBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodOptionsWechatPayBuilder {
        type Out = PaymentMethodOptionsWechatPay;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "app_id" => Deserialize::begin(&mut self.app_id),
                "client" => Deserialize::begin(&mut self.client),
                "setup_future_usage" => Deserialize::begin(&mut self.setup_future_usage),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { app_id: Some(None), client: Some(None), setup_future_usage: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(app_id), Some(client), Some(setup_future_usage)) =
                (self.app_id.take(), self.client.take(), self.setup_future_usage.take())
            else {
                return None;
            };
            Some(Self::Out { app_id, client, setup_future_usage })
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

    impl ObjectDeser for PaymentMethodOptionsWechatPay {
        type Builder = PaymentMethodOptionsWechatPayBuilder;
    }

    impl FromValueOpt for PaymentMethodOptionsWechatPay {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodOptionsWechatPayBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "app_id" => b.app_id = FromValueOpt::from_value(v),
                    "client" => b.client = FromValueOpt::from_value(v),
                    "setup_future_usage" => b.setup_future_usage = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The client type that the end customer will pay from
pub type PaymentMethodOptionsWechatPayClient = stripe_shared::Client;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type PaymentMethodOptionsWechatPaySetupFutureUsage =
    stripe_shared::PaymentMethodOptionsFpxSetupFutureUsage;
