#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentNextActionWechatPayDisplayQrCode {
    /// The data being used to generate QR code
    pub data: String,
    /// The URL to the hosted WeChat Pay instructions page, which allows customers to view the WeChat Pay QR code.
    pub hosted_instructions_url: String,
    /// The base64 image data for a pre-generated QR code
    pub image_data_url: String,
    /// The image_url_png string used to render QR code
    pub image_url_png: String,
    /// The image_url_svg string used to render QR code
    pub image_url_svg: String,
}
#[doc(hidden)]
pub struct PaymentIntentNextActionWechatPayDisplayQrCodeBuilder {
    data: Option<String>,
    hosted_instructions_url: Option<String>,
    image_data_url: Option<String>,
    image_url_png: Option<String>,
    image_url_svg: Option<String>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
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

    impl Deserialize for PaymentIntentNextActionWechatPayDisplayQrCode {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionWechatPayDisplayQrCode>,
        builder: PaymentIntentNextActionWechatPayDisplayQrCodeBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionWechatPayDisplayQrCode> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentNextActionWechatPayDisplayQrCodeBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionWechatPayDisplayQrCodeBuilder {
        type Out = PaymentIntentNextActionWechatPayDisplayQrCode;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "data" => Deserialize::begin(&mut self.data),
                "hosted_instructions_url" => Deserialize::begin(&mut self.hosted_instructions_url),
                "image_data_url" => Deserialize::begin(&mut self.image_data_url),
                "image_url_png" => Deserialize::begin(&mut self.image_url_png),
                "image_url_svg" => Deserialize::begin(&mut self.image_url_svg),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                data: Deserialize::default(),
                hosted_instructions_url: Deserialize::default(),
                image_data_url: Deserialize::default(),
                image_url_png: Deserialize::default(),
                image_url_svg: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(data),
                Some(hosted_instructions_url),
                Some(image_data_url),
                Some(image_url_png),
                Some(image_url_svg),
            ) = (
                self.data.take(),
                self.hosted_instructions_url.take(),
                self.image_data_url.take(),
                self.image_url_png.take(),
                self.image_url_svg.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                data,
                hosted_instructions_url,
                image_data_url,
                image_url_png,
                image_url_svg,
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

    impl ObjectDeser for PaymentIntentNextActionWechatPayDisplayQrCode {
        type Builder = PaymentIntentNextActionWechatPayDisplayQrCodeBuilder;
    }

    impl FromValueOpt for PaymentIntentNextActionWechatPayDisplayQrCode {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentIntentNextActionWechatPayDisplayQrCodeBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "data" => b.data = FromValueOpt::from_value(v),
                    "hosted_instructions_url" => {
                        b.hosted_instructions_url = FromValueOpt::from_value(v)
                    }
                    "image_data_url" => b.image_data_url = FromValueOpt::from_value(v),
                    "image_url_png" => b.image_url_png = FromValueOpt::from_value(v),
                    "image_url_svg" => b.image_url_svg = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
