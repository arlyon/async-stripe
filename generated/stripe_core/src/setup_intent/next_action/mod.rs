#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct NextAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_to_url:
        Option<stripe_core::setup_intent::next_action_redirect_to_url::NextActionRedirectToUrl>,
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
        stripe_core::setup_intent::next_action::verify_with_microdeposits::VerifyWithMicrodeposits,
    >,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for NextAction {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// When confirming a SetupIntent with Stripe.js, Stripe.js depends on the contents of this dictionary to invoke authentication flows.
///
/// The shape of the contents is subject to change and is only intended to be used by Stripe.js.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct NextActionUseStripeSdk {}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for NextActionUseStripeSdk {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

pub mod verify_with_microdeposits;
pub use verify_with_microdeposits::VerifyWithMicrodeposits;
