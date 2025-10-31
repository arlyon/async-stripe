#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodOptionsWechatPay {
    /// The app ID registered with WeChat Pay. Only required when client is ios or android.
    pub app_id: Option<String>,
    /// The client type that the end customer will pay from
    pub client: Option<PaymentMethodOptionsWechatPayClient>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    pub setup_future_usage: Option<PaymentMethodOptionsWechatPaySetupFutureUsage>,
}
#[doc(hidden)]
pub struct PaymentMethodOptionsWechatPayBuilder {
    app_id: Option<Option<String>>,
    client: Option<Option<PaymentMethodOptionsWechatPayClient>>,
    setup_future_usage: Option<Option<PaymentMethodOptionsWechatPaySetupFutureUsage>>,
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
            Self {
                app_id: Deserialize::default(),
                client: Deserialize::default(),
                setup_future_usage: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(app_id), Some(client), Some(setup_future_usage)) =
                (self.app_id.take(), self.client, self.setup_future_usage)
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
            *self.out = self.builder.take_out();
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodOptionsWechatPayClient {
    Android,
    Ios,
    Web,
}
impl PaymentMethodOptionsWechatPayClient {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodOptionsWechatPayClient::*;
        match self {
            Android => "android",
            Ios => "ios",
            Web => "web",
        }
    }
}

impl std::str::FromStr for PaymentMethodOptionsWechatPayClient {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodOptionsWechatPayClient::*;
        match s {
            "android" => Ok(Android),
            "ios" => Ok(Ios),
            "web" => Ok(Web),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentMethodOptionsWechatPayClient {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodOptionsWechatPayClient {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodOptionsWechatPayClient {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentMethodOptionsWechatPayClient {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentMethodOptionsWechatPayClient> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(PaymentMethodOptionsWechatPayClient::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodOptionsWechatPayClient);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodOptionsWechatPayClient {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PaymentMethodOptionsWechatPayClient")
        })
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodOptionsWechatPaySetupFutureUsage {
    None,
}
impl PaymentMethodOptionsWechatPaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodOptionsWechatPaySetupFutureUsage::*;
        match self {
            None => "none",
        }
    }
}

impl std::str::FromStr for PaymentMethodOptionsWechatPaySetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodOptionsWechatPaySetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentMethodOptionsWechatPaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodOptionsWechatPaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodOptionsWechatPaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentMethodOptionsWechatPaySetupFutureUsage {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentMethodOptionsWechatPaySetupFutureUsage> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentMethodOptionsWechatPaySetupFutureUsage::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodOptionsWechatPaySetupFutureUsage);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodOptionsWechatPaySetupFutureUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentMethodOptionsWechatPaySetupFutureUsage",
            )
        })
    }
}
