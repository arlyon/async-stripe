#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BankConnectionsResourceAuthorizationStatusDetails {
pub active: Option<stripe_misc::BankConnectionsResourceAuthorizationStatusDetailsApiResourceActiveStatusDetails>,
pub inactive: Option<stripe_misc::BankConnectionsResourceAuthorizationStatusDetailsApiResourceInactiveStatusDetails>,

}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BankConnectionsResourceAuthorizationStatusDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BankConnectionsResourceAuthorizationStatusDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BankConnectionsResourceAuthorizationStatusDetailsBuilder {
    active: Option<Option<stripe_misc::BankConnectionsResourceAuthorizationStatusDetailsApiResourceActiveStatusDetails>>,
inactive: Option<Option<stripe_misc::BankConnectionsResourceAuthorizationStatusDetailsApiResourceInactiveStatusDetails>>,

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

    impl Deserialize for BankConnectionsResourceAuthorizationStatusDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BankConnectionsResourceAuthorizationStatusDetails>,
        builder: BankConnectionsResourceAuthorizationStatusDetailsBuilder,
    }

    impl Visitor for Place<BankConnectionsResourceAuthorizationStatusDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BankConnectionsResourceAuthorizationStatusDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BankConnectionsResourceAuthorizationStatusDetailsBuilder {
        type Out = BankConnectionsResourceAuthorizationStatusDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "active" => Deserialize::begin(&mut self.active),
                "inactive" => Deserialize::begin(&mut self.inactive),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { active: Some(None), inactive: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(active), Some(inactive)) = (self.active.take(), self.inactive.take()) else {
                return None;
            };
            Some(Self::Out { active, inactive })
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

    impl ObjectDeser for BankConnectionsResourceAuthorizationStatusDetails {
        type Builder = BankConnectionsResourceAuthorizationStatusDetailsBuilder;
    }

    impl FromValueOpt for BankConnectionsResourceAuthorizationStatusDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BankConnectionsResourceAuthorizationStatusDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "active" => b.active = FromValueOpt::from_value(v),
                    "inactive" => b.inactive = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
