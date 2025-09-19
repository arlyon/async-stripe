#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BankConnectionsResourceBalance {
    /// The time that the external institution calculated this balance.
    /// Measured in seconds since the Unix epoch.
    pub as_of: stripe_types::Timestamp,
    pub cash: Option<stripe_misc::BankConnectionsResourceBalanceApiResourceCashBalance>,
    pub credit: Option<stripe_misc::BankConnectionsResourceBalanceApiResourceCreditBalance>,
    /// The balances owed to (or by) the account holder, before subtracting any outbound pending transactions or adding any inbound pending transactions.
    ///
    /// Each key is a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Each value is a integer amount.
    /// A positive amount indicates money owed to the account holder.
    /// A negative amount indicates money owed by the account holder.
    pub current: std::collections::HashMap<String, i64>,
    /// The `type` of the balance.
    /// An additional hash is included on the balance with a name matching this value.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: BankConnectionsResourceBalanceType,
}
#[doc(hidden)]
pub struct BankConnectionsResourceBalanceBuilder {
    as_of: Option<stripe_types::Timestamp>,
    cash: Option<Option<stripe_misc::BankConnectionsResourceBalanceApiResourceCashBalance>>,
    credit: Option<Option<stripe_misc::BankConnectionsResourceBalanceApiResourceCreditBalance>>,
    current: Option<std::collections::HashMap<String, i64>>,
    type_: Option<BankConnectionsResourceBalanceType>,
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

    impl Deserialize for BankConnectionsResourceBalance {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BankConnectionsResourceBalance>,
        builder: BankConnectionsResourceBalanceBuilder,
    }

    impl Visitor for Place<BankConnectionsResourceBalance> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BankConnectionsResourceBalanceBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BankConnectionsResourceBalanceBuilder {
        type Out = BankConnectionsResourceBalance;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "as_of" => Deserialize::begin(&mut self.as_of),
                "cash" => Deserialize::begin(&mut self.cash),
                "credit" => Deserialize::begin(&mut self.credit),
                "current" => Deserialize::begin(&mut self.current),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                as_of: Deserialize::default(),
                cash: Deserialize::default(),
                credit: Deserialize::default(),
                current: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(as_of), Some(cash), Some(credit), Some(current), Some(type_)) =
                (self.as_of, self.cash.take(), self.credit.take(), self.current.take(), self.type_)
            else {
                return None;
            };
            Some(Self::Out { as_of, cash, credit, current, type_ })
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

    impl ObjectDeser for BankConnectionsResourceBalance {
        type Builder = BankConnectionsResourceBalanceBuilder;
    }

    impl FromValueOpt for BankConnectionsResourceBalance {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BankConnectionsResourceBalanceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "as_of" => b.as_of = FromValueOpt::from_value(v),
                    "cash" => b.cash = FromValueOpt::from_value(v),
                    "credit" => b.credit = FromValueOpt::from_value(v),
                    "current" => b.current = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The `type` of the balance.
/// An additional hash is included on the balance with a name matching this value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BankConnectionsResourceBalanceType {
    Cash,
    Credit,
}
impl BankConnectionsResourceBalanceType {
    pub fn as_str(self) -> &'static str {
        use BankConnectionsResourceBalanceType::*;
        match self {
            Cash => "cash",
            Credit => "credit",
        }
    }
}

impl std::str::FromStr for BankConnectionsResourceBalanceType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BankConnectionsResourceBalanceType::*;
        match s {
            "cash" => Ok(Cash),
            "credit" => Ok(Credit),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for BankConnectionsResourceBalanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BankConnectionsResourceBalanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for BankConnectionsResourceBalanceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for BankConnectionsResourceBalanceType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<BankConnectionsResourceBalanceType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(BankConnectionsResourceBalanceType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(BankConnectionsResourceBalanceType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BankConnectionsResourceBalanceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for BankConnectionsResourceBalanceType")
        })
    }
}
