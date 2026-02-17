#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BankConnectionsResourceBalanceRefresh {
    /// The time at which the last refresh attempt was initiated. Measured in seconds since the Unix epoch.
    pub last_attempted_at: stripe_types::Timestamp,
    /// Time at which the next balance refresh can be initiated.
    /// This value will be `null` when `status` is `pending`.
    /// Measured in seconds since the Unix epoch.
    pub next_refresh_available_at: Option<stripe_types::Timestamp>,
    /// The status of the last refresh attempt.
    pub status: BankConnectionsResourceBalanceRefreshStatus,
}
#[doc(hidden)]
pub struct BankConnectionsResourceBalanceRefreshBuilder {
    last_attempted_at: Option<stripe_types::Timestamp>,
    next_refresh_available_at: Option<Option<stripe_types::Timestamp>>,
    status: Option<BankConnectionsResourceBalanceRefreshStatus>,
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

    impl Deserialize for BankConnectionsResourceBalanceRefresh {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BankConnectionsResourceBalanceRefresh>,
        builder: BankConnectionsResourceBalanceRefreshBuilder,
    }

    impl Visitor for Place<BankConnectionsResourceBalanceRefresh> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BankConnectionsResourceBalanceRefreshBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BankConnectionsResourceBalanceRefreshBuilder {
        type Out = BankConnectionsResourceBalanceRefresh;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "last_attempted_at" => Deserialize::begin(&mut self.last_attempted_at),
                "next_refresh_available_at" => {
                    Deserialize::begin(&mut self.next_refresh_available_at)
                }
                "status" => Deserialize::begin(&mut self.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                last_attempted_at: Deserialize::default(),
                next_refresh_available_at: Deserialize::default(),
                status: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(last_attempted_at), Some(next_refresh_available_at), Some(status)) =
                (self.last_attempted_at, self.next_refresh_available_at, self.status.take())
            else {
                return None;
            };
            Some(Self::Out { last_attempted_at, next_refresh_available_at, status })
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

    impl ObjectDeser for BankConnectionsResourceBalanceRefresh {
        type Builder = BankConnectionsResourceBalanceRefreshBuilder;
    }

    impl FromValueOpt for BankConnectionsResourceBalanceRefresh {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BankConnectionsResourceBalanceRefreshBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "last_attempted_at" => b.last_attempted_at = FromValueOpt::from_value(v),
                    "next_refresh_available_at" => {
                        b.next_refresh_available_at = FromValueOpt::from_value(v)
                    }
                    "status" => b.status = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The status of the last refresh attempt.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum BankConnectionsResourceBalanceRefreshStatus {
    Failed,
    Pending,
    Succeeded,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl BankConnectionsResourceBalanceRefreshStatus {
    pub fn as_str(&self) -> &str {
        use BankConnectionsResourceBalanceRefreshStatus::*;
        match self {
            Failed => "failed",
            Pending => "pending",
            Succeeded => "succeeded",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for BankConnectionsResourceBalanceRefreshStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BankConnectionsResourceBalanceRefreshStatus::*;
        match s {
            "failed" => Ok(Failed),
            "pending" => Ok(Pending),
            "succeeded" => Ok(Succeeded),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "BankConnectionsResourceBalanceRefreshStatus"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for BankConnectionsResourceBalanceRefreshStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BankConnectionsResourceBalanceRefreshStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for BankConnectionsResourceBalanceRefreshStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for BankConnectionsResourceBalanceRefreshStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<BankConnectionsResourceBalanceRefreshStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(BankConnectionsResourceBalanceRefreshStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(BankConnectionsResourceBalanceRefreshStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BankConnectionsResourceBalanceRefreshStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
