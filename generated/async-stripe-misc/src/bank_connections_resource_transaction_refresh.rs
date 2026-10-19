#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BankConnectionsResourceTransactionRefresh {
    /// Unique identifier for the object.
    pub id: stripe_misc::BankConnectionsResourceTransactionRefreshId,
    /// The time at which the last refresh attempt was initiated. Measured in seconds since the Unix epoch.
    pub last_attempted_at: stripe_types::Timestamp,
    /// Time at which the next transaction refresh can be initiated.
    /// This value will be `null` when `status` is `pending`.
    /// Measured in seconds since the Unix epoch.
    pub next_refresh_available_at: Option<stripe_types::Timestamp>,
    /// The status of the last refresh attempt.
    pub status: stripe_shared::ChargeStatus,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BankConnectionsResourceTransactionRefresh {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BankConnectionsResourceTransactionRefresh").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BankConnectionsResourceTransactionRefreshBuilder {
    id: Option<stripe_misc::BankConnectionsResourceTransactionRefreshId>,
    last_attempted_at: Option<stripe_types::Timestamp>,
    next_refresh_available_at: Option<Option<stripe_types::Timestamp>>,
    status: Option<stripe_shared::ChargeStatus>,
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

    impl Deserialize for BankConnectionsResourceTransactionRefresh {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BankConnectionsResourceTransactionRefresh>,
        builder: BankConnectionsResourceTransactionRefreshBuilder,
    }

    impl Visitor for Place<BankConnectionsResourceTransactionRefresh> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BankConnectionsResourceTransactionRefreshBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BankConnectionsResourceTransactionRefreshBuilder {
        type Out = BankConnectionsResourceTransactionRefresh;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "id" => Deserialize::begin(&mut self.id),
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
                id: None,
                last_attempted_at: None,
                next_refresh_available_at: Some(None),
                status: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(id), Some(last_attempted_at), Some(next_refresh_available_at), Some(status)) = (
                self.id.take(),
                self.last_attempted_at,
                self.next_refresh_available_at,
                self.status.take(),
            ) else {
                return None;
            };
            Some(Self::Out { id, last_attempted_at, next_refresh_available_at, status })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for BankConnectionsResourceTransactionRefresh {
        type Builder = BankConnectionsResourceTransactionRefreshBuilder;
    }

    impl FromValueOpt for BankConnectionsResourceTransactionRefresh {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BankConnectionsResourceTransactionRefreshBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "id" => b.id = FromValueOpt::from_value(v),
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
impl stripe_types::Object for BankConnectionsResourceTransactionRefresh {
    type Id = stripe_misc::BankConnectionsResourceTransactionRefreshId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(BankConnectionsResourceTransactionRefreshId);
/// The status of the last refresh attempt.
pub type BankConnectionsResourceTransactionRefreshStatus = stripe_shared::ChargeStatus;
