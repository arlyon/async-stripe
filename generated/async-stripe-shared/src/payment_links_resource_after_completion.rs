#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceAfterCompletion {
    pub hosted_confirmation:
        Option<stripe_shared::PaymentLinksResourceCompletionBehaviorConfirmationPage>,
    pub redirect: Option<stripe_shared::PaymentLinksResourceCompletionBehaviorRedirect>,
    /// The specified behavior after the purchase is complete.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: PaymentLinksResourceAfterCompletionType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentLinksResourceAfterCompletion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentLinksResourceAfterCompletion").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentLinksResourceAfterCompletionBuilder {
    hosted_confirmation:
        Option<Option<stripe_shared::PaymentLinksResourceCompletionBehaviorConfirmationPage>>,
    redirect: Option<Option<stripe_shared::PaymentLinksResourceCompletionBehaviorRedirect>>,
    type_: Option<PaymentLinksResourceAfterCompletionType>,
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

    impl Deserialize for PaymentLinksResourceAfterCompletion {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceAfterCompletion>,
        builder: PaymentLinksResourceAfterCompletionBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceAfterCompletion> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentLinksResourceAfterCompletionBuilder {
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
                Some(PaymentLinksResourceAfterCompletion { hosted_confirmation, redirect, type_ });
            Ok(())
        }
    }
};
/// The specified behavior after the purchase is complete.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentLinksResourceAfterCompletionType {
    HostedConfirmation,
    Redirect,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentLinksResourceAfterCompletionType {
    pub fn as_str(&self) -> &str {
        use PaymentLinksResourceAfterCompletionType::*;
        match self {
            HostedConfirmation => "hosted_confirmation",
            Redirect => "redirect",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentLinksResourceAfterCompletionType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinksResourceAfterCompletionType::*;
        match s {
            "hosted_confirmation" => Ok(HostedConfirmation),
            "redirect" => Ok(Redirect),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentLinksResourceAfterCompletionType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentLinksResourceAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentLinksResourceAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentLinksResourceAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentLinksResourceAfterCompletionType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentLinksResourceAfterCompletionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PaymentLinksResourceAfterCompletionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PaymentLinksResourceAfterCompletionType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentLinksResourceAfterCompletionType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentLinksResourceAfterCompletionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
