#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode {
    /// The URL to the hosted Cash App Pay instructions page, which allows customers to view the QR code, and supports QR code refreshing on expiration.
    pub hosted_instructions_url: String,
    /// The url for mobile redirect based auth
    pub mobile_auth_url: String,
    pub qr_code: stripe_shared::PaymentIntentNextActionCashappQrCode,
}
#[doc(hidden)]
pub struct PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCodeBuilder {
    hosted_instructions_url: Option<String>,
    mobile_auth_url: Option<String>,
    qr_code: Option<stripe_shared::PaymentIntentNextActionCashappQrCode>,
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

    impl Deserialize for PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode>,
        builder: PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCodeBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCodeBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCodeBuilder {
        type Out = PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "hosted_instructions_url" => Deserialize::begin(&mut self.hosted_instructions_url),
                "mobile_auth_url" => Deserialize::begin(&mut self.mobile_auth_url),
                "qr_code" => Deserialize::begin(&mut self.qr_code),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                hosted_instructions_url: Deserialize::default(),
                mobile_auth_url: Deserialize::default(),
                qr_code: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(hosted_instructions_url), Some(mobile_auth_url), Some(qr_code)) = (
                self.hosted_instructions_url.take(),
                self.mobile_auth_url.take(),
                self.qr_code.take(),
            ) else {
                return None;
            };
            Some(Self::Out { hosted_instructions_url, mobile_auth_url, qr_code })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode {
        type Builder = PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCodeBuilder;
    }

    impl FromValueOpt for PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCodeBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "hosted_instructions_url" => {
                        b.hosted_instructions_url = FromValueOpt::from_value(v)
                    }
                    "mobile_auth_url" => b.mobile_auth_url = FromValueOpt::from_value(v),
                    "qr_code" => b.qr_code = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
