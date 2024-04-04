#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentIntentNextAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay_handle_redirect: Option<stripe_shared::PaymentIntentNextActionAlipayHandleRedirect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto_display_details: Option<stripe_shared::PaymentIntentNextActionBoleto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_await_notification:
        Option<stripe_shared::PaymentIntentNextActionCardAwaitNotification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp_handle_redirect_or_display_qr_code:
        Option<stripe_shared::PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_bank_transfer_instructions:
        Option<stripe_shared::PaymentIntentNextActionDisplayBankTransferInstructions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini_display_details: Option<stripe_shared::PaymentIntentNextActionKonbini>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo_display_details: Option<stripe_shared::PaymentIntentNextActionDisplayOxxoDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow_display_qr_code: Option<stripe_shared::PaymentIntentNextActionPaynowDisplayQrCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix_display_qr_code: Option<stripe_shared::PaymentIntentNextActionPixDisplayQrCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay_display_qr_code:
        Option<stripe_shared::PaymentIntentNextActionPromptpayDisplayQrCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_to_url: Option<stripe_shared::PaymentIntentNextActionRedirectToUrl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swish_handle_redirect_or_display_qr_code:
        Option<stripe_shared::PaymentIntentNextActionSwishHandleRedirectOrDisplayQrCode>,
    /// Type of the next action to perform, one of `redirect_to_url`, `use_stripe_sdk`, `alipay_handle_redirect`, `oxxo_display_details`, or `verify_with_microdeposits`.
    #[serde(rename = "type")]
    pub type_: String,
    /// When confirming a PaymentIntent with Stripe.js, Stripe.js depends on the contents of this dictionary to invoke authentication flows.
    /// The shape of the contents is subject to change and is only intended to be used by Stripe.js.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_stripe_sdk: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_with_microdeposits:
        Option<stripe_shared::PaymentIntentNextActionVerifyWithMicrodeposits>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay_display_qr_code:
        Option<stripe_shared::PaymentIntentNextActionWechatPayDisplayQrCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay_redirect_to_android_app:
        Option<stripe_shared::PaymentIntentNextActionWechatPayRedirectToAndroidApp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay_redirect_to_ios_app:
        Option<stripe_shared::PaymentIntentNextActionWechatPayRedirectToIosApp>,
}
