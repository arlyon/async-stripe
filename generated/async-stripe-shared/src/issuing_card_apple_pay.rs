#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingCardApplePay {
    /// Apple Pay Eligibility
    pub eligible: bool,
    /// Reason the card is ineligible for Apple Pay
    pub ineligible_reason: Option<stripe_shared::IneligibleReason>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingCardApplePay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingCardApplePay").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingCardApplePayBuilder {
    eligible: Option<bool>,
    ineligible_reason: Option<Option<stripe_shared::IneligibleReason>>,
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

    impl Deserialize for IssuingCardApplePay {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingCardApplePay>,
        builder: IssuingCardApplePayBuilder,
    }

    impl Visitor for Place<IssuingCardApplePay> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingCardApplePayBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingCardApplePayBuilder {
        type Out = IssuingCardApplePay;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "eligible" => Deserialize::begin(&mut self.eligible),
                "ineligible_reason" => Deserialize::begin(&mut self.ineligible_reason),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { eligible: None, ineligible_reason: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(eligible), Some(ineligible_reason)) =
                (self.eligible, self.ineligible_reason.take())
            else {
                return None;
            };
            Some(Self::Out { eligible, ineligible_reason })
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

    impl ObjectDeser for IssuingCardApplePay {
        type Builder = IssuingCardApplePayBuilder;
    }

    impl FromValueOpt for IssuingCardApplePay {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingCardApplePayBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "eligible" => b.eligible = FromValueOpt::from_value(v),
                    "ineligible_reason" => b.ineligible_reason = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Reason the card is ineligible for Apple Pay
pub type IssuingCardApplePayIneligibleReason = stripe_shared::IneligibleReason;
