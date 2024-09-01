#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConnectEmbeddedAccountFeaturesClaim {
    /// Whether to allow platforms to control bank account collection for their connected accounts.
    /// This feature can only be false for custom accounts (or accounts where the platform is compliance owner).
    /// Otherwise, bank account collection is determined by compliance requirements.
    pub external_account_collection: bool,
}
#[doc(hidden)]
pub struct ConnectEmbeddedAccountFeaturesClaimBuilder {
    external_account_collection: Option<bool>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
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

    impl Deserialize for ConnectEmbeddedAccountFeaturesClaim {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ConnectEmbeddedAccountFeaturesClaim>,
        builder: ConnectEmbeddedAccountFeaturesClaimBuilder,
    }

    impl Visitor for Place<ConnectEmbeddedAccountFeaturesClaim> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ConnectEmbeddedAccountFeaturesClaimBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ConnectEmbeddedAccountFeaturesClaimBuilder {
        type Out = ConnectEmbeddedAccountFeaturesClaim;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "external_account_collection" => {
                    Deserialize::begin(&mut self.external_account_collection)
                }

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { external_account_collection: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(external_account_collection),) = (self.external_account_collection,) else {
                return None;
            };
            Some(Self::Out { external_account_collection })
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

    impl ObjectDeser for ConnectEmbeddedAccountFeaturesClaim {
        type Builder = ConnectEmbeddedAccountFeaturesClaimBuilder;
    }

    impl FromValueOpt for ConnectEmbeddedAccountFeaturesClaim {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ConnectEmbeddedAccountFeaturesClaimBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "external_account_collection" => {
                        b.external_account_collection = FromValueOpt::from_value(v)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
