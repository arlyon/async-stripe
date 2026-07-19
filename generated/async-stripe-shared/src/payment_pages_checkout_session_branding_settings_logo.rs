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
    pub type_: PaymentPagesCheckoutSessionBrandingSettingsLogoType,
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
    type_: Option<PaymentPagesCheckoutSessionBrandingSettingsLogoType>,
    url: Option<Option<String>>,
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
                builder: PaymentPagesCheckoutSessionBrandingSettingsLogoBuilder {
                    file: Deserialize::default(),
                    type_: Deserialize::default(),
                    url: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "file" => Deserialize::begin(&mut self.builder.file),
                "type" => Deserialize::begin(&mut self.builder.type_),
                "url" => Deserialize::begin(&mut self.builder.url),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(file), Some(type_), Some(url)) =
                (self.builder.file.take(), self.builder.type_.take(), self.builder.url.take())
            else {
                return Ok(());
            };
            *self.out = Some(PaymentPagesCheckoutSessionBrandingSettingsLogo { file, type_, url });
            Ok(())
        }
    }
};
/// The type of image for the logo. Must be one of `file` or `url`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentPagesCheckoutSessionBrandingSettingsLogoType {
    File,
    Url,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentPagesCheckoutSessionBrandingSettingsLogoType {
    pub fn as_str(&self) -> &str {
        use PaymentPagesCheckoutSessionBrandingSettingsLogoType::*;
        match self {
            File => "file",
            Url => "url",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentPagesCheckoutSessionBrandingSettingsLogoType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentPagesCheckoutSessionBrandingSettingsLogoType::*;
        match s {
            "file" => Ok(File),
            "url" => Ok(Url),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentPagesCheckoutSessionBrandingSettingsLogoType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentPagesCheckoutSessionBrandingSettingsLogoType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentPagesCheckoutSessionBrandingSettingsLogoType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentPagesCheckoutSessionBrandingSettingsLogoType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentPagesCheckoutSessionBrandingSettingsLogoType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentPagesCheckoutSessionBrandingSettingsLogoType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PaymentPagesCheckoutSessionBrandingSettingsLogoType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PaymentPagesCheckoutSessionBrandingSettingsLogoType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentPagesCheckoutSessionBrandingSettingsLogoType::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentPagesCheckoutSessionBrandingSettingsLogoType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
