#[derive(Clone, Debug)]
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
    /// Type of the next action to perform, one of `redirect_to_url`, `use_stripe_sdk`, `alipay_handle_redirect`, `oxxo_display_details`, or `verify_with_microdeposits`.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: String,
    /// When confirming a PaymentIntent with Stripe.js, Stripe.js depends on the contents of this dictionary to invoke authentication flows.
    /// The shape of the contents is subject to change and is only intended to be used by Stripe.js.
    #[cfg_attr(
        any(feature = "deserialize", feature = "serialize"),
        serde(with = "stripe_types::with_serde_json_opt")
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
    use_stripe_sdk: Option<Option<miniserde::json::Value>>,
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
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
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
                "alipay_handle_redirect" => Deserialize::begin(&mut self.alipay_handle_redirect),
                "boleto_display_details" => Deserialize::begin(&mut self.boleto_display_details),
                "card_await_notification" => Deserialize::begin(&mut self.card_await_notification),
                "cashapp_handle_redirect_or_display_qr_code" => {
                    Deserialize::begin(&mut self.cashapp_handle_redirect_or_display_qr_code)
                }
                "display_bank_transfer_instructions" => {
                    Deserialize::begin(&mut self.display_bank_transfer_instructions)
                }
                "konbini_display_details" => Deserialize::begin(&mut self.konbini_display_details),
                "multibanco_display_details" => {
                    Deserialize::begin(&mut self.multibanco_display_details)
                }
                "oxxo_display_details" => Deserialize::begin(&mut self.oxxo_display_details),
                "paynow_display_qr_code" => Deserialize::begin(&mut self.paynow_display_qr_code),
                "pix_display_qr_code" => Deserialize::begin(&mut self.pix_display_qr_code),
                "promptpay_display_qr_code" => {
                    Deserialize::begin(&mut self.promptpay_display_qr_code)
                }
                "redirect_to_url" => Deserialize::begin(&mut self.redirect_to_url),
                "swish_handle_redirect_or_display_qr_code" => {
                    Deserialize::begin(&mut self.swish_handle_redirect_or_display_qr_code)
                }
                "type" => Deserialize::begin(&mut self.type_),
                "use_stripe_sdk" => Deserialize::begin(&mut self.use_stripe_sdk),
                "verify_with_microdeposits" => {
                    Deserialize::begin(&mut self.verify_with_microdeposits)
                }
                "wechat_pay_display_qr_code" => {
                    Deserialize::begin(&mut self.wechat_pay_display_qr_code)
                }
                "wechat_pay_redirect_to_android_app" => {
                    Deserialize::begin(&mut self.wechat_pay_redirect_to_android_app)
                }
                "wechat_pay_redirect_to_ios_app" => {
                    Deserialize::begin(&mut self.wechat_pay_redirect_to_ios_app)
                }

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
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
                use_stripe_sdk: Deserialize::default(),
                verify_with_microdeposits: Deserialize::default(),
                wechat_pay_display_qr_code: Deserialize::default(),
                wechat_pay_redirect_to_android_app: Deserialize::default(),
                wechat_pay_redirect_to_ios_app: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
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
                Some(use_stripe_sdk),
                Some(verify_with_microdeposits),
                Some(wechat_pay_display_qr_code),
                Some(wechat_pay_redirect_to_android_app),
                Some(wechat_pay_redirect_to_ios_app),
            ) = (
                self.alipay_handle_redirect.take(),
                self.boleto_display_details.take(),
                self.card_await_notification,
                self.cashapp_handle_redirect_or_display_qr_code.take(),
                self.display_bank_transfer_instructions.take(),
                self.konbini_display_details.take(),
                self.multibanco_display_details.take(),
                self.oxxo_display_details.take(),
                self.paynow_display_qr_code.take(),
                self.pix_display_qr_code.take(),
                self.promptpay_display_qr_code.take(),
                self.redirect_to_url.take(),
                self.swish_handle_redirect_or_display_qr_code.take(),
                self.type_.take(),
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
                use_stripe_sdk,
                verify_with_microdeposits,
                wechat_pay_display_qr_code,
                wechat_pay_redirect_to_android_app,
                wechat_pay_redirect_to_ios_app,
            })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for PaymentIntentNextAction {
        type Builder = PaymentIntentNextActionBuilder;
    }

    impl FromValueOpt for PaymentIntentNextAction {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentIntentNextActionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "alipay_handle_redirect" => {
                        b.alipay_handle_redirect = FromValueOpt::from_value(v)
                    }
                    "boleto_display_details" => {
                        b.boleto_display_details = FromValueOpt::from_value(v)
                    }
                    "card_await_notification" => {
                        b.card_await_notification = FromValueOpt::from_value(v)
                    }
                    "cashapp_handle_redirect_or_display_qr_code" => {
                        b.cashapp_handle_redirect_or_display_qr_code = FromValueOpt::from_value(v)
                    }
                    "display_bank_transfer_instructions" => {
                        b.display_bank_transfer_instructions = FromValueOpt::from_value(v)
                    }
                    "konbini_display_details" => {
                        b.konbini_display_details = FromValueOpt::from_value(v)
                    }
                    "multibanco_display_details" => {
                        b.multibanco_display_details = FromValueOpt::from_value(v)
                    }
                    "oxxo_display_details" => b.oxxo_display_details = FromValueOpt::from_value(v),
                    "paynow_display_qr_code" => {
                        b.paynow_display_qr_code = FromValueOpt::from_value(v)
                    }
                    "pix_display_qr_code" => b.pix_display_qr_code = FromValueOpt::from_value(v),
                    "promptpay_display_qr_code" => {
                        b.promptpay_display_qr_code = FromValueOpt::from_value(v)
                    }
                    "redirect_to_url" => b.redirect_to_url = FromValueOpt::from_value(v),
                    "swish_handle_redirect_or_display_qr_code" => {
                        b.swish_handle_redirect_or_display_qr_code = FromValueOpt::from_value(v)
                    }
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    "use_stripe_sdk" => b.use_stripe_sdk = FromValueOpt::from_value(v),
                    "verify_with_microdeposits" => {
                        b.verify_with_microdeposits = FromValueOpt::from_value(v)
                    }
                    "wechat_pay_display_qr_code" => {
                        b.wechat_pay_display_qr_code = FromValueOpt::from_value(v)
                    }
                    "wechat_pay_redirect_to_android_app" => {
                        b.wechat_pay_redirect_to_android_app = FromValueOpt::from_value(v)
                    }
                    "wechat_pay_redirect_to_ios_app" => {
                        b.wechat_pay_redirect_to_ios_app = FromValueOpt::from_value(v)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
