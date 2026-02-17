/// Returns redirect links used for onboarding onto Tap to Pay on iPhone.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalOnboardingLink {
    pub link_options: stripe_terminal::TerminalOnboardingLinkLinkOptions,
    /// The type of link being generated.
    pub link_type: stripe_terminal::TerminalOnboardingLinkLinkType,
    /// Stripe account ID to generate the link for.
    pub on_behalf_of: Option<String>,
    /// The link passed back to the user for their onboarding.
    pub redirect_url: String,
}
#[doc(hidden)]
pub struct TerminalOnboardingLinkBuilder {
    link_options: Option<stripe_terminal::TerminalOnboardingLinkLinkOptions>,
    link_type: Option<stripe_terminal::TerminalOnboardingLinkLinkType>,
    on_behalf_of: Option<Option<String>>,
    redirect_url: Option<String>,
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

    impl Deserialize for TerminalOnboardingLink {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalOnboardingLink>,
        builder: TerminalOnboardingLinkBuilder,
    }

    impl Visitor for Place<TerminalOnboardingLink> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TerminalOnboardingLinkBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TerminalOnboardingLinkBuilder {
        type Out = TerminalOnboardingLink;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "link_options" => Deserialize::begin(&mut self.link_options),
                "link_type" => Deserialize::begin(&mut self.link_type),
                "on_behalf_of" => Deserialize::begin(&mut self.on_behalf_of),
                "redirect_url" => Deserialize::begin(&mut self.redirect_url),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                link_options: Deserialize::default(),
                link_type: Deserialize::default(),
                on_behalf_of: Deserialize::default(),
                redirect_url: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(link_options), Some(link_type), Some(on_behalf_of), Some(redirect_url)) = (
                self.link_options.take(),
                self.link_type.take(),
                self.on_behalf_of.take(),
                self.redirect_url.take(),
            ) else {
                return None;
            };
            Some(Self::Out { link_options, link_type, on_behalf_of, redirect_url })
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

    impl ObjectDeser for TerminalOnboardingLink {
        type Builder = TerminalOnboardingLinkBuilder;
    }

    impl FromValueOpt for TerminalOnboardingLink {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TerminalOnboardingLinkBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "link_options" => b.link_options = FromValueOpt::from_value(v),
                    "link_type" => b.link_type = FromValueOpt::from_value(v),
                    "on_behalf_of" => b.on_behalf_of = FromValueOpt::from_value(v),
                    "redirect_url" => b.redirect_url = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for TerminalOnboardingLink {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("TerminalOnboardingLink", 5)?;
        s.serialize_field("link_options", &self.link_options)?;
        s.serialize_field("link_type", &self.link_type)?;
        s.serialize_field("on_behalf_of", &self.on_behalf_of)?;
        s.serialize_field("redirect_url", &self.redirect_url)?;

        s.serialize_field("object", "terminal.onboarding_link")?;
        s.end()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TerminalOnboardingLinkLinkType {
    AppleTermsAndConditions,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TerminalOnboardingLinkLinkType {
    pub fn as_str(&self) -> &str {
        use TerminalOnboardingLinkLinkType::*;
        match self {
            AppleTermsAndConditions => "apple_terms_and_conditions",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TerminalOnboardingLinkLinkType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TerminalOnboardingLinkLinkType::*;
        match s {
            "apple_terms_and_conditions" => Ok(AppleTermsAndConditions),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TerminalOnboardingLinkLinkType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TerminalOnboardingLinkLinkType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TerminalOnboardingLinkLinkType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TerminalOnboardingLinkLinkType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TerminalOnboardingLinkLinkType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TerminalOnboardingLinkLinkType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TerminalOnboardingLinkLinkType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TerminalOnboardingLinkLinkType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TerminalOnboardingLinkLinkType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
