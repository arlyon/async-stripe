#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountGroupMembership {
    /// The group the account is in to determine their payments pricing, and null if the account is on customized pricing.
    /// [See the Platform pricing tool documentation](https://docs.stripe.com/connect/platform-pricing-tools) for details.
    pub payments_pricing: Option<String>,
}
#[doc(hidden)]
pub struct AccountGroupMembershipBuilder {
    payments_pricing: Option<Option<String>>,
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

    impl Deserialize for AccountGroupMembership {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountGroupMembership>,
        builder: AccountGroupMembershipBuilder,
    }

    impl Visitor for Place<AccountGroupMembership> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountGroupMembershipBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AccountGroupMembershipBuilder {
        type Out = AccountGroupMembership;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "payments_pricing" => Deserialize::begin(&mut self.payments_pricing),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { payments_pricing: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(payments_pricing),) = (self.payments_pricing.take(),) else {
                return None;
            };
            Some(Self::Out { payments_pricing })
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

    impl ObjectDeser for AccountGroupMembership {
        type Builder = AccountGroupMembershipBuilder;
    }

    impl FromValueOpt for AccountGroupMembership {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = AccountGroupMembershipBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "payments_pricing" => b.payments_pricing = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
