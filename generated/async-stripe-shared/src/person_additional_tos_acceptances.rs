#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PersonAdditionalTosAcceptances {
    /// Details on the legal guardian's acceptance of the main Stripe service agreement.
    pub account: Option<stripe_shared::PersonAdditionalTosAcceptance>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PersonAdditionalTosAcceptances {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PersonAdditionalTosAcceptances").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PersonAdditionalTosAcceptancesBuilder {
    account: Option<Option<stripe_shared::PersonAdditionalTosAcceptance>>,
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

    impl Deserialize for PersonAdditionalTosAcceptances {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PersonAdditionalTosAcceptances>,
        builder: PersonAdditionalTosAcceptancesBuilder,
    }

    impl Visitor for Place<PersonAdditionalTosAcceptances> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PersonAdditionalTosAcceptancesBuilder { account: Deserialize::default() },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account" => Deserialize::begin(&mut self.builder.account),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(account),) = (self.builder.account.take(),) else {
                return Ok(());
            };
            *self.out = Some(PersonAdditionalTosAcceptances { account });
            Ok(())
        }
    }
};
