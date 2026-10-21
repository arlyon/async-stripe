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
                builder: TerminalOnboardingLinkAppleTermsAndConditionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TerminalOnboardingLinkAppleTermsAndConditionsBuilder {
        type Out = TerminalOnboardingLinkAppleTermsAndConditions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "allow_relinking" => Deserialize::begin(&mut self.allow_relinking),
                "merchant_display_name" => Deserialize::begin(&mut self.merchant_display_name),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { allow_relinking: Some(None), merchant_display_name: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    allow_relinking: Some(allow_relinking),
                    merchant_display_name: Some(merchant_display_name),
                } => Some(Self::Out { allow_relinking, merchant_display_name }),
                builder => {
                    *self = builder;
                    None
                }
            }
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

    impl ObjectDeser for TerminalOnboardingLinkAppleTermsAndConditions {
        type Builder = TerminalOnboardingLinkAppleTermsAndConditionsBuilder;
    }

    impl FromValueOpt for TerminalOnboardingLinkAppleTermsAndConditions {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
