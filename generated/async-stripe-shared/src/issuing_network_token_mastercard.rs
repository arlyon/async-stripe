#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingNetworkTokenMastercard {
    /// A unique reference ID from MasterCard to represent the card account number.
    pub card_reference_id: Option<String>,
    /// The network-unique identifier for the token.
    pub token_reference_id: String,
    /// The ID of the entity requesting tokenization, specific to MasterCard.
    pub token_requestor_id: String,
    /// The name of the entity requesting tokenization, if known.
    /// This is directly provided from MasterCard.
    pub token_requestor_name: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingNetworkTokenMastercard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingNetworkTokenMastercard").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingNetworkTokenMastercardBuilder {
    card_reference_id: Option<Option<String>>,
    token_reference_id: Option<String>,
    token_requestor_id: Option<String>,
    token_requestor_name: Option<Option<String>>,
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

    impl Deserialize for IssuingNetworkTokenMastercard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingNetworkTokenMastercard>,
        builder: IssuingNetworkTokenMastercardBuilder,
    }

    impl Visitor for Place<IssuingNetworkTokenMastercard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingNetworkTokenMastercardBuilder {
                    card_reference_id: Deserialize::default(),
                    token_reference_id: Deserialize::default(),
                    token_requestor_id: Deserialize::default(),
                    token_requestor_name: Deserialize::default(),
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
                "token_requestor_name" => {
                    Deserialize::begin(&mut self.builder.token_requestor_name)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(card_reference_id),
                Some(token_reference_id),
                Some(token_requestor_id),
                Some(token_requestor_name),
            ) = (
                self.builder.card_reference_id.take(),
                self.builder.token_reference_id.take(),
                self.builder.token_requestor_id.take(),
                self.builder.token_requestor_name.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(IssuingNetworkTokenMastercard {
                card_reference_id,
                token_reference_id,
                token_requestor_id,
                token_requestor_name,
            });
            Ok(())
        }
    }
};
