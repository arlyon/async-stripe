/// ReserveReleases represent the release of funds from a ReserveHold.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ReserveRelease {
    /// Amount released.
    /// A positive integer representing how much is released in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal).
    pub amount: i64,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Indicates which party created this ReserveRelease.
    pub created_by: ReserveReleaseCreatedBy,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Unique identifier for the object.
    pub id: stripe_reserve::ReserveReleaseId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The reason for the ReserveRelease, indicating why the funds were released.
    pub reason: ReserveReleaseReason,
    /// The release timestamp of the funds.
    pub released_at: stripe_types::Timestamp,
    /// The ReserveHold this ReserveRelease is associated with.
    pub reserve_hold: Option<stripe_types::Expandable<stripe_reserve::ReserveHold>>,
    /// The ReservePlan ID this ReserveRelease is associated with.
    /// This field is only populated if a ReserveRelease is created by a ReservePlan disable operation, or from a scheduled ReservedHold expiry.
    pub reserve_plan: Option<stripe_types::Expandable<stripe_reserve::ReservePlan>>,
    pub source_transaction:
        Option<stripe_reserve::ReservesReserveReleasesResourcesSourceTransaction>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ReserveRelease {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ReserveRelease").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ReserveReleaseBuilder {
    amount: Option<i64>,
    created: Option<stripe_types::Timestamp>,
    created_by: Option<ReserveReleaseCreatedBy>,
    currency: Option<stripe_types::Currency>,
    id: Option<stripe_reserve::ReserveReleaseId>,
    livemode: Option<bool>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    reason: Option<ReserveReleaseReason>,
    released_at: Option<stripe_types::Timestamp>,
    reserve_hold: Option<Option<stripe_types::Expandable<stripe_reserve::ReserveHold>>>,
    reserve_plan: Option<Option<stripe_types::Expandable<stripe_reserve::ReservePlan>>>,
    source_transaction:
        Option<Option<stripe_reserve::ReservesReserveReleasesResourcesSourceTransaction>>,
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

    impl Deserialize for ReserveRelease {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ReserveRelease>,
        builder: ReserveReleaseBuilder,
    }

    impl Visitor for Place<ReserveRelease> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ReserveReleaseBuilder {
                    amount: Deserialize::default(),
                    created: Deserialize::default(),
                    created_by: Deserialize::default(),
                    currency: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                    metadata: Deserialize::default(),
                    reason: Deserialize::default(),
                    released_at: Deserialize::default(),
                    reserve_hold: Deserialize::default(),
                    reserve_plan: Deserialize::default(),
                    source_transaction: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "created" => Deserialize::begin(&mut self.builder.created),
                "created_by" => Deserialize::begin(&mut self.builder.created_by),
                "currency" => Deserialize::begin(&mut self.builder.currency),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "reason" => Deserialize::begin(&mut self.builder.reason),
                "released_at" => Deserialize::begin(&mut self.builder.released_at),
                "reserve_hold" => Deserialize::begin(&mut self.builder.reserve_hold),
                "reserve_plan" => Deserialize::begin(&mut self.builder.reserve_plan),
                "source_transaction" => Deserialize::begin(&mut self.builder.source_transaction),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(amount),
                Some(created),
                Some(created_by),
                Some(currency),
                Some(id),
                Some(livemode),
                Some(metadata),
                Some(reason),
                Some(released_at),
                Some(reserve_hold),
                Some(reserve_plan),
                Some(source_transaction),
            ) = (
                self.builder.amount,
                self.builder.created,
                self.builder.created_by.take(),
                self.builder.currency.take(),
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.metadata.take(),
                self.builder.reason.take(),
                self.builder.released_at,
                self.builder.reserve_hold.take(),
                self.builder.reserve_plan.take(),
                self.builder.source_transaction.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(ReserveRelease {
                amount,
                created,
                created_by,
                currency,
                id,
                livemode,
                metadata,
                reason,
                released_at,
                reserve_hold,
                reserve_plan,
                source_transaction,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for ReserveRelease {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("ReserveRelease", 13)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("created_by", &self.created_by)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("reason", &self.reason)?;
        s.serialize_field("released_at", &self.released_at)?;
        s.serialize_field("reserve_hold", &self.reserve_hold)?;
        s.serialize_field("reserve_plan", &self.reserve_plan)?;
        s.serialize_field("source_transaction", &self.source_transaction)?;

        s.serialize_field("object", "reserve.release")?;
        s.end()
    }
}
/// Indicates which party created this ReserveRelease.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ReserveReleaseCreatedBy {
    Application,
    Stripe,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ReserveReleaseCreatedBy {
    pub fn as_str(&self) -> &str {
        use ReserveReleaseCreatedBy::*;
        match self {
            Application => "application",
            Stripe => "stripe",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ReserveReleaseCreatedBy {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ReserveReleaseCreatedBy::*;
        match s {
            "application" => Ok(Application),
            "stripe" => Ok(Stripe),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "ReserveReleaseCreatedBy");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ReserveReleaseCreatedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ReserveReleaseCreatedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ReserveReleaseCreatedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ReserveReleaseCreatedBy)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ReserveReleaseCreatedBy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for ReserveReleaseCreatedBy {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<ReserveReleaseCreatedBy> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ReserveReleaseCreatedBy::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ReserveReleaseCreatedBy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The reason for the ReserveRelease, indicating why the funds were released.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ReserveReleaseReason {
    BulkHoldExpiry,
    HoldReleasedEarly,
    HoldReversed,
    PlanDisabled,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ReserveReleaseReason {
    pub fn as_str(&self) -> &str {
        use ReserveReleaseReason::*;
        match self {
            BulkHoldExpiry => "bulk_hold_expiry",
            HoldReleasedEarly => "hold_released_early",
            HoldReversed => "hold_reversed",
            PlanDisabled => "plan_disabled",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ReserveReleaseReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ReserveReleaseReason::*;
        match s {
            "bulk_hold_expiry" => Ok(BulkHoldExpiry),
            "hold_released_early" => Ok(HoldReleasedEarly),
            "hold_reversed" => Ok(HoldReversed),
            "plan_disabled" => Ok(PlanDisabled),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "ReserveReleaseReason");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ReserveReleaseReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ReserveReleaseReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ReserveReleaseReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ReserveReleaseReason)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ReserveReleaseReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for ReserveReleaseReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<ReserveReleaseReason> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ReserveReleaseReason::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ReserveReleaseReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
impl stripe_types::Object for ReserveRelease {
    type Id = stripe_reserve::ReserveReleaseId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(ReserveReleaseId);
