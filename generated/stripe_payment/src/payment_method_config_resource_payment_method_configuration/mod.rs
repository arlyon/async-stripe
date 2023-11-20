/// PaymentMethodConfigurations control which payment methods are displayed to your customers when you don't explicitly specify payment method types.
///
/// You can have multiple configurations with different sets of payment methods for different scenarios.  There are two types of PaymentMethodConfigurations.
/// Which is used depends on the [charge type](https://stripe.com/docs/connect/charges):  **Direct** configurations apply to payments created on your account, including Connect destination charges, Connect separate charges and transfers, and payments not involving Connect.  **Child** configurations apply to payments created on your connected accounts using direct charges, and charges with the on_behalf_of parameter.  Child configurations have a `parent` that sets default values and controls which settings connected accounts may override.
/// You can specify a parent ID at payment time, and Stripe will automatically resolve the connected account’s associated child configuration.
/// Parent configurations are [managed in the dashboard](https://dashboard.stripe.com/settings/payment_methods/connected_accounts) and are not available in this API.  Related guides: - [Payment Method Configurations API](https://stripe.com/docs/connect/payment-method-configurations) - [Multiple configurations on dynamic payment methods](https://stripe.com/docs/payments/multiple-payment-method-configs) - [Multiple configurations for your Connect accounts](https://stripe.com/docs/connect/multiple-payment-method-configurations).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodConfigResourcePaymentMethodConfiguration {
#[serde(skip_serializing_if = "Option::is_none")]
pub acss_debit: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    /// Whether the configuration can be used for new payments.
pub active: bool,
#[serde(skip_serializing_if = "Option::is_none")]
pub affirm: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
#[serde(skip_serializing_if = "Option::is_none")]
pub afterpay_clearpay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
#[serde(skip_serializing_if = "Option::is_none")]
pub alipay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
#[serde(skip_serializing_if = "Option::is_none")]
pub apple_pay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    /// For child configs, the Connect application associated with the configuration.
pub application: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
pub au_becs_debit: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
#[serde(skip_serializing_if = "Option::is_none")]
pub bacs_debit: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
#[serde(skip_serializing_if = "Option::is_none")]
pub bancontact: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
#[serde(skip_serializing_if = "Option::is_none")]
pub blik: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
#[serde(skip_serializing_if = "Option::is_none")]
pub boleto: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
#[serde(skip_serializing_if = "Option::is_none")]
pub card: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
#[serde(skip_serializing_if = "Option::is_none")]
pub cartes_bancaires: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
#[serde(skip_serializing_if = "Option::is_none")]
pub cashapp: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
#[serde(skip_serializing_if = "Option::is_none")]
pub eps: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
#[serde(skip_serializing_if = "Option::is_none")]
pub fpx: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
#[serde(skip_serializing_if = "Option::is_none")]
pub giropay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
#[serde(skip_serializing_if = "Option::is_none")]
pub google_pay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
#[serde(skip_serializing_if = "Option::is_none")]
pub grabpay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    /// Unique identifier for the object.
pub id: stripe_payment::payment_method_config_resource_payment_method_configuration::PaymentMethodConfigurationId,
#[serde(skip_serializing_if = "Option::is_none")]
pub id_bank_transfer: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
#[serde(skip_serializing_if = "Option::is_none")]
pub ideal: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    /// The default configuration is used whenever a payment method configuration is not specified.
pub is_default: bool,
#[serde(skip_serializing_if = "Option::is_none")]
pub jcb: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
#[serde(skip_serializing_if = "Option::is_none")]
pub klarna: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
#[serde(skip_serializing_if = "Option::is_none")]
pub konbini: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
#[serde(skip_serializing_if = "Option::is_none")]
pub link: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
pub livemode: bool,
#[serde(skip_serializing_if = "Option::is_none")]
pub multibanco: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    /// The configuration's name.
pub name: String,
#[serde(skip_serializing_if = "Option::is_none")]
pub netbanking: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
#[serde(skip_serializing_if = "Option::is_none")]
pub oxxo: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
#[serde(skip_serializing_if = "Option::is_none")]
pub p24: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    /// For child configs, the configuration's parent configuration.
pub parent: Option<String>,
#[serde(skip_serializing_if = "Option::is_none")]
pub pay_by_bank: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
#[serde(skip_serializing_if = "Option::is_none")]
pub paynow: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
#[serde(skip_serializing_if = "Option::is_none")]
pub paypal: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
#[serde(skip_serializing_if = "Option::is_none")]
pub promptpay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
#[serde(skip_serializing_if = "Option::is_none")]
pub sepa_debit: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
#[serde(skip_serializing_if = "Option::is_none")]
pub sofort: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
#[serde(skip_serializing_if = "Option::is_none")]
pub upi: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
#[serde(skip_serializing_if = "Option::is_none")]
pub us_bank_account: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
#[serde(skip_serializing_if = "Option::is_none")]
pub wechat_pay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,

}
impl stripe_types::Object for PaymentMethodConfigResourcePaymentMethodConfiguration {
    type Id = stripe_payment::payment_method_config_resource_payment_method_configuration::PaymentMethodConfigurationId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
stripe_types::def_id!(PaymentMethodConfigurationId);
#[cfg(feature = "payment_method_config_resource_payment_method_configuration")]
mod requests;
#[cfg(feature = "payment_method_config_resource_payment_method_configuration")]
pub use requests::*;
