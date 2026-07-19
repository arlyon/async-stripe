#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SetupIntentNextAction {
    pub cashapp_handle_redirect_or_display_qr_code:
        Option<stripe_shared::PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode>,
    pub redirect_to_url: Option<stripe_shared::SetupIntentNextActionRedirectToUrl>,
    /// Type of the next action to perform.
    /// Refer to the other child attributes under `next_action` for available values.
    /// Examples include: `redirect_to_url`, `use_stripe_sdk`, `alipay_handle_redirect`, `oxxo_display_details`, or `verify_with_microdeposits`.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: String,
    pub upi_handle_redirect_or_display_qr_code:
        Option<stripe_shared::PaymentIntentNextActionUpiHandleRedirectOrDisplayQrCode>,
    /// When confirming a SetupIntent with Stripe.js, Stripe.js depends on the contents of this dictionary to invoke authentication flows.
    /// The shape of the contents is subject to change and is only intended to be used by Stripe.js.
    #[cfg_attr(
        any(feature = "deserialize", feature = "serialize"),
        serde(with = "stripe_types::with_serde_json_opt")
    )]
    pub use_stripe_sdk: Option<stripe_miniserde::json::Value>,
    pub verify_with_microdeposits:
        Option<stripe_shared::SetupIntentNextActionVerifyWithMicrodeposits>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SetupIntentNextAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SetupIntentNextAction").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SetupIntentNextActionBuilder {
    cashapp_handle_redirect_or_display_qr_code:
        Option<Option<stripe_shared::PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode>>,
    redirect_to_url: Option<Option<stripe_shared::SetupIntentNextActionRedirectToUrl>>,
    type_: Option<String>,
    upi_handle_redirect_or_display_qr_code:
        Option<Option<stripe_shared::PaymentIntentNextActionUpiHandleRedirectOrDisplayQrCode>>,
    use_stripe_sdk: Option<Option<stripe_miniserde::json::Value>>,
    verify_with_microdeposits:
        Option<Option<stripe_shared::SetupIntentNextActionVerifyWithMicrodeposits>>,
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
                builder: SetupIntentNextActionBuilder {
                    cashapp_handle_redirect_or_display_qr_code: Deserialize::default(),
                    redirect_to_url: Deserialize::default(),
                    type_: Deserialize::default(),
                    upi_handle_redirect_or_display_qr_code: Deserialize::default(),
                    use_stripe_sdk: Deserialize::default(),
                    verify_with_microdeposits: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "cashapp_handle_redirect_or_display_qr_code" => {
                    Deserialize::begin(&mut self.builder.cashapp_handle_redirect_or_display_qr_code)
                }
                "redirect_to_url" => Deserialize::begin(&mut self.builder.redirect_to_url),
                "type" => Deserialize::begin(&mut self.builder.type_),
                "upi_handle_redirect_or_display_qr_code" => {
                    Deserialize::begin(&mut self.builder.upi_handle_redirect_or_display_qr_code)
                }
                "use_stripe_sdk" => Deserialize::begin(&mut self.builder.use_stripe_sdk),
                "verify_with_microdeposits" => {
                    Deserialize::begin(&mut self.builder.verify_with_microdeposits)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(cashapp_handle_redirect_or_display_qr_code),
                Some(redirect_to_url),
                Some(type_),
                Some(upi_handle_redirect_or_display_qr_code),
                Some(use_stripe_sdk),
                Some(verify_with_microdeposits),
            ) = (
                self.builder.cashapp_handle_redirect_or_display_qr_code.take(),
                self.builder.redirect_to_url.take(),
                self.builder.type_.take(),
                self.builder.upi_handle_redirect_or_display_qr_code.take(),
                self.builder.use_stripe_sdk.take(),
                self.builder.verify_with_microdeposits.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(SetupIntentNextAction {
                cashapp_handle_redirect_or_display_qr_code,
                redirect_to_url,
                type_,
                upi_handle_redirect_or_display_qr_code,
                use_stripe_sdk,
                verify_with_microdeposits,
            });
            Ok(())
        }
    }
};
