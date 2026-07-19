#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingNetworkTokenVisa {
    /// A unique reference ID from Visa to represent the card account number.
    pub card_reference_id: Option<String>,
    /// The network-unique identifier for the token.
    pub token_reference_id: String,
    /// The ID of the entity requesting tokenization, specific to Visa.
    pub token_requestor_id: String,
    /// Degree of risk associated with the token between `01` and `99`, with higher number indicating higher risk.
    /// A `00` value indicates the token was not scored by Visa.
    pub token_risk_score: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingNetworkTokenVisa {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingNetworkTokenVisa").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingNetworkTokenVisaBuilder {
    card_reference_id: Option<Option<String>>,
    token_reference_id: Option<String>,
    token_requestor_id: Option<String>,
    token_risk_score: Option<Option<String>>,
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
                builder: IssuingNetworkTokenVisaBuilder {
                    card_reference_id: Deserialize::default(),
                    token_reference_id: Deserialize::default(),
                    token_requestor_id: Deserialize::default(),
                    token_risk_score: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "card_reference_id" => Deserialize::begin(&mut self.builder.card_reference_id),
                "token_reference_id" => Deserialize::begin(&mut self.builder.token_reference_id),
                "token_requestor_id" => Deserialize::begin(&mut self.builder.token_requestor_id),
                "token_risk_score" => Deserialize::begin(&mut self.builder.token_risk_score),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(card_reference_id),
                Some(token_reference_id),
                Some(token_requestor_id),
                Some(token_risk_score),
            ) = (
                self.builder.card_reference_id.take(),
                self.builder.token_reference_id.take(),
                self.builder.token_requestor_id.take(),
                self.builder.token_risk_score.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(IssuingNetworkTokenVisa {
                card_reference_id,
                token_reference_id,
                token_requestor_id,
                token_risk_score,
            });
            Ok(())
        }
    }
};
