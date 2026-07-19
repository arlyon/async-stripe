#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingNetworkTokenAddress {
    /// The street address of the cardholder tokenizing the card.
    pub line1: String,
    /// The postal code of the cardholder tokenizing the card.
    pub postal_code: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingNetworkTokenAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingNetworkTokenAddress").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingNetworkTokenAddressBuilder {
    line1: Option<String>,
    postal_code: Option<String>,
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

    impl Deserialize for IssuingNetworkTokenAddress {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingNetworkTokenAddress>,
        builder: IssuingNetworkTokenAddressBuilder,
    }

    impl Visitor for Place<IssuingNetworkTokenAddress> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingNetworkTokenAddressBuilder {
                    line1: Deserialize::default(),
                    postal_code: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "line1" => Deserialize::begin(&mut self.builder.line1),
                "postal_code" => Deserialize::begin(&mut self.builder.postal_code),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(line1), Some(postal_code)) =
                (self.builder.line1.take(), self.builder.postal_code.take())
            else {
                return Ok(());
            };
            *self.out = Some(IssuingNetworkTokenAddress { line1, postal_code });
            Ok(())
        }
    }
};
