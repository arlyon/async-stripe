#[derive(Clone, Debug)]
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
                builder: PaymentIntentNextActionPromptpayDisplayQrCodeBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionPromptpayDisplayQrCodeBuilder {
        type Out = PaymentIntentNextActionPromptpayDisplayQrCode;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "data" => Deserialize::begin(&mut self.data),
                "hosted_instructions_url" => Deserialize::begin(&mut self.hosted_instructions_url),
                "image_url_png" => Deserialize::begin(&mut self.image_url_png),
                "image_url_svg" => Deserialize::begin(&mut self.image_url_svg),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                data: Deserialize::default(),
                hosted_instructions_url: Deserialize::default(),
                image_url_png: Deserialize::default(),
                image_url_svg: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(data),
                Some(hosted_instructions_url),
                Some(image_url_png),
                Some(image_url_svg),
            ) = (
                self.data.take(),
                self.hosted_instructions_url.take(),
                self.image_url_png.take(),
                self.image_url_svg.take(),
            )
            else {
                return None;
            };
            Some(Self::Out { data, hosted_instructions_url, image_url_png, image_url_svg })
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

    impl ObjectDeser for PaymentIntentNextActionPromptpayDisplayQrCode {
        type Builder = PaymentIntentNextActionPromptpayDisplayQrCodeBuilder;
    }

    impl FromValueOpt for PaymentIntentNextActionPromptpayDisplayQrCode {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentIntentNextActionPromptpayDisplayQrCodeBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "data" => b.data = FromValueOpt::from_value(v),
                    "hosted_instructions_url" => {
                        b.hosted_instructions_url = FromValueOpt::from_value(v)
                    }
                    "image_url_png" => b.image_url_png = FromValueOpt::from_value(v),
                    "image_url_svg" => b.image_url_svg = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
