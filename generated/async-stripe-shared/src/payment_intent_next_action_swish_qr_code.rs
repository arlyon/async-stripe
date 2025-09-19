#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentNextActionSwishQrCode {
    /// The raw data string used to generate QR code, it should be used together with QR code library.
    pub data: String,
    /// The image_url_png string used to render QR code
    pub image_url_png: String,
    /// The image_url_svg string used to render QR code
    pub image_url_svg: String,
}
#[doc(hidden)]
pub struct PaymentIntentNextActionSwishQrCodeBuilder {
    data: Option<String>,
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
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentIntentNextActionSwishQrCode {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionSwishQrCode>,
        builder: PaymentIntentNextActionSwishQrCodeBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionSwishQrCode> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentNextActionSwishQrCodeBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionSwishQrCodeBuilder {
        type Out = PaymentIntentNextActionSwishQrCode;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "data" => Deserialize::begin(&mut self.data),
                "image_url_png" => Deserialize::begin(&mut self.image_url_png),
                "image_url_svg" => Deserialize::begin(&mut self.image_url_svg),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                data: Deserialize::default(),
                image_url_png: Deserialize::default(),
                image_url_svg: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(data), Some(image_url_png), Some(image_url_svg)) =
                (self.data.take(), self.image_url_png.take(), self.image_url_svg.take())
            else {
                return None;
            };
            Some(Self::Out { data, image_url_png, image_url_svg })
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

    impl ObjectDeser for PaymentIntentNextActionSwishQrCode {
        type Builder = PaymentIntentNextActionSwishQrCodeBuilder;
    }

    impl FromValueOpt for PaymentIntentNextActionSwishQrCode {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentIntentNextActionSwishQrCodeBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "data" => b.data = FromValueOpt::from_value(v),
                    "image_url_png" => b.image_url_png = FromValueOpt::from_value(v),
                    "image_url_svg" => b.image_url_svg = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
