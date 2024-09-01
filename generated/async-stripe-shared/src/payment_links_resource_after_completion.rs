#[derive(Clone, Debug)]
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
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: PaymentLinksResourceAfterCompletionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentLinksResourceAfterCompletionBuilder {
        type Out = PaymentLinksResourceAfterCompletion;
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
            let (Some(hosted_confirmation), Some(redirect), Some(type_)) =
                (self.hosted_confirmation.take(), self.redirect.take(), self.type_)
            else {
                return None;
            };
            Some(Self::Out { hosted_confirmation, redirect, type_ })
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

    impl ObjectDeser for PaymentLinksResourceAfterCompletion {
        type Builder = PaymentLinksResourceAfterCompletionBuilder;
    }

    impl FromValueOpt for PaymentLinksResourceAfterCompletion {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentLinksResourceAfterCompletionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "hosted_confirmation" => b.hosted_confirmation = FromValueOpt::from_value(v),
                    "redirect" => b.redirect = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The specified behavior after the purchase is complete.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentLinksResourceAfterCompletionType {
    HostedConfirmation,
    Redirect,
}
impl PaymentLinksResourceAfterCompletionType {
    pub fn as_str(self) -> &'static str {
        use PaymentLinksResourceAfterCompletionType::*;
        match self {
            HostedConfirmation => "hosted_confirmation",
            Redirect => "redirect",
        }
    }
}

impl std::str::FromStr for PaymentLinksResourceAfterCompletionType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinksResourceAfterCompletionType::*;
        match s {
            "hosted_confirmation" => Ok(HostedConfirmation),
            "redirect" => Ok(Redirect),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentLinksResourceAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentLinksResourceAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for PaymentLinksResourceAfterCompletionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentLinksResourceAfterCompletionType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentLinksResourceAfterCompletionType::from_str(s).map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentLinksResourceAfterCompletionType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentLinksResourceAfterCompletionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PaymentLinksResourceAfterCompletionType")
        })
    }
}
