#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalFlowsFlowAfterCompletion {
    /// Configuration when `after_completion.type=hosted_confirmation`.
    pub hosted_confirmation: Option<stripe_billing::PortalFlowsAfterCompletionHostedConfirmation>,
    /// Configuration when `after_completion.type=redirect`.
    pub redirect: Option<stripe_billing::PortalFlowsAfterCompletionRedirect>,
    /// The specified type of behavior after the flow is completed.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: PortalFlowsFlowAfterCompletionType,
}
#[doc(hidden)]
pub struct PortalFlowsFlowAfterCompletionBuilder {
    hosted_confirmation:
        Option<Option<stripe_billing::PortalFlowsAfterCompletionHostedConfirmation>>,
    redirect: Option<Option<stripe_billing::PortalFlowsAfterCompletionRedirect>>,
    type_: Option<PortalFlowsFlowAfterCompletionType>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PortalFlowsFlowAfterCompletion {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalFlowsFlowAfterCompletion>,
        builder: PortalFlowsFlowAfterCompletionBuilder,
    }

    impl Visitor for Place<PortalFlowsFlowAfterCompletion> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PortalFlowsFlowAfterCompletionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PortalFlowsFlowAfterCompletionBuilder {
        type Out = PortalFlowsFlowAfterCompletion;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "hosted_confirmation" => Deserialize::begin(&mut self.hosted_confirmation),
                "redirect" => Deserialize::begin(&mut self.redirect),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                hosted_confirmation: Deserialize::default(),
                redirect: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                hosted_confirmation: self.hosted_confirmation.take()?,
                redirect: self.redirect.take()?,
                type_: self.type_?,
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

    impl ObjectDeser for PortalFlowsFlowAfterCompletion {
        type Builder = PortalFlowsFlowAfterCompletionBuilder;
    }

    impl FromValueOpt for PortalFlowsFlowAfterCompletion {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PortalFlowsFlowAfterCompletionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "hosted_confirmation" => {
                        b.hosted_confirmation = Some(FromValueOpt::from_value(v)?)
                    }
                    "redirect" => b.redirect = Some(FromValueOpt::from_value(v)?),
                    "type" => b.type_ = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The specified type of behavior after the flow is completed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PortalFlowsFlowAfterCompletionType {
    HostedConfirmation,
    PortalHomepage,
    Redirect,
}
impl PortalFlowsFlowAfterCompletionType {
    pub fn as_str(self) -> &'static str {
        use PortalFlowsFlowAfterCompletionType::*;
        match self {
            HostedConfirmation => "hosted_confirmation",
            PortalHomepage => "portal_homepage",
            Redirect => "redirect",
        }
    }
}

impl std::str::FromStr for PortalFlowsFlowAfterCompletionType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PortalFlowsFlowAfterCompletionType::*;
        match s {
            "hosted_confirmation" => Ok(HostedConfirmation),
            "portal_homepage" => Ok(PortalHomepage),
            "redirect" => Ok(Redirect),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PortalFlowsFlowAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PortalFlowsFlowAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PortalFlowsFlowAfterCompletionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PortalFlowsFlowAfterCompletionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PortalFlowsFlowAfterCompletionType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(PortalFlowsFlowAfterCompletionType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PortalFlowsFlowAfterCompletionType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PortalFlowsFlowAfterCompletionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PortalFlowsFlowAfterCompletionType")
        })
    }
}
