/// Link type options associated with the current onboarding link object.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalOnboardingLinkLinkOptions {
    /// The options associated with the Apple Terms and Conditions link type.
    pub apple_terms_and_conditions:
        Option<stripe_terminal::TerminalOnboardingLinkAppleTermsAndConditions>,
}
#[doc(hidden)]
pub struct TerminalOnboardingLinkLinkOptionsBuilder {
    apple_terms_and_conditions:
        Option<Option<stripe_terminal::TerminalOnboardingLinkAppleTermsAndConditions>>,
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

    impl Deserialize for TerminalOnboardingLinkLinkOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalOnboardingLinkLinkOptions>,
        builder: TerminalOnboardingLinkLinkOptionsBuilder,
    }

    impl Visitor for Place<TerminalOnboardingLinkLinkOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TerminalOnboardingLinkLinkOptionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TerminalOnboardingLinkLinkOptionsBuilder {
        type Out = TerminalOnboardingLinkLinkOptions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "apple_terms_and_conditions" => {
                    Deserialize::begin(&mut self.apple_terms_and_conditions)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { apple_terms_and_conditions: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(apple_terms_and_conditions),) = (self.apple_terms_and_conditions.take(),)
            else {
                return None;
            };
            Some(Self::Out { apple_terms_and_conditions })
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

    impl ObjectDeser for TerminalOnboardingLinkLinkOptions {
        type Builder = TerminalOnboardingLinkLinkOptionsBuilder;
    }

    impl FromValueOpt for TerminalOnboardingLinkLinkOptions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TerminalOnboardingLinkLinkOptionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "apple_terms_and_conditions" => {
                        b.apple_terms_and_conditions = FromValueOpt::from_value(v)
                    }
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
