#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BankConnectionsResourceAccountStatusDetails {
    pub active: Option<
        stripe_misc::BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetails,
    >,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BankConnectionsResourceAccountStatusDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BankConnectionsResourceAccountStatusDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BankConnectionsResourceAccountStatusDetailsBuilder {
    active: Option<
        Option<
            stripe_misc::BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetails,
        >,
    >,
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

    impl Deserialize for BankConnectionsResourceAccountStatusDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BankConnectionsResourceAccountStatusDetails>,
        builder: BankConnectionsResourceAccountStatusDetailsBuilder,
    }

    impl Visitor for Place<BankConnectionsResourceAccountStatusDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BankConnectionsResourceAccountStatusDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BankConnectionsResourceAccountStatusDetailsBuilder {
        type Out = BankConnectionsResourceAccountStatusDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "active" => Deserialize::begin(&mut self.active),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { active: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(active),) = (self.active.take(),) else {
                return None;
            };
            Some(Self::Out { active })
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

    impl ObjectDeser for BankConnectionsResourceAccountStatusDetails {
        type Builder = BankConnectionsResourceAccountStatusDetailsBuilder;
    }

    impl FromValueOpt for BankConnectionsResourceAccountStatusDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BankConnectionsResourceAccountStatusDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "active" => b.active = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
