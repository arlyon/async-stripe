#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct UsBankAccountNetworks {
    /// The preferred network.
    pub preferred: Option<String>,
    /// All supported networks.
    pub supported: Vec<stripe_shared::UsBankAccountNetworksSupported>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UsBankAccountNetworks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UsBankAccountNetworks").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct UsBankAccountNetworksBuilder {
    preferred: Option<Option<String>>,
    supported: Option<Vec<stripe_shared::UsBankAccountNetworksSupported>>,
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

    impl Deserialize for UsBankAccountNetworks {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<UsBankAccountNetworks>,
        builder: UsBankAccountNetworksBuilder,
    }

    impl Visitor for Place<UsBankAccountNetworks> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: UsBankAccountNetworksBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for UsBankAccountNetworksBuilder {
        type Out = UsBankAccountNetworks;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "preferred" => Deserialize::begin(&mut self.preferred),
                "supported" => Deserialize::begin(&mut self.supported),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { preferred: Some(None), supported: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(preferred), Some(supported)) = (self.preferred.take(), self.supported.take())
            else {
                return None;
            };
            Some(Self::Out { preferred, supported })
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

    impl ObjectDeser for UsBankAccountNetworks {
        type Builder = UsBankAccountNetworksBuilder;
    }

    impl FromValueOpt for UsBankAccountNetworks {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = UsBankAccountNetworksBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "preferred" => b.preferred = FromValueOpt::from_value(v),
                    "supported" => b.supported = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// All supported networks.
pub type UsBankAccountNetworksSupported = stripe_shared::UsBankAccountNetworksSupported;
