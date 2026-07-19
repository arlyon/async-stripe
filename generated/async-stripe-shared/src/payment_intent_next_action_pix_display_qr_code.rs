#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentNextActionPixDisplayQrCode {
    /// The raw data string used to generate QR code, it should be used together with QR code library.
    pub data: Option<String>,
    /// The date (unix timestamp) when the PIX expires.
    pub expires_at: Option<i64>,
    /// The URL to the hosted pix instructions page, which allows customers to view the pix QR code.
    pub hosted_instructions_url: Option<String>,
    /// The image_url_png string used to render png QR code
    pub image_url_png: Option<String>,
    /// The image_url_svg string used to render svg QR code
    pub image_url_svg: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentIntentNextActionPixDisplayQrCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentIntentNextActionPixDisplayQrCode").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentIntentNextActionPixDisplayQrCodeBuilder {
    data: Option<Option<String>>,
    expires_at: Option<Option<i64>>,
    hosted_instructions_url: Option<Option<String>>,
    image_url_png: Option<Option<String>>,
    image_url_svg: Option<Option<String>>,
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

    impl Deserialize for PaymentIntentNextActionPixDisplayQrCode {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionPixDisplayQrCode>,
        builder: PaymentIntentNextActionPixDisplayQrCodeBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionPixDisplayQrCode> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentNextActionPixDisplayQrCodeBuilder {
                    data: Deserialize::default(),
                    expires_at: Deserialize::default(),
                    hosted_instructions_url: Deserialize::default(),
                    image_url_png: Deserialize::default(),
                    image_url_svg: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "data" => Deserialize::begin(&mut self.builder.data),
                "expires_at" => Deserialize::begin(&mut self.builder.expires_at),
                "hosted_instructions_url" => {
                    Deserialize::begin(&mut self.builder.hosted_instructions_url)
                }
                "image_url_png" => Deserialize::begin(&mut self.builder.image_url_png),
                "image_url_svg" => Deserialize::begin(&mut self.builder.image_url_svg),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(data),
                Some(expires_at),
                Some(hosted_instructions_url),
                Some(image_url_png),
                Some(image_url_svg),
            ) = (
                self.builder.data.take(),
                self.builder.expires_at,
                self.builder.hosted_instructions_url.take(),
                self.builder.image_url_png.take(),
                self.builder.image_url_svg.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(PaymentIntentNextActionPixDisplayQrCode {
                data,
                expires_at,
                hosted_instructions_url,
                image_url_png,
                image_url_svg,
            });
            Ok(())
        }
    }
};
