#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SetupIntentNextAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp_handle_redirect_or_display_qr_code:
        Option<stripe_shared::PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_to_url: Option<stripe_shared::SetupIntentNextActionRedirectToUrl>,
    /// Type of the next action to perform, one of `redirect_to_url`, `use_stripe_sdk`, `alipay_handle_redirect`, `oxxo_display_details`, or `verify_with_microdeposits`.
    #[serde(rename = "type")]
    pub type_: String,
    /// When confirming a SetupIntent with Stripe.js, Stripe.js depends on the contents of this dictionary to invoke authentication flows.
    /// The shape of the contents is subject to change and is only intended to be used by Stripe.js.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_stripe_sdk: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_with_microdeposits:
        Option<stripe_shared::SetupIntentNextActionVerifyWithMicrodeposits>,
}
