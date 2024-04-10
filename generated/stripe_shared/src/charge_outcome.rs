#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ChargeOutcome {
    /// Possible values are `approved_by_network`, `declined_by_network`, `not_sent_to_network`, and `reversed_after_approval`.
    /// The value `reversed_after_approval` indicates the payment was [blocked by Stripe](https://stripe.com/docs/declines#blocked-payments) after bank authorization, and may temporarily appear as "pending" on a cardholder's statement.
    pub network_status: Option<String>,
    /// An enumerated value providing a more detailed explanation of the outcome's `type`.
    /// Charges blocked by Radar's default block rule have the value `highest_risk_level`.
    /// Charges placed in review by Radar's default review rule have the value `elevated_risk_level`.
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
    network_status: Option<Option<String>>,
    reason: Option<Option<String>>,
    risk_level: Option<Option<String>>,
    risk_score: Option<Option<i64>>,
    rule: Option<Option<stripe_types::Expandable<stripe_shared::Rule>>>,
    seller_message: Option<Option<String>>,
    type_: Option<String>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
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
            Some(Self::Out {
                network_status: self.network_status.take()?,
                reason: self.reason.take()?,
                risk_level: self.risk_level.take()?,
                risk_score: self.risk_score?,
                rule: self.rule.take()?,
                seller_message: self.seller_message.take()?,
                type_: self.type_.take()?,
            })
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
                    "network_status" => b.network_status = Some(FromValueOpt::from_value(v)?),
                    "reason" => b.reason = Some(FromValueOpt::from_value(v)?),
                    "risk_level" => b.risk_level = Some(FromValueOpt::from_value(v)?),
                    "risk_score" => b.risk_score = Some(FromValueOpt::from_value(v)?),
                    "rule" => b.rule = Some(FromValueOpt::from_value(v)?),
                    "seller_message" => b.seller_message = Some(FromValueOpt::from_value(v)?),
                    "type" => b.type_ = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
