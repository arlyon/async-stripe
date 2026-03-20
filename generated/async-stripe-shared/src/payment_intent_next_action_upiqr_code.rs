#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentNextActionUpiqrCode {
    /// The date (unix timestamp) when the QR code expires.
    pub expires_at: stripe_types::Timestamp,
    /// The image_url_png string used to render QR code
    pub image_url_png: String,
    /// The image_url_svg string used to render QR code
    pub image_url_svg: String,
}
#[doc(hidden)]
pub struct PaymentIntentNextActionUpiqrCodeBuilder {
    expires_at: Option<stripe_types::Timestamp>,
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

    impl Deserialize for PaymentIntentNextActionUpiqrCode {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionUpiqrCode>,
        builder: PaymentIntentNextActionUpiqrCodeBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionUpiqrCode> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentNextActionUpiqrCodeBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionUpiqrCodeBuilder {
        type Out = PaymentIntentNextActionUpiqrCode;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "expires_at" => Deserialize::begin(&mut self.expires_at),
                "image_url_png" => Deserialize::begin(&mut self.image_url_png),
                "image_url_svg" => Deserialize::begin(&mut self.image_url_svg),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                expires_at: Deserialize::default(),
                image_url_png: Deserialize::default(),
                image_url_svg: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(expires_at), Some(image_url_png), Some(image_url_svg)) =
                (self.expires_at, self.image_url_png.take(), self.image_url_svg.take())
            else {
                return None;
            };
            Some(Self::Out { expires_at, image_url_png, image_url_svg })
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

    impl ObjectDeser for PaymentIntentNextActionUpiqrCode {
        type Builder = PaymentIntentNextActionUpiqrCodeBuilder;
    }

    impl FromValueOpt for PaymentIntentNextActionUpiqrCode {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentIntentNextActionUpiqrCodeBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "expires_at" => b.expires_at = FromValueOpt::from_value(v),
                    "image_url_png" => b.image_url_png = FromValueOpt::from_value(v),
                    "image_url_svg" => b.image_url_svg = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
