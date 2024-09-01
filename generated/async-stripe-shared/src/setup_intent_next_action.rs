#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SetupIntentNextAction {
    pub cashapp_handle_redirect_or_display_qr_code:
        Option<stripe_shared::PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode>,
    pub redirect_to_url: Option<stripe_shared::SetupIntentNextActionRedirectToUrl>,
    /// Type of the next action to perform, one of `redirect_to_url`, `use_stripe_sdk`, `alipay_handle_redirect`, `oxxo_display_details`, or `verify_with_microdeposits`.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: String,
    /// When confirming a SetupIntent with Stripe.js, Stripe.js depends on the contents of this dictionary to invoke authentication flows.
    /// The shape of the contents is subject to change and is only intended to be used by Stripe.js.
    #[cfg_attr(
        any(feature = "deserialize", feature = "serialize"),
        serde(with = "stripe_types::with_serde_json_opt")
    )]
    pub use_stripe_sdk: Option<miniserde::json::Value>,
    pub verify_with_microdeposits:
        Option<stripe_shared::SetupIntentNextActionVerifyWithMicrodeposits>,
}
#[doc(hidden)]
pub struct SetupIntentNextActionBuilder {
    cashapp_handle_redirect_or_display_qr_code:
        Option<Option<stripe_shared::PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode>>,
    redirect_to_url: Option<Option<stripe_shared::SetupIntentNextActionRedirectToUrl>>,
    type_: Option<String>,
    use_stripe_sdk: Option<Option<miniserde::json::Value>>,
    verify_with_microdeposits:
        Option<Option<stripe_shared::SetupIntentNextActionVerifyWithMicrodeposits>>,
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

    impl Deserialize for SetupIntentNextAction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupIntentNextAction>,
        builder: SetupIntentNextActionBuilder,
    }

    impl Visitor for Place<SetupIntentNextAction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SetupIntentNextActionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SetupIntentNextActionBuilder {
        type Out = SetupIntentNextAction;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "cashapp_handle_redirect_or_display_qr_code" => {
                    Deserialize::begin(&mut self.cashapp_handle_redirect_or_display_qr_code)
                }
                "redirect_to_url" => Deserialize::begin(&mut self.redirect_to_url),
                "type" => Deserialize::begin(&mut self.type_),
                "use_stripe_sdk" => Deserialize::begin(&mut self.use_stripe_sdk),
                "verify_with_microdeposits" => {
                    Deserialize::begin(&mut self.verify_with_microdeposits)
                }

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                cashapp_handle_redirect_or_display_qr_code: Deserialize::default(),
                redirect_to_url: Deserialize::default(),
                type_: Deserialize::default(),
                use_stripe_sdk: Deserialize::default(),
                verify_with_microdeposits: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(cashapp_handle_redirect_or_display_qr_code),
                Some(redirect_to_url),
                Some(type_),
                Some(use_stripe_sdk),
                Some(verify_with_microdeposits),
            ) = (
                self.cashapp_handle_redirect_or_display_qr_code.take(),
                self.redirect_to_url.take(),
                self.type_.take(),
                self.use_stripe_sdk.take(),
                self.verify_with_microdeposits.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                cashapp_handle_redirect_or_display_qr_code,
                redirect_to_url,
                type_,
                use_stripe_sdk,
                verify_with_microdeposits,
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

    impl ObjectDeser for SetupIntentNextAction {
        type Builder = SetupIntentNextActionBuilder;
    }

    impl FromValueOpt for SetupIntentNextAction {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SetupIntentNextActionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "cashapp_handle_redirect_or_display_qr_code" => {
                        b.cashapp_handle_redirect_or_display_qr_code = FromValueOpt::from_value(v)
                    }
                    "redirect_to_url" => b.redirect_to_url = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    "use_stripe_sdk" => b.use_stripe_sdk = FromValueOpt::from_value(v),
                    "verify_with_microdeposits" => {
                        b.verify_with_microdeposits = FromValueOpt::from_value(v)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
