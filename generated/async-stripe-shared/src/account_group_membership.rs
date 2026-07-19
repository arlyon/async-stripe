#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountGroupMembership {
    /// The group the account is in to determine their payments pricing, and null if the account is on customized pricing.
    /// [See the Platform pricing tool documentation](https://docs.stripe.com/connect/platform-pricing-tools) for details.
    pub payments_pricing: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountGroupMembership {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccountGroupMembership").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct AccountGroupMembershipBuilder {
    payments_pricing: Option<Option<String>>,
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
                builder: AccountGroupMembershipBuilder { payments_pricing: Deserialize::default() },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "payments_pricing" => Deserialize::begin(&mut self.builder.payments_pricing),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(payments_pricing),) = (self.builder.payments_pricing.take(),) else {
                return Ok(());
            };
            *self.out = Some(AccountGroupMembership { payments_pricing });
            Ok(())
        }
    }
};
