#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BankConnectionsResourceBalanceApiResourceCashBalance {
    /// The funds available to the account holder.
    /// Typically this is the current balance after subtracting any outbound pending transactions and adding any inbound pending transactions.
    ///
    /// Each key is a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Each value is a integer amount.
    /// A positive amount indicates money owed to the account holder.
    /// A negative amount indicates money owed by the account holder.
    pub available: Option<std::collections::HashMap<String, i64>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BankConnectionsResourceBalanceApiResourceCashBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BankConnectionsResourceBalanceApiResourceCashBalance")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BankConnectionsResourceBalanceApiResourceCashBalanceBuilder {
    available: Option<Option<std::collections::HashMap<String, i64>>>,
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

    impl Deserialize for BankConnectionsResourceBalanceApiResourceCashBalance {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BankConnectionsResourceBalanceApiResourceCashBalance>,
        builder: BankConnectionsResourceBalanceApiResourceCashBalanceBuilder,
    }

    impl Visitor for Place<BankConnectionsResourceBalanceApiResourceCashBalance> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BankConnectionsResourceBalanceApiResourceCashBalanceBuilder {
                    available: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "available" => Deserialize::begin(&mut self.builder.available),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(available),) = (self.builder.available.take(),) else {
                return Ok(());
            };
            *self.out = Some(BankConnectionsResourceBalanceApiResourceCashBalance { available });
            Ok(())
        }
    }
};
