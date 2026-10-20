#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentNextAction {
    pub alipay_handle_redirect: Option<stripe_shared::PaymentIntentNextActionAlipayHandleRedirect>,
    pub blik_authorize: Option<stripe_shared::PaymentIntentNextActionBlikAuthorize>,
    pub boleto_display_details: Option<stripe_shared::PaymentIntentNextActionBoleto>,
    pub card_await_notification:
        Option<stripe_shared::PaymentIntentNextActionCardAwaitNotification>,
    pub cashapp_handle_redirect_or_display_qr_code:
        Option<stripe_shared::PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode>,
    pub display_bank_transfer_instructions:
        Option<stripe_shared::PaymentIntentNextActionDisplayBankTransferInstructions>,
    pub klarna_display_qr_code: Option<stripe_shared::PaymentIntentNextActionKlarnaDisplayQrCode>,
    pub konbini_display_details: Option<stripe_shared::PaymentIntentNextActionKonbini>,
    pub multibanco_display_details:
        Option<stripe_shared::PaymentIntentNextActionDisplayMultibancoDetails>,
    pub oxxo_display_details: Option<stripe_shared::PaymentIntentNextActionDisplayOxxoDetails>,
    pub paynow_display_qr_code: Option<stripe_shared::PaymentIntentNextActionPaynowDisplayQrCode>,
    pub pix_display_qr_code: Option<stripe_shared::PaymentIntentNextActionPixDisplayQrCode>,
    pub promptpay_display_qr_code:
        Option<stripe_shared::PaymentIntentNextActionPromptpayDisplayQrCode>,
    pub redirect_to_url: Option<stripe_shared::PaymentIntentNextActionRedirectToUrl>,
    pub swish_handle_redirect_or_display_qr_code:
        Option<stripe_shared::PaymentIntentNextActionSwishHandleRedirectOrDisplayQrCode>,
    /// Type of the next action to perform.
    /// Refer to the other child attributes under `next_action` for available values.
    /// Examples include: `redirect_to_url`, `use_stripe_sdk`, `alipay_handle_redirect`, `oxxo_display_details`, or `verify_with_microdeposits`.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: String,
    pub upi_handle_redirect_or_display_qr_code:
        Option<stripe_shared::PaymentIntentNextActionUpiHandleRedirectOrDisplayQrCode>,
    /// When confirming a PaymentIntent with Stripe.js, Stripe.js depends on the contents of this dictionary to invoke authentication flows.
    /// The shape of the contents is subject to change and is only intended to be used by Stripe.js.
    #[cfg_attr(
        any(feature = "deserialize", feature = "serialize"),
        serde(default, with = "stripe_types::with_serde_json_opt")
    )]
    pub use_stripe_sdk: Option<miniserde::json::Value>,
    pub verify_with_microdeposits:
        Option<stripe_shared::PaymentIntentNextActionVerifyWithMicrodeposits>,
    pub wechat_pay_display_qr_code:
        Option<stripe_shared::PaymentIntentNextActionWechatPayDisplayQrCode>,
    pub wechat_pay_redirect_to_android_app:
        Option<stripe_shared::PaymentIntentNextActionWechatPayRedirectToAndroidApp>,
    pub wechat_pay_redirect_to_ios_app:
        Option<stripe_shared::PaymentIntentNextActionWechatPayRedirectToIosApp>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentIntentNextAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentIntentNextAction").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentIntentNextActionBuilder {
    alipay_handle_redirect: Option<stripe_shared::PaymentIntentNextActionAlipayHandleRedirect>,
    blik_authorize: Option<stripe_shared::PaymentIntentNextActionBlikAuthorize>,
    boleto_display_details: Option<stripe_shared::PaymentIntentNextActionBoleto>,
    card_await_notification: Option<stripe_shared::PaymentIntentNextActionCardAwaitNotification>,
    cashapp_handle_redirect_or_display_qr_code:
        Option<stripe_shared::PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode>,
    display_bank_transfer_instructions:
        Option<stripe_shared::PaymentIntentNextActionDisplayBankTransferInstructions>,
    klarna_display_qr_code: Option<stripe_shared::PaymentIntentNextActionKlarnaDisplayQrCode>,
    konbini_display_details: Option<stripe_shared::PaymentIntentNextActionKonbini>,
    multibanco_display_details:
        Option<stripe_shared::PaymentIntentNextActionDisplayMultibancoDetails>,
    oxxo_display_details: Option<stripe_shared::PaymentIntentNextActionDisplayOxxoDetails>,
    paynow_display_qr_code: Option<stripe_shared::PaymentIntentNextActionPaynowDisplayQrCode>,
    pix_display_qr_code: Option<stripe_shared::PaymentIntentNextActionPixDisplayQrCode>,
    promptpay_display_qr_code: Option<stripe_shared::PaymentIntentNextActionPromptpayDisplayQrCode>,
    redirect_to_url: Option<stripe_shared::PaymentIntentNextActionRedirectToUrl>,
    swish_handle_redirect_or_display_qr_code:
        Option<stripe_shared::PaymentIntentNextActionSwishHandleRedirectOrDisplayQrCode>,
    type_: Option<String>,
    upi_handle_redirect_or_display_qr_code:
        Option<stripe_shared::PaymentIntentNextActionUpiHandleRedirectOrDisplayQrCode>,
    use_stripe_sdk: Option<Option<miniserde::json::Value>>,
    verify_with_microdeposits:
        Option<stripe_shared::PaymentIntentNextActionVerifyWithMicrodeposits>,
    wechat_pay_display_qr_code:
        Option<stripe_shared::PaymentIntentNextActionWechatPayDisplayQrCode>,
    wechat_pay_redirect_to_android_app:
        Option<stripe_shared::PaymentIntentNextActionWechatPayRedirectToAndroidApp>,
    wechat_pay_redirect_to_ios_app:
        Option<stripe_shared::PaymentIntentNextActionWechatPayRedirectToIosApp>,
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

    impl Deserialize for PaymentIntentNextAction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextAction>,
        builder: PaymentIntentNextActionBuilder,
    }

    impl Visitor for Place<PaymentIntentNextAction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentNextActionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionBuilder {
        type Out = PaymentIntentNextAction;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "alipay_handle_redirect" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.alipay_handle_redirect,
                ),
                "blik_authorize" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.blik_authorize)
                }
                "boleto_display_details" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.boleto_display_details,
                ),
                "card_await_notification" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.card_await_notification,
                ),
                "cashapp_handle_redirect_or_display_qr_code" => {
                    stripe_types::miniserde_helpers::begin_nullable(
                        &mut self.cashapp_handle_redirect_or_display_qr_code,
                    )
                }
                "display_bank_transfer_instructions" => {
                    stripe_types::miniserde_helpers::begin_nullable(
                        &mut self.display_bank_transfer_instructions,
                    )
                }
                "klarna_display_qr_code" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.klarna_display_qr_code,
                ),
                "konbini_display_details" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.konbini_display_details,
                ),
                "multibanco_display_details" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.multibanco_display_details,
                ),
                "oxxo_display_details" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.oxxo_display_details)
                }
                "paynow_display_qr_code" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.paynow_display_qr_code,
                ),
                "pix_display_qr_code" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.pix_display_qr_code)
                }
                "promptpay_display_qr_code" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.promptpay_display_qr_code,
                ),
                "redirect_to_url" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.redirect_to_url)
                }
                "swish_handle_redirect_or_display_qr_code" => {
                    stripe_types::miniserde_helpers::begin_nullable(
                        &mut self.swish_handle_redirect_or_display_qr_code,
                    )
                }
                "type" => Deserialize::begin(&mut self.type_),
                "upi_handle_redirect_or_display_qr_code" => {
                    stripe_types::miniserde_helpers::begin_nullable(
                        &mut self.upi_handle_redirect_or_display_qr_code,
                    )
                }
                "use_stripe_sdk" => Deserialize::begin(&mut self.use_stripe_sdk),
                "verify_with_microdeposits" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.verify_with_microdeposits,
                ),
                "wechat_pay_display_qr_code" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.wechat_pay_display_qr_code,
                ),
                "wechat_pay_redirect_to_android_app" => {
                    stripe_types::miniserde_helpers::begin_nullable(
                        &mut self.wechat_pay_redirect_to_android_app,
                    )
                }
                "wechat_pay_redirect_to_ios_app" => {
                    stripe_types::miniserde_helpers::begin_nullable(
                        &mut self.wechat_pay_redirect_to_ios_app,
                    )
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                alipay_handle_redirect: None,
                blik_authorize: None,
                boleto_display_details: None,
                card_await_notification: None,
                cashapp_handle_redirect_or_display_qr_code: None,
                display_bank_transfer_instructions: None,
                klarna_display_qr_code: None,
                konbini_display_details: None,
                multibanco_display_details: None,
                oxxo_display_details: None,
                paynow_display_qr_code: None,
                pix_display_qr_code: None,
                promptpay_display_qr_code: None,
                redirect_to_url: None,
                swish_handle_redirect_or_display_qr_code: None,
                type_: None,
                upi_handle_redirect_or_display_qr_code: None,
                use_stripe_sdk: Some(None),
                verify_with_microdeposits: None,
                wechat_pay_display_qr_code: None,
                wechat_pay_redirect_to_android_app: None,
                wechat_pay_redirect_to_ios_app: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                alipay_handle_redirect,
                blik_authorize,
                boleto_display_details,
                card_await_notification,
                cashapp_handle_redirect_or_display_qr_code,
                display_bank_transfer_instructions,
                klarna_display_qr_code,
                konbini_display_details,
                multibanco_display_details,
                oxxo_display_details,
                paynow_display_qr_code,
                pix_display_qr_code,
                promptpay_display_qr_code,
                redirect_to_url,
                swish_handle_redirect_or_display_qr_code,
                Some(type_),
                upi_handle_redirect_or_display_qr_code,
                Some(use_stripe_sdk),
                verify_with_microdeposits,
                wechat_pay_display_qr_code,
                wechat_pay_redirect_to_android_app,
                wechat_pay_redirect_to_ios_app,
            ) = (
                self.alipay_handle_redirect.take(),
                self.blik_authorize,
                self.boleto_display_details.take(),
                self.card_await_notification,
                self.cashapp_handle_redirect_or_display_qr_code.take(),
                self.display_bank_transfer_instructions.take(),
                self.klarna_display_qr_code.take(),
                self.konbini_display_details.take(),
                self.multibanco_display_details.take(),
                self.oxxo_display_details.take(),
                self.paynow_display_qr_code.take(),
                self.pix_display_qr_code.take(),
                self.promptpay_display_qr_code.take(),
                self.redirect_to_url.take(),
                self.swish_handle_redirect_or_display_qr_code.take(),
                self.type_.take(),
                self.upi_handle_redirect_or_display_qr_code.take(),
                self.use_stripe_sdk.take(),
                self.verify_with_microdeposits.take(),
                self.wechat_pay_display_qr_code.take(),
                self.wechat_pay_redirect_to_android_app.take(),
                self.wechat_pay_redirect_to_ios_app.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                alipay_handle_redirect,
                blik_authorize,
                boleto_display_details,
                card_await_notification,
                cashapp_handle_redirect_or_display_qr_code,
                display_bank_transfer_instructions,
                klarna_display_qr_code,
                konbini_display_details,
                multibanco_display_details,
                oxxo_display_details,
                paynow_display_qr_code,
                pix_display_qr_code,
                promptpay_display_qr_code,
                redirect_to_url,
                swish_handle_redirect_or_display_qr_code,
                type_,
                upi_handle_redirect_or_display_qr_code,
                use_stripe_sdk,
                verify_with_microdeposits,
                wechat_pay_display_qr_code,
                wechat_pay_redirect_to_android_app,
                wechat_pay_redirect_to_ios_app,
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for PaymentIntentNextAction {
        type Builder = PaymentIntentNextActionBuilder;
    }

    impl FromValueOpt for PaymentIntentNextAction {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
