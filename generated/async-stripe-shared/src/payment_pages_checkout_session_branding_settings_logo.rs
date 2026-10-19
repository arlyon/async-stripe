#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionBrandingSettingsLogo {
    /// The ID of a [File upload](https://stripe.com/docs/api/files) representing the logo.
    /// Purpose must be `business_logo`.
    /// Required if `type` is `file` and disallowed otherwise.
    pub file: Option<String>,
    /// The type of image for the logo. Must be one of `file` or `url`.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: stripe_shared::CreateCheckoutSessionBrandingSettingsIconType,
    /// The URL of the image. Present when `type` is `url`.
    pub url: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentPagesCheckoutSessionBrandingSettingsLogo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentPagesCheckoutSessionBrandingSettingsLogo").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionBrandingSettingsLogoBuilder {
    file: Option<Option<String>>,
    type_: Option<stripe_shared::CreateCheckoutSessionBrandingSettingsIconType>,
    url: Option<Option<String>>,
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

    impl Deserialize for PaymentPagesCheckoutSessionBrandingSettingsLogo {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionBrandingSettingsLogo>,
        builder: PaymentPagesCheckoutSessionBrandingSettingsLogoBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionBrandingSettingsLogo> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentPagesCheckoutSessionBrandingSettingsLogoBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionBrandingSettingsLogoBuilder {
        type Out = PaymentPagesCheckoutSessionBrandingSettingsLogo;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "file" => Deserialize::begin(&mut self.file),
                "type" => Deserialize::begin(&mut self.type_),
                "url" => Deserialize::begin(&mut self.url),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { file: Some(None), type_: None, url: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(file), Some(type_), Some(url)) =
                (self.file.take(), self.type_.take(), self.url.take())
            else {
                return None;
            };
            Some(Self::Out { file, type_, url })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for PaymentPagesCheckoutSessionBrandingSettingsLogo {
        type Builder = PaymentPagesCheckoutSessionBrandingSettingsLogoBuilder;
    }

    impl FromValueOpt for PaymentPagesCheckoutSessionBrandingSettingsLogo {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentPagesCheckoutSessionBrandingSettingsLogoBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "file" => b.file = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    "url" => b.url = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The type of image for the logo. Must be one of `file` or `url`.
pub type PaymentPagesCheckoutSessionBrandingSettingsLogoType =
    stripe_shared::CreateCheckoutSessionBrandingSettingsIconType;
