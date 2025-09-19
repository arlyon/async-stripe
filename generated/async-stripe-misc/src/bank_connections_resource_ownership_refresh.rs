#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BankConnectionsResourceOwnershipRefresh {
    /// The time at which the last refresh attempt was initiated. Measured in seconds since the Unix epoch.
    pub last_attempted_at: stripe_types::Timestamp,
    /// Time at which the next ownership refresh can be initiated.
    /// This value will be `null` when `status` is `pending`.
    /// Measured in seconds since the Unix epoch.
    pub next_refresh_available_at: Option<stripe_types::Timestamp>,
    /// The status of the last refresh attempt.
    pub status: BankConnectionsResourceOwnershipRefreshStatus,
}
#[doc(hidden)]
pub struct BankConnectionsResourceOwnershipRefreshBuilder {
    last_attempted_at: Option<stripe_types::Timestamp>,
    next_refresh_available_at: Option<Option<stripe_types::Timestamp>>,
    status: Option<BankConnectionsResourceOwnershipRefreshStatus>,
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

    impl Deserialize for BankConnectionsResourceOwnershipRefresh {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BankConnectionsResourceOwnershipRefresh>,
        builder: BankConnectionsResourceOwnershipRefreshBuilder,
    }

    impl Visitor for Place<BankConnectionsResourceOwnershipRefresh> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BankConnectionsResourceOwnershipRefreshBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BankConnectionsResourceOwnershipRefreshBuilder {
        type Out = BankConnectionsResourceOwnershipRefresh;
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
                (self.last_attempted_at, self.next_refresh_available_at, self.status)
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

    impl ObjectDeser for BankConnectionsResourceOwnershipRefresh {
        type Builder = BankConnectionsResourceOwnershipRefreshBuilder;
    }

    impl FromValueOpt for BankConnectionsResourceOwnershipRefresh {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BankConnectionsResourceOwnershipRefreshBuilder::deser_default();
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BankConnectionsResourceOwnershipRefreshStatus {
    Failed,
    Pending,
    Succeeded,
}
impl BankConnectionsResourceOwnershipRefreshStatus {
    pub fn as_str(self) -> &'static str {
        use BankConnectionsResourceOwnershipRefreshStatus::*;
        match self {
            Failed => "failed",
            Pending => "pending",
            Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for BankConnectionsResourceOwnershipRefreshStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BankConnectionsResourceOwnershipRefreshStatus::*;
        match s {
            "failed" => Ok(Failed),
            "pending" => Ok(Pending),
            "succeeded" => Ok(Succeeded),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for BankConnectionsResourceOwnershipRefreshStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BankConnectionsResourceOwnershipRefreshStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for BankConnectionsResourceOwnershipRefreshStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for BankConnectionsResourceOwnershipRefreshStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<BankConnectionsResourceOwnershipRefreshStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            BankConnectionsResourceOwnershipRefreshStatus::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(BankConnectionsResourceOwnershipRefreshStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BankConnectionsResourceOwnershipRefreshStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for BankConnectionsResourceOwnershipRefreshStatus",
            )
        })
    }
}
