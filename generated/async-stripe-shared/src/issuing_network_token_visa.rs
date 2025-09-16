#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingNetworkTokenVisa {
    /// A unique reference ID from Visa to represent the card account number.
    pub card_reference_id: String,
    /// The network-unique identifier for the token.
    pub token_reference_id: String,
    /// The ID of the entity requesting tokenization, specific to Visa.
    pub token_requestor_id: String,
    /// Degree of risk associated with the token between `01` and `99`, with higher number indicating higher risk.
    /// A `00` value indicates the token was not scored by Visa.
    pub token_risk_score: Option<String>,
}
#[doc(hidden)]
pub struct IssuingNetworkTokenVisaBuilder {
    card_reference_id: Option<String>,
    token_reference_id: Option<String>,
    token_requestor_id: Option<String>,
    token_risk_score: Option<Option<String>>,
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

    impl Deserialize for IssuingNetworkTokenVisa {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingNetworkTokenVisa>,
        builder: IssuingNetworkTokenVisaBuilder,
    }

    impl Visitor for Place<IssuingNetworkTokenVisa> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingNetworkTokenVisaBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingNetworkTokenVisaBuilder {
        type Out = IssuingNetworkTokenVisa;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "card_reference_id" => Deserialize::begin(&mut self.card_reference_id),
                "token_reference_id" => Deserialize::begin(&mut self.token_reference_id),
                "token_requestor_id" => Deserialize::begin(&mut self.token_requestor_id),
                "token_risk_score" => Deserialize::begin(&mut self.token_risk_score),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                card_reference_id: Deserialize::default(),
                token_reference_id: Deserialize::default(),
                token_requestor_id: Deserialize::default(),
                token_risk_score: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(card_reference_id),
                Some(token_reference_id),
                Some(token_requestor_id),
                Some(token_risk_score),
            ) = (
                self.card_reference_id.take(),
                self.token_reference_id.take(),
                self.token_requestor_id.take(),
                self.token_risk_score.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                card_reference_id,
                token_reference_id,
                token_requestor_id,
                token_risk_score,
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

    impl ObjectDeser for IssuingNetworkTokenVisa {
        type Builder = IssuingNetworkTokenVisaBuilder;
    }

    impl FromValueOpt for IssuingNetworkTokenVisa {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingNetworkTokenVisaBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "card_reference_id" => b.card_reference_id = FromValueOpt::from_value(v),
                    "token_reference_id" => b.token_reference_id = FromValueOpt::from_value(v),
                    "token_requestor_id" => b.token_requestor_id = FromValueOpt::from_value(v),
                    "token_risk_score" => b.token_risk_score = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
