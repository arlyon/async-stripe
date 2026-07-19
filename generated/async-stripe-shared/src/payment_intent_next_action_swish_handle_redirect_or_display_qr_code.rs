#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentNextActionSwishHandleRedirectOrDisplayQrCode {
    /// The URL to the hosted Swish instructions page, which allows customers to view the QR code.
    pub hosted_instructions_url: String,
    /// The url for mobile redirect based auth (for internal use only and not typically available in standard API requests).
    pub mobile_auth_url: String,
    pub qr_code: stripe_shared::PaymentIntentNextActionSwishQrCode,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentIntentNextActionSwishHandleRedirectOrDisplayQrCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentIntentNextActionSwishHandleRedirectOrDisplayQrCode")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentIntentNextActionSwishHandleRedirectOrDisplayQrCodeBuilder {
    hosted_instructions_url: Option<String>,
    mobile_auth_url: Option<String>,
    qr_code: Option<stripe_shared::PaymentIntentNextActionSwishQrCode>,
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

    impl Deserialize for PaymentIntentNextActionSwishHandleRedirectOrDisplayQrCode {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionSwishHandleRedirectOrDisplayQrCode>,
        builder: PaymentIntentNextActionSwishHandleRedirectOrDisplayQrCodeBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionSwishHandleRedirectOrDisplayQrCode> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentNextActionSwishHandleRedirectOrDisplayQrCodeBuilder {
                    hosted_instructions_url: Deserialize::default(),
                    mobile_auth_url: Deserialize::default(),
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
                "mobile_auth_url" => Deserialize::begin(&mut self.builder.mobile_auth_url),
                "qr_code" => Deserialize::begin(&mut self.builder.qr_code),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(hosted_instructions_url), Some(mobile_auth_url), Some(qr_code)) = (
                self.builder.hosted_instructions_url.take(),
                self.builder.mobile_auth_url.take(),
                self.builder.qr_code.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(PaymentIntentNextActionSwishHandleRedirectOrDisplayQrCode {
                hosted_instructions_url,
                mobile_auth_url,
                qr_code,
            });
            Ok(())
        }
    }
};
