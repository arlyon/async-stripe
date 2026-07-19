#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentNextActionUpiHandleRedirectOrDisplayQrCode {
    /// The URL to the hosted UPI instructions page, which allows customers to view the QR code.
    pub hosted_instructions_url: String,
    pub qr_code: stripe_shared::PaymentIntentNextActionUpiqrCode,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentIntentNextActionUpiHandleRedirectOrDisplayQrCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentIntentNextActionUpiHandleRedirectOrDisplayQrCode")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentIntentNextActionUpiHandleRedirectOrDisplayQrCodeBuilder {
    hosted_instructions_url: Option<String>,
    qr_code: Option<stripe_shared::PaymentIntentNextActionUpiqrCode>,
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

    impl Deserialize for PaymentIntentNextActionUpiHandleRedirectOrDisplayQrCode {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionUpiHandleRedirectOrDisplayQrCode>,
        builder: PaymentIntentNextActionUpiHandleRedirectOrDisplayQrCodeBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionUpiHandleRedirectOrDisplayQrCode> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentNextActionUpiHandleRedirectOrDisplayQrCodeBuilder {
                    hosted_instructions_url: Deserialize::default(),
                    qr_code: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "hosted_instructions_url" => {
                    Deserialize::begin(&mut self.builder.hosted_instructions_url)
                }
                "qr_code" => Deserialize::begin(&mut self.builder.qr_code),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(hosted_instructions_url), Some(qr_code)) =
                (self.builder.hosted_instructions_url.take(), self.builder.qr_code.take())
            else {
                return Ok(());
            };
            *self.out = Some(PaymentIntentNextActionUpiHandleRedirectOrDisplayQrCode {
                hosted_instructions_url,
                qr_code,
            });
            Ok(())
        }
    }
};
