/// Details of an succeeded card outcome attached to this payment evaluation.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InsightsResourcesPaymentEvaluationSucceededCard {
    /// Result of the address line 1 check.
    pub address_line1_check: stripe_shared::InsightsResourcesPaymentEvaluationRejectedCardCvcCheck,
    /// Indicates whether the cardholder provided a postal code and if it matched the cardholder’s billing address.
    pub address_postal_code_check:
        stripe_shared::InsightsResourcesPaymentEvaluationRejectedCardCvcCheck,
    /// Result of the CVC check.
    pub cvc_check: stripe_shared::InsightsResourcesPaymentEvaluationRejectedCardCvcCheck,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InsightsResourcesPaymentEvaluationSucceededCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InsightsResourcesPaymentEvaluationSucceededCard").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InsightsResourcesPaymentEvaluationSucceededCardBuilder {
    address_line1_check:
        Option<stripe_shared::InsightsResourcesPaymentEvaluationRejectedCardCvcCheck>,
    address_postal_code_check:
        Option<stripe_shared::InsightsResourcesPaymentEvaluationRejectedCardCvcCheck>,
    cvc_check: Option<stripe_shared::InsightsResourcesPaymentEvaluationRejectedCardCvcCheck>,
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

    impl Deserialize for InsightsResourcesPaymentEvaluationSucceededCard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InsightsResourcesPaymentEvaluationSucceededCard>,
        builder: InsightsResourcesPaymentEvaluationSucceededCardBuilder,
    }

    impl Visitor for Place<InsightsResourcesPaymentEvaluationSucceededCard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InsightsResourcesPaymentEvaluationSucceededCardBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InsightsResourcesPaymentEvaluationSucceededCardBuilder {
        type Out = InsightsResourcesPaymentEvaluationSucceededCard;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address_line1_check" => Deserialize::begin(&mut self.address_line1_check),
                "address_postal_code_check" => {
                    Deserialize::begin(&mut self.address_postal_code_check)
                }
                "cvc_check" => Deserialize::begin(&mut self.cvc_check),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { address_line1_check: None, address_postal_code_check: None, cvc_check: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(address_line1_check), Some(address_postal_code_check), Some(cvc_check)) = (
                self.address_line1_check.take(),
                self.address_postal_code_check.take(),
                self.cvc_check.take(),
            ) else {
                return None;
            };
            Some(Self::Out { address_line1_check, address_postal_code_check, cvc_check })
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

    impl ObjectDeser for InsightsResourcesPaymentEvaluationSucceededCard {
        type Builder = InsightsResourcesPaymentEvaluationSucceededCardBuilder;
    }

    impl FromValueOpt for InsightsResourcesPaymentEvaluationSucceededCard {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InsightsResourcesPaymentEvaluationSucceededCardBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "address_line1_check" => b.address_line1_check = FromValueOpt::from_value(v),
                    "address_postal_code_check" => {
                        b.address_postal_code_check = FromValueOpt::from_value(v)
                    }
                    "cvc_check" => b.cvc_check = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Result of the address line 1 check.
pub type InsightsResourcesPaymentEvaluationSucceededCardAddressLine1Check =
    stripe_shared::InsightsResourcesPaymentEvaluationRejectedCardCvcCheck;
/// Indicates whether the cardholder provided a postal code and if it matched the cardholder’s billing address.
pub type InsightsResourcesPaymentEvaluationSucceededCardAddressPostalCodeCheck =
    stripe_shared::InsightsResourcesPaymentEvaluationRejectedCardCvcCheck;
/// Result of the CVC check.
pub type InsightsResourcesPaymentEvaluationSucceededCardCvcCheck =
    stripe_shared::InsightsResourcesPaymentEvaluationRejectedCardCvcCheck;
