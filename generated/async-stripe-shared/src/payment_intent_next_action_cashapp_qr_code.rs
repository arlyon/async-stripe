#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentNextActionCashappQrCode {
    /// The date (unix timestamp) when the QR code expires.
    pub expires_at: stripe_types::Timestamp,
    /// The image_url_png string used to render QR code
    pub image_url_png: String,
    /// The image_url_svg string used to render QR code
    pub image_url_svg: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentIntentNextActionCashappQrCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentIntentNextActionCashappQrCode").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentIntentNextActionCashappQrCodeBuilder {
    expires_at: Option<stripe_types::Timestamp>,
    image_url_png: Option<String>,
    image_url_svg: Option<String>,
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

    impl Deserialize for PaymentIntentNextActionCashappQrCode {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionCashappQrCode>,
        builder: PaymentIntentNextActionCashappQrCodeBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionCashappQrCode> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentNextActionCashappQrCodeBuilder {
                    expires_at: Deserialize::default(),
                    image_url_png: Deserialize::default(),
                    image_url_svg: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "expires_at" => Deserialize::begin(&mut self.builder.expires_at),
                "image_url_png" => Deserialize::begin(&mut self.builder.image_url_png),
                "image_url_svg" => Deserialize::begin(&mut self.builder.image_url_svg),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(expires_at), Some(image_url_png), Some(image_url_svg)) = (
                self.builder.expires_at,
                self.builder.image_url_png.take(),
                self.builder.image_url_svg.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(PaymentIntentNextActionCashappQrCode {
                expires_at,
                image_url_png,
                image_url_svg,
            });
            Ok(())
        }
    }
};
