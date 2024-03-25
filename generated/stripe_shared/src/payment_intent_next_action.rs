#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentIntentNextAction {
    pub alipay_handle_redirect: Option<stripe_shared::PaymentIntentNextActionAlipayHandleRedirect>,
    pub boleto_display_details: Option<stripe_shared::PaymentIntentNextActionBoleto>,
    pub card_await_notification: Option<stripe_shared::PaymentIntentNextActionCardAwaitNotification>,
    pub cashapp_handle_redirect_or_display_qr_code: Option<stripe_shared::PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode>,
    pub display_bank_transfer_instructions: Option<stripe_shared::PaymentIntentNextActionDisplayBankTransferInstructions>,
    pub konbini_display_details: Option<stripe_shared::PaymentIntentNextActionKonbini>,
    pub oxxo_display_details: Option<stripe_shared::PaymentIntentNextActionDisplayOxxoDetails>,
    pub paynow_display_qr_code: Option<stripe_shared::PaymentIntentNextActionPaynowDisplayQrCode>,
    pub pix_display_qr_code: Option<stripe_shared::PaymentIntentNextActionPixDisplayQrCode>,
    pub promptpay_display_qr_code: Option<stripe_shared::PaymentIntentNextActionPromptpayDisplayQrCode>,
    pub redirect_to_url: Option<stripe_shared::PaymentIntentNextActionRedirectToUrl>,
    /// Type of the next action to perform, one of `redirect_to_url`, `use_stripe_sdk`, `alipay_handle_redirect`, `oxxo_display_details`, or `verify_with_microdeposits`.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: String,
    /// When confirming a PaymentIntent with Stripe.js, Stripe.js depends on the contents of this dictionary to invoke authentication flows.
    /// The shape of the contents is subject to change and is only intended to be used by Stripe.js.
    pub use_stripe_sdk: Option<stripe_types::Value>,
    pub verify_with_microdeposits: Option<stripe_shared::PaymentIntentNextActionVerifyWithMicrodeposits>,
    pub wechat_pay_display_qr_code: Option<stripe_shared::PaymentIntentNextActionWechatPayDisplayQrCode>,
    pub wechat_pay_redirect_to_android_app: Option<stripe_shared::PaymentIntentNextActionWechatPayRedirectToAndroidApp>,
    pub wechat_pay_redirect_to_ios_app: Option<stripe_shared::PaymentIntentNextActionWechatPayRedirectToIosApp>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentIntentNextActionBuilder {
    alipay_handle_redirect: Option<Option<stripe_shared::PaymentIntentNextActionAlipayHandleRedirect>>,
    boleto_display_details: Option<Option<stripe_shared::PaymentIntentNextActionBoleto>>,
    card_await_notification: Option<Option<stripe_shared::PaymentIntentNextActionCardAwaitNotification>>,
    cashapp_handle_redirect_or_display_qr_code: Option<Option<stripe_shared::PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode>>,
    display_bank_transfer_instructions: Option<Option<stripe_shared::PaymentIntentNextActionDisplayBankTransferInstructions>>,
    konbini_display_details: Option<Option<stripe_shared::PaymentIntentNextActionKonbini>>,
    oxxo_display_details: Option<Option<stripe_shared::PaymentIntentNextActionDisplayOxxoDetails>>,
    paynow_display_qr_code: Option<Option<stripe_shared::PaymentIntentNextActionPaynowDisplayQrCode>>,
    pix_display_qr_code: Option<Option<stripe_shared::PaymentIntentNextActionPixDisplayQrCode>>,
    promptpay_display_qr_code: Option<Option<stripe_shared::PaymentIntentNextActionPromptpayDisplayQrCode>>,
    redirect_to_url: Option<Option<stripe_shared::PaymentIntentNextActionRedirectToUrl>>,
    type_: Option<String>,
    use_stripe_sdk: Option<Option<stripe_types::Value>>,
    verify_with_microdeposits: Option<Option<stripe_shared::PaymentIntentNextActionVerifyWithMicrodeposits>>,
    wechat_pay_display_qr_code: Option<Option<stripe_shared::PaymentIntentNextActionWechatPayDisplayQrCode>>,
    wechat_pay_redirect_to_android_app: Option<Option<stripe_shared::PaymentIntentNextActionWechatPayRedirectToAndroidApp>>,
    wechat_pay_redirect_to_ios_app: Option<Option<stripe_shared::PaymentIntentNextActionWechatPayRedirectToIosApp>>,
}

#[cfg(feature = "min-ser")]
#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
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
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentIntentNextActionBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionBuilder {
        type Out = PaymentIntentNextAction;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "alipay_handle_redirect" => Deserialize::begin(&mut self.alipay_handle_redirect),
                "boleto_display_details" => Deserialize::begin(&mut self.boleto_display_details),
                "card_await_notification" => Deserialize::begin(&mut self.card_await_notification),
                "cashapp_handle_redirect_or_display_qr_code" => Deserialize::begin(&mut self.cashapp_handle_redirect_or_display_qr_code),
                "display_bank_transfer_instructions" => Deserialize::begin(&mut self.display_bank_transfer_instructions),
                "konbini_display_details" => Deserialize::begin(&mut self.konbini_display_details),
                "oxxo_display_details" => Deserialize::begin(&mut self.oxxo_display_details),
                "paynow_display_qr_code" => Deserialize::begin(&mut self.paynow_display_qr_code),
                "pix_display_qr_code" => Deserialize::begin(&mut self.pix_display_qr_code),
                "promptpay_display_qr_code" => Deserialize::begin(&mut self.promptpay_display_qr_code),
                "redirect_to_url" => Deserialize::begin(&mut self.redirect_to_url),
                "type" => Deserialize::begin(&mut self.type_),
                "use_stripe_sdk" => Deserialize::begin(&mut self.use_stripe_sdk),
                "verify_with_microdeposits" => Deserialize::begin(&mut self.verify_with_microdeposits),
                "wechat_pay_display_qr_code" => Deserialize::begin(&mut self.wechat_pay_display_qr_code),
                "wechat_pay_redirect_to_android_app" => Deserialize::begin(&mut self.wechat_pay_redirect_to_android_app),
                "wechat_pay_redirect_to_ios_app" => Deserialize::begin(&mut self.wechat_pay_redirect_to_ios_app),

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
                oxxo_display_details: Deserialize::default(),
                paynow_display_qr_code: Deserialize::default(),
                pix_display_qr_code: Deserialize::default(),
                promptpay_display_qr_code: Deserialize::default(),
                redirect_to_url: Deserialize::default(),
                type_: Deserialize::default(),
                use_stripe_sdk: Deserialize::default(),
                verify_with_microdeposits: Deserialize::default(),
                wechat_pay_display_qr_code: Deserialize::default(),
                wechat_pay_redirect_to_android_app: Deserialize::default(),
                wechat_pay_redirect_to_ios_app: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let alipay_handle_redirect = self.alipay_handle_redirect.take()?;
            let boleto_display_details = self.boleto_display_details.take()?;
            let card_await_notification = self.card_await_notification.take()?;
            let cashapp_handle_redirect_or_display_qr_code = self.cashapp_handle_redirect_or_display_qr_code.take()?;
            let display_bank_transfer_instructions = self.display_bank_transfer_instructions.take()?;
            let konbini_display_details = self.konbini_display_details.take()?;
            let oxxo_display_details = self.oxxo_display_details.take()?;
            let paynow_display_qr_code = self.paynow_display_qr_code.take()?;
            let pix_display_qr_code = self.pix_display_qr_code.take()?;
            let promptpay_display_qr_code = self.promptpay_display_qr_code.take()?;
            let redirect_to_url = self.redirect_to_url.take()?;
            let type_ = self.type_.take()?;
            let use_stripe_sdk = self.use_stripe_sdk.take()?;
            let verify_with_microdeposits = self.verify_with_microdeposits.take()?;
            let wechat_pay_display_qr_code = self.wechat_pay_display_qr_code.take()?;
            let wechat_pay_redirect_to_android_app = self.wechat_pay_redirect_to_android_app.take()?;
            let wechat_pay_redirect_to_ios_app = self.wechat_pay_redirect_to_ios_app.take()?;

            Some(Self::Out {
                alipay_handle_redirect,
                boleto_display_details,
                card_await_notification,
                cashapp_handle_redirect_or_display_qr_code,
                display_bank_transfer_instructions,
                konbini_display_details,
                oxxo_display_details,
                paynow_display_qr_code,
                pix_display_qr_code,
                promptpay_display_qr_code,
                redirect_to_url,
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
                    "alipay_handle_redirect" => b.alipay_handle_redirect = Some(FromValueOpt::from_value(v)?),
                    "boleto_display_details" => b.boleto_display_details = Some(FromValueOpt::from_value(v)?),
                    "card_await_notification" => b.card_await_notification = Some(FromValueOpt::from_value(v)?),
                    "cashapp_handle_redirect_or_display_qr_code" => b.cashapp_handle_redirect_or_display_qr_code = Some(FromValueOpt::from_value(v)?),
                    "display_bank_transfer_instructions" => b.display_bank_transfer_instructions = Some(FromValueOpt::from_value(v)?),
                    "konbini_display_details" => b.konbini_display_details = Some(FromValueOpt::from_value(v)?),
                    "oxxo_display_details" => b.oxxo_display_details = Some(FromValueOpt::from_value(v)?),
                    "paynow_display_qr_code" => b.paynow_display_qr_code = Some(FromValueOpt::from_value(v)?),
                    "pix_display_qr_code" => b.pix_display_qr_code = Some(FromValueOpt::from_value(v)?),
                    "promptpay_display_qr_code" => b.promptpay_display_qr_code = Some(FromValueOpt::from_value(v)?),
                    "redirect_to_url" => b.redirect_to_url = Some(FromValueOpt::from_value(v)?),
                    "type" => b.type_ = Some(FromValueOpt::from_value(v)?),
                    "use_stripe_sdk" => b.use_stripe_sdk = Some(FromValueOpt::from_value(v)?),
                    "verify_with_microdeposits" => b.verify_with_microdeposits = Some(FromValueOpt::from_value(v)?),
                    "wechat_pay_display_qr_code" => b.wechat_pay_display_qr_code = Some(FromValueOpt::from_value(v)?),
                    "wechat_pay_redirect_to_android_app" => b.wechat_pay_redirect_to_android_app = Some(FromValueOpt::from_value(v)?),
                    "wechat_pay_redirect_to_ios_app" => b.wechat_pay_redirect_to_ios_app = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
