#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SetupIntentNextActionPixDisplayQrCode {
    /// The raw data string used to generate QR code, it should be used together with QR code library.
    pub data: String,
    /// The date (unix timestamp) when the PIX expires.
    pub expires_at: stripe_types::Timestamp,
    /// The URL to the hosted pix instructions page, which allows customers to view the pix QR code.
    pub hosted_instructions_url: String,
    /// The image_url_png string used to render png QR code
    pub image_url_png: String,
    /// The image_url_svg string used to render svg QR code
    pub image_url_svg: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SetupIntentNextActionPixDisplayQrCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SetupIntentNextActionPixDisplayQrCode").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SetupIntentNextActionPixDisplayQrCodeBuilder {
    data: Option<String>,
    expires_at: Option<stripe_types::Timestamp>,
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
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SetupIntentNextActionPixDisplayQrCode {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupIntentNextActionPixDisplayQrCode>,
        builder: SetupIntentNextActionPixDisplayQrCodeBuilder,
    }

    impl Visitor for Place<SetupIntentNextActionPixDisplayQrCode> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SetupIntentNextActionPixDisplayQrCodeBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SetupIntentNextActionPixDisplayQrCodeBuilder {
        type Out = SetupIntentNextActionPixDisplayQrCode;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "data" => Deserialize::begin(&mut self.data),
                "expires_at" => Deserialize::begin(&mut self.expires_at),
                "hosted_instructions_url" => Deserialize::begin(&mut self.hosted_instructions_url),
                "image_url_png" => Deserialize::begin(&mut self.image_url_png),
                "image_url_svg" => Deserialize::begin(&mut self.image_url_svg),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                data: None,
                expires_at: None,
                hosted_instructions_url: None,
                image_url_png: None,
                image_url_svg: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(data),
                Some(expires_at),
                Some(hosted_instructions_url),
                Some(image_url_png),
                Some(image_url_svg),
            ) = (
                self.data.take(),
                self.expires_at,
                self.hosted_instructions_url.take(),
                self.image_url_png.take(),
                self.image_url_svg.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                data,
                expires_at,
                hosted_instructions_url,
                image_url_png,
                image_url_svg,
            })
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

    impl ObjectDeser for SetupIntentNextActionPixDisplayQrCode {
        type Builder = SetupIntentNextActionPixDisplayQrCodeBuilder;
    }

    impl FromValueOpt for SetupIntentNextActionPixDisplayQrCode {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SetupIntentNextActionPixDisplayQrCodeBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "data" => b.data = FromValueOpt::from_value(v),
                    "expires_at" => b.expires_at = FromValueOpt::from_value(v),
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
