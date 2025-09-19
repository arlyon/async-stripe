#[derive(Clone, Debug)]
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
#[doc(hidden)]
pub struct BankConnectionsResourceBalanceApiResourceCashBalanceBuilder {
    available: Option<Option<std::collections::HashMap<String, i64>>>,
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
                builder: BankConnectionsResourceBalanceApiResourceCashBalanceBuilder::deser_default(
                ),
            }))
        }
    }

    impl MapBuilder for BankConnectionsResourceBalanceApiResourceCashBalanceBuilder {
        type Out = BankConnectionsResourceBalanceApiResourceCashBalance;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "available" => Deserialize::begin(&mut self.available),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { available: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(available),) = (self.available.take(),) else {
                return None;
            };
            Some(Self::Out { available })
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

    impl ObjectDeser for BankConnectionsResourceBalanceApiResourceCashBalance {
        type Builder = BankConnectionsResourceBalanceApiResourceCashBalanceBuilder;
    }

    impl FromValueOpt for BankConnectionsResourceBalanceApiResourceCashBalance {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                BankConnectionsResourceBalanceApiResourceCashBalanceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "available" => b.available = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
