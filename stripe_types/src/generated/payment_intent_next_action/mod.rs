#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentIntentNextAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay_handle_redirect: Option<stripe_types::PaymentIntentNextActionAlipayHandleRedirect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto_display_details: Option<stripe_types::PaymentIntentNextActionBoleto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_await_notification: Option<stripe_types::PaymentIntentNextActionCardAwaitNotification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp_handle_redirect_or_display_qr_code: Option<stripe_types::PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_bank_transfer_instructions: Option<stripe_types::PaymentIntentNextActionDisplayBankTransferInstructions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini_display_details: Option<stripe_types::PaymentIntentNextActionKonbini>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo_display_details: Option<stripe_types::PaymentIntentNextActionDisplayOxxoDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow_display_qr_code: Option<stripe_types::PaymentIntentNextActionPaynowDisplayQrCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix_display_qr_code: Option<stripe_types::PaymentIntentNextActionPixDisplayQrCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay_display_qr_code: Option<stripe_types::PaymentIntentNextActionPromptpayDisplayQrCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_to_url: Option<stripe_types::PaymentIntentNextActionRedirectToUrl>,
    /// Type of the next action to perform, one of `redirect_to_url`, `use_stripe_sdk`, `alipay_handle_redirect`, `oxxo_display_details`, or `verify_with_microdeposits`.
    #[serde(rename = "type")]
    pub type_: String,
    /// When confirming a PaymentIntent with Stripe.js, Stripe.js depends on the contents of this dictionary to invoke authentication flows.
    ///
    /// The shape of the contents is subject to change and is only intended to be used by Stripe.js.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_stripe_sdk: Option<PaymentIntentNextActionUseStripeSdk>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_with_microdeposits: Option<stripe_types::PaymentIntentNextActionVerifyWithMicrodeposits>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay_display_qr_code: Option<stripe_types::PaymentIntentNextActionWechatPayDisplayQrCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay_redirect_to_android_app: Option<stripe_types::PaymentIntentNextActionWechatPayRedirectToAndroidApp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay_redirect_to_ios_app: Option<stripe_types::PaymentIntentNextActionWechatPayRedirectToIosApp>,
}
/// When confirming a PaymentIntent with Stripe.js, Stripe.js depends on the contents of this dictionary to invoke authentication flows.
///
/// The shape of the contents is subject to change and is only intended to be used by Stripe.js.
#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentIntentNextActionUseStripeSdk {}
