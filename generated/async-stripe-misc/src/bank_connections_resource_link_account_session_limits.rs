#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BankConnectionsResourceLinkAccountSessionLimits {
    /// The number of accounts that can be linked in this Session.
    pub accounts: i64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BankConnectionsResourceLinkAccountSessionLimits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BankConnectionsResourceLinkAccountSessionLimits").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BankConnectionsResourceLinkAccountSessionLimitsBuilder {
    accounts: Option<i64>,
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

    impl Deserialize for BankConnectionsResourceLinkAccountSessionLimits {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BankConnectionsResourceLinkAccountSessionLimits>,
        builder: BankConnectionsResourceLinkAccountSessionLimitsBuilder,
    }

    impl Visitor for Place<BankConnectionsResourceLinkAccountSessionLimits> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BankConnectionsResourceLinkAccountSessionLimitsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BankConnectionsResourceLinkAccountSessionLimitsBuilder {
        type Out = BankConnectionsResourceLinkAccountSessionLimits;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "accounts" => Deserialize::begin(&mut self.accounts),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { accounts: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(accounts),) = (self.accounts,) else {
                return None;
            };
            Some(Self::Out { accounts })
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

    impl ObjectDeser for BankConnectionsResourceLinkAccountSessionLimits {
        type Builder = BankConnectionsResourceLinkAccountSessionLimitsBuilder;
    }

    impl FromValueOpt for BankConnectionsResourceLinkAccountSessionLimits {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BankConnectionsResourceLinkAccountSessionLimitsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "accounts" => b.accounts = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
