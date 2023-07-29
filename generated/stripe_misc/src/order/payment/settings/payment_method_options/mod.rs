#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodOptions {
#[serde(skip_serializing_if = "Option::is_none")]
pub acss_debit: Option<stripe_types::payment_intent::payment_method_options::acss_debit::AcssDebit>,
#[serde(skip_serializing_if = "Option::is_none")]
pub afterpay_clearpay: Option<stripe_misc::order::payment::settings::payment_method_options::afterpay_clearpay::AfterpayClearpay>,
#[serde(skip_serializing_if = "Option::is_none")]
pub alipay: Option<stripe_types::payment_intent::payment_method_options::alipay::Alipay>,
#[serde(skip_serializing_if = "Option::is_none")]
pub bancontact: Option<stripe_types::payment_intent::payment_method_options::bancontact::Bancontact>,
#[serde(skip_serializing_if = "Option::is_none")]
pub card: Option<stripe_misc::order::payment::settings::payment_method_options::card::Card>,
#[serde(skip_serializing_if = "Option::is_none")]
pub customer_balance: Option<stripe_types::payment_intent::payment_method_options::customer_balance::CustomerBalance>,
#[serde(skip_serializing_if = "Option::is_none")]
pub ideal: Option<stripe_types::payment_intent::payment_method_options::ideal::Ideal>,
#[serde(skip_serializing_if = "Option::is_none")]
pub klarna: Option<stripe_types::payment_intent::payment_method_options::klarna::Klarna>,
#[serde(skip_serializing_if = "Option::is_none")]
pub link: Option<stripe_types::payment_intent::payment_method_options::link::Link>,
#[serde(skip_serializing_if = "Option::is_none")]
pub oxxo: Option<stripe_types::payment_intent::payment_method_options::oxxo::Oxxo>,
#[serde(skip_serializing_if = "Option::is_none")]
pub p24: Option<stripe_types::payment_intent::payment_method_options::p24::P24>,
#[serde(skip_serializing_if = "Option::is_none")]
pub paypal: Option<stripe_types::payment_intent::payment_method_options::paypal::Paypal>,
#[serde(skip_serializing_if = "Option::is_none")]
pub sepa_debit: Option<stripe_types::payment_intent::payment_method_options::sepa_debit::SepaDebit>,
#[serde(skip_serializing_if = "Option::is_none")]
pub sofort: Option<stripe_types::payment_intent::payment_method_options::sofort::Sofort>,
#[serde(skip_serializing_if = "Option::is_none")]
pub wechat_pay: Option<stripe_types::payment_intent::payment_method_options::wechat_pay::WechatPay>,

}
pub mod afterpay_clearpay;
pub use afterpay_clearpay::AfterpayClearpay;
pub mod card;
pub use card::Card;
