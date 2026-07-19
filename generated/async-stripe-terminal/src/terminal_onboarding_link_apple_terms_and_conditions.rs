/// Options associated with the Apple Terms and Conditions link type.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalOnboardingLinkAppleTermsAndConditions {
    /// Whether the link should also support users relinking their Apple account.
    pub allow_relinking: Option<bool>,
    /// The business name of the merchant accepting Apple's Terms and Conditions.
    pub merchant_display_name: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalOnboardingLinkAppleTermsAndConditions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalOnboardingLinkAppleTermsAndConditions").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TerminalOnboardingLinkAppleTermsAndConditionsBuilder {
    allow_relinking: Option<Option<bool>>,
    merchant_display_name: Option<String>,
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

    impl Deserialize for TerminalOnboardingLinkAppleTermsAndConditions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalOnboardingLinkAppleTermsAndConditions>,
        builder: TerminalOnboardingLinkAppleTermsAndConditionsBuilder,
    }

    impl Visitor for Place<TerminalOnboardingLinkAppleTermsAndConditions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TerminalOnboardingLinkAppleTermsAndConditionsBuilder {
                    allow_relinking: Deserialize::default(),
                    merchant_display_name: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "allow_relinking" => Deserialize::begin(&mut self.builder.allow_relinking),
                "merchant_display_name" => {
                    Deserialize::begin(&mut self.builder.merchant_display_name)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(allow_relinking), Some(merchant_display_name)) =
                (self.builder.allow_relinking, self.builder.merchant_display_name.take())
            else {
                return Ok(());
            };
            *self.out = Some(TerminalOnboardingLinkAppleTermsAndConditions {
                allow_relinking,
                merchant_display_name,
            });
            Ok(())
        }
    }
};
