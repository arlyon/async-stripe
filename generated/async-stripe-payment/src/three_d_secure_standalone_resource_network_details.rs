/// Contains details specific to the individual network.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ThreeDSecureStandaloneResourceNetworkDetails {
    pub cartes_bancaires: Option<stripe_payment::ThreeDSecureStandaloneResourceCartesBancaires>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ThreeDSecureStandaloneResourceNetworkDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ThreeDSecureStandaloneResourceNetworkDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ThreeDSecureStandaloneResourceNetworkDetailsBuilder {
    cartes_bancaires: Option<Option<stripe_payment::ThreeDSecureStandaloneResourceCartesBancaires>>,
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

    impl Deserialize for ThreeDSecureStandaloneResourceNetworkDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ThreeDSecureStandaloneResourceNetworkDetails>,
        builder: ThreeDSecureStandaloneResourceNetworkDetailsBuilder,
    }

    impl Visitor for Place<ThreeDSecureStandaloneResourceNetworkDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ThreeDSecureStandaloneResourceNetworkDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ThreeDSecureStandaloneResourceNetworkDetailsBuilder {
        type Out = ThreeDSecureStandaloneResourceNetworkDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "cartes_bancaires" => Deserialize::begin(&mut self.cartes_bancaires),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { cartes_bancaires: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(cartes_bancaires),) = (self.cartes_bancaires.take(),) else {
                return None;
            };
            Some(Self::Out { cartes_bancaires })
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

    impl ObjectDeser for ThreeDSecureStandaloneResourceNetworkDetails {
        type Builder = ThreeDSecureStandaloneResourceNetworkDetailsBuilder;
    }

    impl FromValueOpt for ThreeDSecureStandaloneResourceNetworkDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ThreeDSecureStandaloneResourceNetworkDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "cartes_bancaires" => b.cartes_bancaires = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
