#[derive(Clone, Debug)]
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
#[doc(hidden)]
pub struct BankConnectionsResourceBalanceApiResourceCreditBalanceBuilder {
    used: Option<Option<std::collections::HashMap<String, i64>>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder:
                    BankConnectionsResourceBalanceApiResourceCreditBalanceBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BankConnectionsResourceBalanceApiResourceCreditBalanceBuilder {
        type Out = BankConnectionsResourceBalanceApiResourceCreditBalance;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "used" => Deserialize::begin(&mut self.used),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { used: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(used),) = (self.used.take(),) else {
                return None;
            };
            Some(Self::Out { used })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for BankConnectionsResourceBalanceApiResourceCreditBalance {
        type Builder = BankConnectionsResourceBalanceApiResourceCreditBalanceBuilder;
    }

    impl FromValueOpt for BankConnectionsResourceBalanceApiResourceCreditBalance {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                BankConnectionsResourceBalanceApiResourceCreditBalanceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "used" => b.used = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
