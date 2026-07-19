#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalFlowsFlowAfterCompletion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PortalFlowsFlowAfterCompletion").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PortalFlowsFlowAfterCompletionBuilder {
    hosted_confirmation:
        Option<Option<stripe_billing::PortalFlowsAfterCompletionHostedConfirmation>>,
    redirect: Option<Option<stripe_billing::PortalFlowsAfterCompletionRedirect>>,
    type_: Option<PortalFlowsFlowAfterCompletionType>,
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
                builder: PortalFlowsFlowAfterCompletionBuilder {
                    hosted_confirmation: Deserialize::default(),
                    redirect: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "hosted_confirmation" => Deserialize::begin(&mut self.builder.hosted_confirmation),
                "redirect" => Deserialize::begin(&mut self.builder.redirect),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(hosted_confirmation), Some(redirect), Some(type_)) = (
                self.builder.hosted_confirmation.take(),
                self.builder.redirect.take(),
                self.builder.type_.take(),
            ) else {
                return Ok(());
            };
            *self.out =
                Some(PortalFlowsFlowAfterCompletion { hosted_confirmation, redirect, type_ });
            Ok(())
        }
    }
};
/// The specified type of behavior after the flow is completed.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PortalFlowsFlowAfterCompletionType {
    HostedConfirmation,
    PortalHomepage,
    Redirect,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PortalFlowsFlowAfterCompletionType {
    pub fn as_str(&self) -> &str {
        use PortalFlowsFlowAfterCompletionType::*;
        match self {
            HostedConfirmation => "hosted_confirmation",
            PortalHomepage => "portal_homepage",
            Redirect => "redirect",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PortalFlowsFlowAfterCompletionType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PortalFlowsFlowAfterCompletionType::*;
        match s {
            "hosted_confirmation" => Ok(HostedConfirmation),
            "portal_homepage" => Ok(PortalHomepage),
            "redirect" => Ok(Redirect),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PortalFlowsFlowAfterCompletionType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PortalFlowsFlowAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PortalFlowsFlowAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalFlowsFlowAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PortalFlowsFlowAfterCompletionType)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for PortalFlowsFlowAfterCompletionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PortalFlowsFlowAfterCompletionType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PortalFlowsFlowAfterCompletionType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PortalFlowsFlowAfterCompletionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
