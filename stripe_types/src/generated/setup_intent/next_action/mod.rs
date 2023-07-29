#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NextAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_to_url:
        Option<stripe_types::setup_intent::next_action_redirect_to_url::NextActionRedirectToUrl>,
    /// Type of the next action to perform, one of `redirect_to_url`, `use_stripe_sdk`, `alipay_handle_redirect`, `oxxo_display_details`, or `verify_with_microdeposits`.
    #[serde(rename = "type")]
    pub type_: String,
    /// When confirming a SetupIntent with Stripe.js, Stripe.js depends on the contents of this dictionary to invoke authentication flows.
    ///
    /// The shape of the contents is subject to change and is only intended to be used by Stripe.js.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_stripe_sdk: Option<NextActionUseStripeSdk>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_with_microdeposits: Option<
        stripe_types::setup_intent::next_action::verify_with_microdeposits::VerifyWithMicrodeposits,
    >,
}
/// When confirming a SetupIntent with Stripe.js, Stripe.js depends on the contents of this dictionary to invoke authentication flows.
///
/// The shape of the contents is subject to change and is only intended to be used by Stripe.js.
#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct NextActionUseStripeSdk {}
pub mod verify_with_microdeposits;
pub use verify_with_microdeposits::VerifyWithMicrodeposits;
