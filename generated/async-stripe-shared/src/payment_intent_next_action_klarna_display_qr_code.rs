#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentNextActionKlarnaDisplayQrCode {
    /// The data being used to generate QR code
    pub data: String,
    /// The timestamp at which the QR code expires.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// The image_url_png string used to render QR code
    pub image_url_png: String,
    /// The image_url_svg string used to render QR code
    pub image_url_svg: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentIntentNextActionKlarnaDisplayQrCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentIntentNextActionKlarnaDisplayQrCode").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentIntentNextActionKlarnaDisplayQrCodeBuilder {
    data: Option<String>,
    expires_at: Option<Option<stripe_types::Timestamp>>,
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

    impl Deserialize for PaymentIntentNextActionKlarnaDisplayQrCode {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionKlarnaDisplayQrCode>,
        builder: PaymentIntentNextActionKlarnaDisplayQrCodeBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionKlarnaDisplayQrCode> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentNextActionKlarnaDisplayQrCodeBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionKlarnaDisplayQrCodeBuilder {
        type Out = PaymentIntentNextActionKlarnaDisplayQrCode;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "data" => Deserialize::begin(&mut self.data),
                "expires_at" => Deserialize::begin(&mut self.expires_at),
                "image_url_png" => Deserialize::begin(&mut self.image_url_png),
                "image_url_svg" => Deserialize::begin(&mut self.image_url_svg),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { data: None, expires_at: Some(None), image_url_png: None, image_url_svg: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(data), Some(expires_at), Some(image_url_png), Some(image_url_svg)) = (
                self.data.take(),
                self.expires_at,
                self.image_url_png.take(),
                self.image_url_svg.take(),
            ) else {
                return None;
            };
            Some(Self::Out { data, expires_at, image_url_png, image_url_svg })
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

    impl ObjectDeser for PaymentIntentNextActionKlarnaDisplayQrCode {
        type Builder = PaymentIntentNextActionKlarnaDisplayQrCodeBuilder;
    }

    impl FromValueOpt for PaymentIntentNextActionKlarnaDisplayQrCode {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentIntentNextActionKlarnaDisplayQrCodeBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "data" => b.data = FromValueOpt::from_value(v),
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
