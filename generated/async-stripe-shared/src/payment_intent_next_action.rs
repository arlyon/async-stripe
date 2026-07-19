#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentNextAction {
    pub alipay_handle_redirect: Option<stripe_shared::PaymentIntentNextActionAlipayHandleRedirect>,
    pub boleto_display_details: Option<stripe_shared::PaymentIntentNextActionBoleto>,
    pub card_await_notification:
        Option<stripe_shared::PaymentIntentNextActionCardAwaitNotification>,
    pub cashapp_handle_redirect_or_display_qr_code:
        Option<stripe_shared::PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode>,
    pub display_bank_transfer_instructions:
        Option<stripe_shared::PaymentIntentNextActionDisplayBankTransferInstructions>,
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
        serde(with = "stripe_types::with_serde_json_opt")
    )]
    pub use_stripe_sdk: Option<stripe_miniserde::json::Value>,
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
    alipay_handle_redirect:
        Option<Option<stripe_shared::PaymentIntentNextActionAlipayHandleRedirect>>,
    boleto_display_details: Option<Option<stripe_shared::PaymentIntentNextActionBoleto>>,
    card_await_notification:
        Option<Option<stripe_shared::PaymentIntentNextActionCardAwaitNotification>>,
    cashapp_handle_redirect_or_display_qr_code:
        Option<Option<stripe_shared::PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode>>,
    display_bank_transfer_instructions:
        Option<Option<stripe_shared::PaymentIntentNextActionDisplayBankTransferInstructions>>,
    konbini_display_details: Option<Option<stripe_shared::PaymentIntentNextActionKonbini>>,
    multibanco_display_details:
        Option<Option<stripe_shared::PaymentIntentNextActionDisplayMultibancoDetails>>,
    oxxo_display_details: Option<Option<stripe_shared::PaymentIntentNextActionDisplayOxxoDetails>>,
    paynow_display_qr_code:
        Option<Option<stripe_shared::PaymentIntentNextActionPaynowDisplayQrCode>>,
    pix_display_qr_code: Option<Option<stripe_shared::PaymentIntentNextActionPixDisplayQrCode>>,
    promptpay_display_qr_code:
        Option<Option<stripe_shared::PaymentIntentNextActionPromptpayDisplayQrCode>>,
    redirect_to_url: Option<Option<stripe_shared::PaymentIntentNextActionRedirectToUrl>>,
    swish_handle_redirect_or_display_qr_code:
        Option<Option<stripe_shared::PaymentIntentNextActionSwishHandleRedirectOrDisplayQrCode>>,
    type_: Option<String>,
    upi_handle_redirect_or_display_qr_code:
        Option<Option<stripe_shared::PaymentIntentNextActionUpiHandleRedirectOrDisplayQrCode>>,
    use_stripe_sdk: Option<Option<stripe_miniserde::json::Value>>,
    verify_with_microdeposits:
        Option<Option<stripe_shared::PaymentIntentNextActionVerifyWithMicrodeposits>>,
    wechat_pay_display_qr_code:
        Option<Option<stripe_shared::PaymentIntentNextActionWechatPayDisplayQrCode>>,
    wechat_pay_redirect_to_android_app:
        Option<Option<stripe_shared::PaymentIntentNextActionWechatPayRedirectToAndroidApp>>,
    wechat_pay_redirect_to_ios_app:
        Option<Option<stripe_shared::PaymentIntentNextActionWechatPayRedirectToIosApp>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: PaymentIntentNextActionBuilder {
                    alipay_handle_redirect: Deserialize::default(),
                    boleto_display_details: Deserialize::default(),
                    card_await_notification: Deserialize::default(),
                    cashapp_handle_redirect_or_display_qr_code: Deserialize::default(),
                    display_bank_transfer_instructions: Deserialize::default(),
                    konbini_display_details: Deserialize::default(),
                    multibanco_display_details: Deserialize::default(),
                    oxxo_display_details: Deserialize::default(),
                    paynow_display_qr_code: Deserialize::default(),
                    pix_display_qr_code: Deserialize::default(),
                    promptpay_display_qr_code: Deserialize::default(),
                    redirect_to_url: Deserialize::default(),
                    swish_handle_redirect_or_display_qr_code: Deserialize::default(),
                    type_: Deserialize::default(),
                    upi_handle_redirect_or_display_qr_code: Deserialize::default(),
                    use_stripe_sdk: Deserialize::default(),
                    verify_with_microdeposits: Deserialize::default(),
                    wechat_pay_display_qr_code: Deserialize::default(),
                    wechat_pay_redirect_to_android_app: Deserialize::default(),
                    wechat_pay_redirect_to_ios_app: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "alipay_handle_redirect" => {
                    Deserialize::begin(&mut self.builder.alipay_handle_redirect)
                }
                "boleto_display_details" => {
                    Deserialize::begin(&mut self.builder.boleto_display_details)
                }
                "card_await_notification" => {
                    Deserialize::begin(&mut self.builder.card_await_notification)
                }
                "cashapp_handle_redirect_or_display_qr_code" => {
                    Deserialize::begin(&mut self.builder.cashapp_handle_redirect_or_display_qr_code)
                }
                "display_bank_transfer_instructions" => {
                    Deserialize::begin(&mut self.builder.display_bank_transfer_instructions)
                }
                "konbini_display_details" => {
                    Deserialize::begin(&mut self.builder.konbini_display_details)
                }
                "multibanco_display_details" => {
                    Deserialize::begin(&mut self.builder.multibanco_display_details)
                }
                "oxxo_display_details" => {
                    Deserialize::begin(&mut self.builder.oxxo_display_details)
                }
                "paynow_display_qr_code" => {
                    Deserialize::begin(&mut self.builder.paynow_display_qr_code)
                }
                "pix_display_qr_code" => Deserialize::begin(&mut self.builder.pix_display_qr_code),
                "promptpay_display_qr_code" => {
                    Deserialize::begin(&mut self.builder.promptpay_display_qr_code)
                }
                "redirect_to_url" => Deserialize::begin(&mut self.builder.redirect_to_url),
                "swish_handle_redirect_or_display_qr_code" => {
                    Deserialize::begin(&mut self.builder.swish_handle_redirect_or_display_qr_code)
                }
                "type" => Deserialize::begin(&mut self.builder.type_),
                "upi_handle_redirect_or_display_qr_code" => {
                    Deserialize::begin(&mut self.builder.upi_handle_redirect_or_display_qr_code)
                }
                "use_stripe_sdk" => Deserialize::begin(&mut self.builder.use_stripe_sdk),
                "verify_with_microdeposits" => {
                    Deserialize::begin(&mut self.builder.verify_with_microdeposits)
                }
                "wechat_pay_display_qr_code" => {
                    Deserialize::begin(&mut self.builder.wechat_pay_display_qr_code)
                }
                "wechat_pay_redirect_to_android_app" => {
                    Deserialize::begin(&mut self.builder.wechat_pay_redirect_to_android_app)
                }
                "wechat_pay_redirect_to_ios_app" => {
                    Deserialize::begin(&mut self.builder.wechat_pay_redirect_to_ios_app)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(alipay_handle_redirect),
                Some(boleto_display_details),
                Some(card_await_notification),
                Some(cashapp_handle_redirect_or_display_qr_code),
                Some(display_bank_transfer_instructions),
                Some(konbini_display_details),
                Some(multibanco_display_details),
                Some(oxxo_display_details),
                Some(paynow_display_qr_code),
                Some(pix_display_qr_code),
                Some(promptpay_display_qr_code),
                Some(redirect_to_url),
                Some(swish_handle_redirect_or_display_qr_code),
                Some(type_),
                Some(upi_handle_redirect_or_display_qr_code),
                Some(use_stripe_sdk),
                Some(verify_with_microdeposits),
                Some(wechat_pay_display_qr_code),
                Some(wechat_pay_redirect_to_android_app),
                Some(wechat_pay_redirect_to_ios_app),
            ) = (
                self.builder.alipay_handle_redirect.take(),
                self.builder.boleto_display_details.take(),
                self.builder.card_await_notification,
                self.builder.cashapp_handle_redirect_or_display_qr_code.take(),
                self.builder.display_bank_transfer_instructions.take(),
                self.builder.konbini_display_details.take(),
                self.builder.multibanco_display_details.take(),
                self.builder.oxxo_display_details.take(),
                self.builder.paynow_display_qr_code.take(),
                self.builder.pix_display_qr_code.take(),
                self.builder.promptpay_display_qr_code.take(),
                self.builder.redirect_to_url.take(),
                self.builder.swish_handle_redirect_or_display_qr_code.take(),
                self.builder.type_.take(),
                self.builder.upi_handle_redirect_or_display_qr_code.take(),
                self.builder.use_stripe_sdk.take(),
                self.builder.verify_with_microdeposits.take(),
                self.builder.wechat_pay_display_qr_code.take(),
                self.builder.wechat_pay_redirect_to_android_app.take(),
                self.builder.wechat_pay_redirect_to_ios_app.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(PaymentIntentNextAction {
                alipay_handle_redirect,
                boleto_display_details,
                card_await_notification,
                cashapp_handle_redirect_or_display_qr_code,
                display_bank_transfer_instructions,
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
            });
            Ok(())
        }
    }
};
