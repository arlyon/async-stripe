// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_method_options_wechat_pay".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodOptionsWechatPay {
    /// The app ID registered with WeChat Pay.
    ///
    /// Only required when client is ios or android.
    pub app_id: Option<String>,

    /// The client type that the end customer will pay from.
    pub client: Option<PaymentMethodOptionsWechatPayClient>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<PaymentMethodOptionsWechatPaySetupFutureUsage>,
}

/// An enum representing the possible values of an `PaymentMethodOptionsWechatPay`'s `client` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsWechatPayClient {
    Android,
    Ios,
    Web,
}

impl PaymentMethodOptionsWechatPayClient {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodOptionsWechatPayClient::Android => "android",
            PaymentMethodOptionsWechatPayClient::Ios => "ios",
            PaymentMethodOptionsWechatPayClient::Web => "web",
        }
    }
}

impl AsRef<str> for PaymentMethodOptionsWechatPayClient {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodOptionsWechatPayClient {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodOptionsWechatPayClient {
    fn default() -> Self {
        Self::Android
    }
}

/// An enum representing the possible values of an `PaymentMethodOptionsWechatPay`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsWechatPaySetupFutureUsage {
    None,
}

impl PaymentMethodOptionsWechatPaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodOptionsWechatPaySetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for PaymentMethodOptionsWechatPaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodOptionsWechatPaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodOptionsWechatPaySetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}
