#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct NextAction {
#[serde(skip_serializing_if = "Option::is_none")]
pub alipay_handle_redirect: Option<stripe_types::payment_intent::next_action_alipay_handle_redirect::NextActionAlipayHandleRedirect>,
#[serde(skip_serializing_if = "Option::is_none")]
pub boleto_display_details: Option<stripe_types::payment_intent::next_action_display_boleto_details::NextActionDisplayBoletoDetails>,
#[serde(skip_serializing_if = "Option::is_none")]
pub card_await_notification: Option<stripe_types::payment_intent::next_action_card_await_notification::NextActionCardAwaitNotification>,
#[serde(skip_serializing_if = "Option::is_none")]
pub display_bank_transfer_instructions: Option<stripe_types::payment_intent::next_action_display_bank_transfer_instructions::NextActionDisplayBankTransferInstructions>,
#[serde(skip_serializing_if = "Option::is_none")]
pub konbini_display_details: Option<stripe_types::payment_intent::next_action_konbini_display_details::NextActionKonbiniDisplayDetails>,
#[serde(skip_serializing_if = "Option::is_none")]
pub oxxo_display_details: Option<stripe_types::payment_intent::next_action_oxxo_display_details::NextActionOxxoDisplayDetails>,
#[serde(skip_serializing_if = "Option::is_none")]
pub paynow_display_qr_code: Option<stripe_types::payment_intent::next_action::paynow_display_qr_code::PaynowDisplayQrCode>,
#[serde(skip_serializing_if = "Option::is_none")]
pub pix_display_qr_code: Option<stripe_types::payment_intent::next_action::pix_display_qr_code::PixDisplayQrCode>,
#[serde(skip_serializing_if = "Option::is_none")]
pub promptpay_display_qr_code: Option<stripe_types::payment_intent::next_action::promptpay_display_qr_code::PromptpayDisplayQrCode>,
#[serde(skip_serializing_if = "Option::is_none")]
pub redirect_to_url: Option<stripe_types::payment_intent::next_action_redirect_to_url::NextActionRedirectToUrl>,
    /// Type of the next action to perform, one of `redirect_to_url`, `use_stripe_sdk`, `alipay_handle_redirect`, `oxxo_display_details`, or `verify_with_microdeposits`.
#[serde(rename = "type")]
pub type_: String,
    /// When confirming a PaymentIntent with Stripe.js, Stripe.js depends on the contents of this dictionary to invoke authentication flows.
    ///
    /// The shape of the contents is subject to change and is only intended to be used by Stripe.js.
#[serde(skip_serializing_if = "Option::is_none")]
pub use_stripe_sdk: Option<NextActionUseStripeSdk>,
#[serde(skip_serializing_if = "Option::is_none")]
pub verify_with_microdeposits: Option<stripe_types::payment_intent::next_action::verify_with_microdeposits::VerifyWithMicrodeposits>,
#[serde(skip_serializing_if = "Option::is_none")]
pub wechat_pay_display_qr_code: Option<stripe_types::payment_intent::next_action::wechat_pay_display_qr_code::WechatPayDisplayQrCode>,
#[serde(skip_serializing_if = "Option::is_none")]
pub wechat_pay_redirect_to_android_app: Option<stripe_types::payment_intent::next_action::wechat_pay_redirect_to_android_app::WechatPayRedirectToAndroidApp>,
#[serde(skip_serializing_if = "Option::is_none")]
pub wechat_pay_redirect_to_ios_app: Option<stripe_types::payment_intent::next_action::wechat_pay_redirect_to_ios_app::WechatPayRedirectToIosApp>,

}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for NextAction {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// When confirming a PaymentIntent with Stripe.js, Stripe.js depends on the contents of this dictionary to invoke authentication flows.
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

pub mod paynow_display_qr_code;
pub use paynow_display_qr_code::PaynowDisplayQrCode;
pub mod pix_display_qr_code;
pub use pix_display_qr_code::PixDisplayQrCode;
pub mod promptpay_display_qr_code;
pub use promptpay_display_qr_code::PromptpayDisplayQrCode;
pub mod verify_with_microdeposits;
pub use verify_with_microdeposits::VerifyWithMicrodeposits;
pub mod wechat_pay_display_qr_code;
pub use wechat_pay_display_qr_code::WechatPayDisplayQrCode;
pub mod wechat_pay_redirect_to_android_app;
pub use wechat_pay_redirect_to_android_app::WechatPayRedirectToAndroidApp;
pub mod wechat_pay_redirect_to_ios_app;
pub use wechat_pay_redirect_to_ios_app::WechatPayRedirectToIosApp;
