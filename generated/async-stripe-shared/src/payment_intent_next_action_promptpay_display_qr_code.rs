#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentNextActionPromptpayDisplayQrCode {
    /// The raw data string used to generate QR code, it should be used together with QR code library.
    pub data: String,
    /// The URL to the hosted PromptPay instructions page, which allows customers to view the PromptPay QR code.
    pub hosted_instructions_url: String,
    /// The PNG path used to render the QR code, can be used as the source in an HTML img tag
    pub image_url_png: String,
    /// The SVG path used to render the QR code, can be used as the source in an HTML img tag
    pub image_url_svg: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentIntentNextActionPromptpayDisplayQrCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentIntentNextActionPromptpayDisplayQrCode").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentIntentNextActionPromptpayDisplayQrCodeBuilder {
    data: Option<String>,
    hosted_instructions_url: Option<String>,
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

    impl Deserialize for PaymentIntentNextActionPromptpayDisplayQrCode {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionPromptpayDisplayQrCode>,
        builder: PaymentIntentNextActionPromptpayDisplayQrCodeBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionPromptpayDisplayQrCode> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentNextActionPromptpayDisplayQrCodeBuilder {
                    data: Deserialize::default(),
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
                Some(hosted_instructions_url),
                Some(image_url_png),
                Some(image_url_svg),
            ) = (
                self.builder.data.take(),
                self.builder.hosted_instructions_url.take(),
                self.builder.image_url_png.take(),
                self.builder.image_url_svg.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(PaymentIntentNextActionPromptpayDisplayQrCode {
                data,
                hosted_instructions_url,
                image_url_png,
                image_url_svg,
            });
            Ok(())
        }
    }
};
