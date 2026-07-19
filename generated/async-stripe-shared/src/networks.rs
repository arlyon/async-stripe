#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Networks {
    /// All networks available for selection via [payment_method_options.card.network](/api/payment_intents/confirm#confirm_payment_intent-payment_method_options-card-network).
    pub available: Vec<String>,
    /// The preferred network for co-branded cards.
    /// Can be `cartes_bancaires`, `mastercard`, `visa` or `invalid_preference` if requested network is not valid for the card.
    pub preferred: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for Networks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Networks").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct NetworksBuilder {
    available: Option<Vec<String>>,
    preferred: Option<Option<String>>,
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

    impl Deserialize for Networks {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Networks>,
        builder: NetworksBuilder,
    }

    impl Visitor for Place<Networks> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: NetworksBuilder {
                    available: Deserialize::default(),
                    preferred: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "available" => Deserialize::begin(&mut self.builder.available),
                "preferred" => Deserialize::begin(&mut self.builder.preferred),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(available), Some(preferred)) =
                (self.builder.available.take(), self.builder.preferred.take())
            else {
                return Ok(());
            };
            *self.out = Some(Networks { available, preferred });
            Ok(())
        }
    }
};
