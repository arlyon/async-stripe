#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasurySharedResourceBillingDetails {
    pub address: stripe_shared::Address,
    /// Email address.
    pub email: Option<String>,
    /// Full name.
    pub name: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasurySharedResourceBillingDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasurySharedResourceBillingDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasurySharedResourceBillingDetailsBuilder {
    address: Option<stripe_shared::Address>,
    email: Option<Option<String>>,
    name: Option<Option<String>>,
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

    impl Deserialize for TreasurySharedResourceBillingDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasurySharedResourceBillingDetails>,
        builder: TreasurySharedResourceBillingDetailsBuilder,
    }

    impl Visitor for Place<TreasurySharedResourceBillingDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasurySharedResourceBillingDetailsBuilder {
                    address: Deserialize::default(),
                    email: Deserialize::default(),
                    name: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address" => Deserialize::begin(&mut self.builder.address),
                "email" => Deserialize::begin(&mut self.builder.email),
                "name" => Deserialize::begin(&mut self.builder.name),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(address), Some(email), Some(name)) =
                (self.builder.address.take(), self.builder.email.take(), self.builder.name.take())
            else {
                return Ok(());
            };
            *self.out = Some(TreasurySharedResourceBillingDetails { address, email, name });
            Ok(())
        }
    }
};
