#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BankConnectionsResourceBalanceApiResourceCreditBalance {
    /// The credit that has been used by the account holder.
    ///
    /// Each key is a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Each value is a integer amount.
    /// A positive amount indicates money owed to the account holder.
    /// A negative amount indicates money owed by the account holder.
    pub used: Option<std::collections::HashMap<String, i64>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BankConnectionsResourceBalanceApiResourceCreditBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BankConnectionsResourceBalanceApiResourceCreditBalance")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BankConnectionsResourceBalanceApiResourceCreditBalanceBuilder {
    used: Option<Option<std::collections::HashMap<String, i64>>>,
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

    impl Deserialize for BankConnectionsResourceBalanceApiResourceCreditBalance {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BankConnectionsResourceBalanceApiResourceCreditBalance>,
        builder: BankConnectionsResourceBalanceApiResourceCreditBalanceBuilder,
    }

    impl Visitor for Place<BankConnectionsResourceBalanceApiResourceCreditBalance> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BankConnectionsResourceBalanceApiResourceCreditBalanceBuilder {
                    used: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "used" => Deserialize::begin(&mut self.builder.used),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(used),) = (self.builder.used.take(),) else {
                return Ok(());
            };
            *self.out = Some(BankConnectionsResourceBalanceApiResourceCreditBalance { used });
            Ok(())
        }
    }
};
