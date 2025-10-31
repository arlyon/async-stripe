#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ChargeOutcome {
    /// An enumerated value providing a more detailed explanation on [how to proceed with an error](https://stripe.com/docs/declines#retrying-issuer-declines).
    pub advice_code: Option<ChargeOutcomeAdviceCode>,
    /// For charges declined by the network, a 2 digit code which indicates the advice returned by the network on how to proceed with an error.
    pub network_advice_code: Option<String>,
    /// For charges declined by the network, an alphanumeric code which indicates the reason the charge failed.
    pub network_decline_code: Option<String>,
    /// Possible values are `approved_by_network`, `declined_by_network`, `not_sent_to_network`, and `reversed_after_approval`.
    /// The value `reversed_after_approval` indicates the payment was [blocked by Stripe](https://stripe.com/docs/declines#blocked-payments) after bank authorization, and may temporarily appear as "pending" on a cardholder's statement.
    pub network_status: Option<String>,
    /// An enumerated value providing a more detailed explanation of the outcome's `type`.
    /// Charges blocked by Radar's default block rule have the value `highest_risk_level`.
    /// Charges placed in review by Radar's default review rule have the value `elevated_risk_level`.
    /// Charges blocked because the payment is unlikely to be authorized have the value `low_probability_of_authorization`.
    /// Charges authorized, blocked, or placed in review by custom rules have the value `rule`.
    /// See [understanding declines](https://stripe.com/docs/declines) for more details.
    pub reason: Option<String>,
    /// Stripe Radar's evaluation of the riskiness of the payment.
    /// Possible values for evaluated payments are `normal`, `elevated`, `highest`.
    /// For non-card payments, and card-based payments predating the public assignment of risk levels, this field will have the value `not_assessed`.
    /// In the event of an error in the evaluation, this field will have the value `unknown`.
    /// This field is only available with Radar.
    pub risk_level: Option<String>,
    /// Stripe Radar's evaluation of the riskiness of the payment.
    /// Possible values for evaluated payments are between 0 and 100.
    /// For non-card payments, card-based payments predating the public assignment of risk scores, or in the event of an error during evaluation, this field will not be present.
    /// This field is only available with Radar for Fraud Teams.
    pub risk_score: Option<i64>,
    /// The ID of the Radar rule that matched the payment, if applicable.
    pub rule: Option<stripe_types::Expandable<stripe_shared::Rule>>,
    /// A human-readable description of the outcome type and reason, designed for you (the recipient of the payment), not your customer.
    pub seller_message: Option<String>,
    /// Possible values are `authorized`, `manual_review`, `issuer_declined`, `blocked`, and `invalid`.
    /// See [understanding declines](https://stripe.com/docs/declines) and [Radar reviews](https://stripe.com/docs/radar/reviews) for details.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: String,
}
#[doc(hidden)]
pub struct ChargeOutcomeBuilder {
    advice_code: Option<Option<ChargeOutcomeAdviceCode>>,
    network_advice_code: Option<Option<String>>,
    network_decline_code: Option<Option<String>>,
    network_status: Option<Option<String>>,
    reason: Option<Option<String>>,
    risk_level: Option<Option<String>>,
    risk_score: Option<Option<i64>>,
    rule: Option<Option<stripe_types::Expandable<stripe_shared::Rule>>>,
    seller_message: Option<Option<String>>,
    type_: Option<String>,
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

    impl Deserialize for ChargeOutcome {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ChargeOutcome>,
        builder: ChargeOutcomeBuilder,
    }

    impl Visitor for Place<ChargeOutcome> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ChargeOutcomeBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ChargeOutcomeBuilder {
        type Out = ChargeOutcome;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "advice_code" => Deserialize::begin(&mut self.advice_code),
                "network_advice_code" => Deserialize::begin(&mut self.network_advice_code),
                "network_decline_code" => Deserialize::begin(&mut self.network_decline_code),
                "network_status" => Deserialize::begin(&mut self.network_status),
                "reason" => Deserialize::begin(&mut self.reason),
                "risk_level" => Deserialize::begin(&mut self.risk_level),
                "risk_score" => Deserialize::begin(&mut self.risk_score),
                "rule" => Deserialize::begin(&mut self.rule),
                "seller_message" => Deserialize::begin(&mut self.seller_message),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                advice_code: Deserialize::default(),
                network_advice_code: Deserialize::default(),
                network_decline_code: Deserialize::default(),
                network_status: Deserialize::default(),
                reason: Deserialize::default(),
                risk_level: Deserialize::default(),
                risk_score: Deserialize::default(),
                rule: Deserialize::default(),
                seller_message: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(advice_code),
                Some(network_advice_code),
                Some(network_decline_code),
                Some(network_status),
                Some(reason),
                Some(risk_level),
                Some(risk_score),
                Some(rule),
                Some(seller_message),
                Some(type_),
            ) = (
                self.advice_code,
                self.network_advice_code.take(),
                self.network_decline_code.take(),
                self.network_status.take(),
                self.reason.take(),
                self.risk_level.take(),
                self.risk_score,
                self.rule.take(),
                self.seller_message.take(),
                self.type_.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                advice_code,
                network_advice_code,
                network_decline_code,
                network_status,
                reason,
                risk_level,
                risk_score,
                rule,
                seller_message,
                type_,
            })
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

    impl ObjectDeser for ChargeOutcome {
        type Builder = ChargeOutcomeBuilder;
    }

    impl FromValueOpt for ChargeOutcome {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ChargeOutcomeBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "advice_code" => b.advice_code = FromValueOpt::from_value(v),
                    "network_advice_code" => b.network_advice_code = FromValueOpt::from_value(v),
                    "network_decline_code" => b.network_decline_code = FromValueOpt::from_value(v),
                    "network_status" => b.network_status = FromValueOpt::from_value(v),
                    "reason" => b.reason = FromValueOpt::from_value(v),
                    "risk_level" => b.risk_level = FromValueOpt::from_value(v),
                    "risk_score" => b.risk_score = FromValueOpt::from_value(v),
                    "rule" => b.rule = FromValueOpt::from_value(v),
                    "seller_message" => b.seller_message = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// An enumerated value providing a more detailed explanation on [how to proceed with an error](https://stripe.com/docs/declines#retrying-issuer-declines).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ChargeOutcomeAdviceCode {
    ConfirmCardData,
    DoNotTryAgain,
    TryAgainLater,
}
impl ChargeOutcomeAdviceCode {
    pub fn as_str(self) -> &'static str {
        use ChargeOutcomeAdviceCode::*;
        match self {
            ConfirmCardData => "confirm_card_data",
            DoNotTryAgain => "do_not_try_again",
            TryAgainLater => "try_again_later",
        }
    }
}

impl std::str::FromStr for ChargeOutcomeAdviceCode {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ChargeOutcomeAdviceCode::*;
        match s {
            "confirm_card_data" => Ok(ConfirmCardData),
            "do_not_try_again" => Ok(DoNotTryAgain),
            "try_again_later" => Ok(TryAgainLater),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ChargeOutcomeAdviceCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ChargeOutcomeAdviceCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ChargeOutcomeAdviceCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ChargeOutcomeAdviceCode {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ChargeOutcomeAdviceCode> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ChargeOutcomeAdviceCode::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ChargeOutcomeAdviceCode);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ChargeOutcomeAdviceCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ChargeOutcomeAdviceCode"))
    }
}
