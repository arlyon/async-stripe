#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TokenCardNetworks {
    /// The preferred network for co-branded cards.
    /// Can be `cartes_bancaires`, `mastercard`, `visa` or `invalid_preference` if requested network is not valid for the card.
    pub preferred: Option<String>,
}
#[doc(hidden)]
pub struct TokenCardNetworksBuilder {
    preferred: Option<Option<String>>,
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

    impl Deserialize for TokenCardNetworks {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TokenCardNetworks>,
        builder: TokenCardNetworksBuilder,
    }

    impl Visitor for Place<TokenCardNetworks> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TokenCardNetworksBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TokenCardNetworksBuilder {
        type Out = TokenCardNetworks;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "preferred" => Deserialize::begin(&mut self.preferred),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { preferred: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(preferred),) = (self.preferred.take(),) else {
                return None;
            };
            Some(Self::Out { preferred })
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

    impl ObjectDeser for TokenCardNetworks {
        type Builder = TokenCardNetworksBuilder;
    }

    impl FromValueOpt for TokenCardNetworks {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TokenCardNetworksBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "preferred" => b.preferred = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
