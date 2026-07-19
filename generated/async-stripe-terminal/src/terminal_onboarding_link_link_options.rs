/// Link type options associated with the current onboarding link object.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalOnboardingLinkLinkOptions {
    /// The options associated with the Apple Terms and Conditions link type.
    pub apple_terms_and_conditions:
        Option<stripe_terminal::TerminalOnboardingLinkAppleTermsAndConditions>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalOnboardingLinkLinkOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalOnboardingLinkLinkOptions").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TerminalOnboardingLinkLinkOptionsBuilder {
    apple_terms_and_conditions:
        Option<Option<stripe_terminal::TerminalOnboardingLinkAppleTermsAndConditions>>,
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
                builder: TerminalOnboardingLinkLinkOptionsBuilder {
                    apple_terms_and_conditions: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "apple_terms_and_conditions" => {
                    Deserialize::begin(&mut self.builder.apple_terms_and_conditions)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(apple_terms_and_conditions),) =
                (self.builder.apple_terms_and_conditions.take(),)
            else {
                return Ok(());
            };
            *self.out = Some(TerminalOnboardingLinkLinkOptions { apple_terms_and_conditions });
            Ok(())
        }
    }
};
