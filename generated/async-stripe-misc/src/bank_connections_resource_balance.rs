#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BankConnectionsResourceBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BankConnectionsResourceBalance").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: BankConnectionsResourceBalanceBuilder {
                    as_of: Deserialize::default(),
                    cash: Deserialize::default(),
                    credit: Deserialize::default(),
                    current: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "as_of" => Deserialize::begin(&mut self.builder.as_of),
                "cash" => Deserialize::begin(&mut self.builder.cash),
                "credit" => Deserialize::begin(&mut self.builder.credit),
                "current" => Deserialize::begin(&mut self.builder.current),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(as_of), Some(cash), Some(credit), Some(current), Some(type_)) = (
                self.builder.as_of,
                self.builder.cash.take(),
                self.builder.credit.take(),
                self.builder.current.take(),
                self.builder.type_.take(),
            ) else {
                return Ok(());
            };
            *self.out =
                Some(BankConnectionsResourceBalance { as_of, cash, credit, current, type_ });
            Ok(())
        }
    }
};
/// The `type` of the balance.
/// An additional hash is included on the balance with a name matching this value.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum BankConnectionsResourceBalanceType {
    Cash,
    Credit,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl BankConnectionsResourceBalanceType {
    pub fn as_str(&self) -> &str {
        use BankConnectionsResourceBalanceType::*;
        match self {
            Cash => "cash",
            Credit => "credit",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for BankConnectionsResourceBalanceType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BankConnectionsResourceBalanceType::*;
        match s {
            "cash" => Ok(Cash),
            "credit" => Ok(Credit),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "BankConnectionsResourceBalanceType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for BankConnectionsResourceBalanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for BankConnectionsResourceBalanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BankConnectionsResourceBalanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(BankConnectionsResourceBalanceType)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for BankConnectionsResourceBalanceType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<BankConnectionsResourceBalanceType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BankConnectionsResourceBalanceType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BankConnectionsResourceBalanceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
