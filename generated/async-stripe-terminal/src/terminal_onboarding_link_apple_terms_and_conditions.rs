/// Options associated with the Apple Terms and Conditions link type.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalOnboardingLinkAppleTermsAndConditions {
    /// Whether the link should also support users relinking their Apple account.
    pub allow_relinking: Option<bool>,
    /// The business name of the merchant accepting Apple's Terms and Conditions.
    pub merchant_display_name: String,
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
            Self {
                allow_relinking: Deserialize::default(),
                merchant_display_name: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(allow_relinking), Some(merchant_display_name)) =
                (self.allow_relinking, self.merchant_display_name.take())
            else {
                return None;
            };
            Some(Self::Out { allow_relinking, merchant_display_name })
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

    impl ObjectDeser for TerminalOnboardingLinkAppleTermsAndConditions {
        type Builder = TerminalOnboardingLinkAppleTermsAndConditionsBuilder;
    }

    impl FromValueOpt for TerminalOnboardingLinkAppleTermsAndConditions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TerminalOnboardingLinkAppleTermsAndConditionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "allow_relinking" => b.allow_relinking = FromValueOpt::from_value(v),
                    "merchant_display_name" => {
                        b.merchant_display_name = FromValueOpt::from_value(v)
                    }
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
