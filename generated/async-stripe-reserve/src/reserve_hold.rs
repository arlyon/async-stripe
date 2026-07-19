/// ReserveHolds are used to place a temporary ReserveHold on a merchant's funds.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ReserveHold {
    /// Amount reserved.
    /// A positive integer representing how much is reserved in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal).
    pub amount: i64,
    /// Amount in cents that can be released from this ReserveHold
    pub amount_releasable: Option<i64>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Indicates which party created this ReserveHold.
    pub created_by: ReserveHoldCreatedBy,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Unique identifier for the object.
    pub id: stripe_reserve::ReserveHoldId,
    /// Whether there are any funds available to release on this ReserveHold.
    /// Note that if the ReserveHold is in the process of being released, this could be false, even though the funds haven't been fully released yet.
    pub is_releasable: Option<bool>,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The reason for the ReserveHold.
    pub reason: ReserveHoldReason,
    pub release_schedule: stripe_reserve::ReservesReserveHoldsResourcesReleaseSchedule,
    /// The ReservePlan which produced this ReserveHold (i.e., resplan_123)
    pub reserve_plan: Option<stripe_types::Expandable<stripe_reserve::ReservePlan>>,
    /// The Charge which funded this ReserveHold (e.g., ch_123)
    pub source_charge: Option<stripe_types::Expandable<stripe_shared::Charge>>,
    /// Which source balance type this ReserveHold reserves funds from.
    /// One of `bank_account`, `card`, or `fpx`.
    pub source_type: ReserveHoldSourceType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ReserveHold {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ReserveHold").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ReserveHoldBuilder {
    amount: Option<i64>,
    amount_releasable: Option<Option<i64>>,
    created: Option<stripe_types::Timestamp>,
    created_by: Option<ReserveHoldCreatedBy>,
    currency: Option<stripe_types::Currency>,
    id: Option<stripe_reserve::ReserveHoldId>,
    is_releasable: Option<Option<bool>>,
    livemode: Option<bool>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    reason: Option<ReserveHoldReason>,
    release_schedule: Option<stripe_reserve::ReservesReserveHoldsResourcesReleaseSchedule>,
    reserve_plan: Option<Option<stripe_types::Expandable<stripe_reserve::ReservePlan>>>,
    source_charge: Option<Option<stripe_types::Expandable<stripe_shared::Charge>>>,
    source_type: Option<ReserveHoldSourceType>,
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

    impl Deserialize for ReserveHold {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ReserveHold>,
        builder: ReserveHoldBuilder,
    }

    impl Visitor for Place<ReserveHold> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ReserveHoldBuilder {
                    amount: Deserialize::default(),
                    amount_releasable: Deserialize::default(),
                    created: Deserialize::default(),
                    created_by: Deserialize::default(),
                    currency: Deserialize::default(),
                    id: Deserialize::default(),
                    is_releasable: Deserialize::default(),
                    livemode: Deserialize::default(),
                    metadata: Deserialize::default(),
                    reason: Deserialize::default(),
                    release_schedule: Deserialize::default(),
                    reserve_plan: Deserialize::default(),
                    source_charge: Deserialize::default(),
                    source_type: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "amount_releasable" => Deserialize::begin(&mut self.builder.amount_releasable),
                "created" => Deserialize::begin(&mut self.builder.created),
                "created_by" => Deserialize::begin(&mut self.builder.created_by),
                "currency" => Deserialize::begin(&mut self.builder.currency),
                "id" => Deserialize::begin(&mut self.builder.id),
                "is_releasable" => Deserialize::begin(&mut self.builder.is_releasable),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "reason" => Deserialize::begin(&mut self.builder.reason),
                "release_schedule" => Deserialize::begin(&mut self.builder.release_schedule),
                "reserve_plan" => Deserialize::begin(&mut self.builder.reserve_plan),
                "source_charge" => Deserialize::begin(&mut self.builder.source_charge),
                "source_type" => Deserialize::begin(&mut self.builder.source_type),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(amount),
                Some(amount_releasable),
                Some(created),
                Some(created_by),
                Some(currency),
                Some(id),
                Some(is_releasable),
                Some(livemode),
                Some(metadata),
                Some(reason),
                Some(release_schedule),
                Some(reserve_plan),
                Some(source_charge),
                Some(source_type),
            ) = (
                self.builder.amount,
                self.builder.amount_releasable,
                self.builder.created,
                self.builder.created_by.take(),
                self.builder.currency.take(),
                self.builder.id.take(),
                self.builder.is_releasable,
                self.builder.livemode,
                self.builder.metadata.take(),
                self.builder.reason.take(),
                self.builder.release_schedule,
                self.builder.reserve_plan.take(),
                self.builder.source_charge.take(),
                self.builder.source_type.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(ReserveHold {
                amount,
                amount_releasable,
                created,
                created_by,
                currency,
                id,
                is_releasable,
                livemode,
                metadata,
                reason,
                release_schedule,
                reserve_plan,
                source_charge,
                source_type,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for ReserveHold {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("ReserveHold", 15)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("amount_releasable", &self.amount_releasable)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("created_by", &self.created_by)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("is_releasable", &self.is_releasable)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("reason", &self.reason)?;
        s.serialize_field("release_schedule", &self.release_schedule)?;
        s.serialize_field("reserve_plan", &self.reserve_plan)?;
        s.serialize_field("source_charge", &self.source_charge)?;
        s.serialize_field("source_type", &self.source_type)?;

        s.serialize_field("object", "reserve.hold")?;
        s.end()
    }
}
/// Indicates which party created this ReserveHold.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ReserveHoldCreatedBy {
    Application,
    Stripe,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ReserveHoldCreatedBy {
    pub fn as_str(&self) -> &str {
        use ReserveHoldCreatedBy::*;
        match self {
            Application => "application",
            Stripe => "stripe",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ReserveHoldCreatedBy {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ReserveHoldCreatedBy::*;
        match s {
            "application" => Ok(Application),
            "stripe" => Ok(Stripe),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "ReserveHoldCreatedBy");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ReserveHoldCreatedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ReserveHoldCreatedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ReserveHoldCreatedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ReserveHoldCreatedBy)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ReserveHoldCreatedBy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for ReserveHoldCreatedBy {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<ReserveHoldCreatedBy> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ReserveHoldCreatedBy::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ReserveHoldCreatedBy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The reason for the ReserveHold.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ReserveHoldReason {
    Charge,
    Standalone,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ReserveHoldReason {
    pub fn as_str(&self) -> &str {
        use ReserveHoldReason::*;
        match self {
            Charge => "charge",
            Standalone => "standalone",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ReserveHoldReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ReserveHoldReason::*;
        match s {
            "charge" => Ok(Charge),
            "standalone" => Ok(Standalone),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "ReserveHoldReason");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ReserveHoldReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ReserveHoldReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ReserveHoldReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ReserveHoldReason)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ReserveHoldReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for ReserveHoldReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<ReserveHoldReason> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ReserveHoldReason::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ReserveHoldReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Which source balance type this ReserveHold reserves funds from.
/// One of `bank_account`, `card`, or `fpx`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ReserveHoldSourceType {
    BankAccount,
    Card,
    Fpx,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ReserveHoldSourceType {
    pub fn as_str(&self) -> &str {
        use ReserveHoldSourceType::*;
        match self {
            BankAccount => "bank_account",
            Card => "card",
            Fpx => "fpx",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ReserveHoldSourceType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ReserveHoldSourceType::*;
        match s {
            "bank_account" => Ok(BankAccount),
            "card" => Ok(Card),
            "fpx" => Ok(Fpx),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "ReserveHoldSourceType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ReserveHoldSourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ReserveHoldSourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ReserveHoldSourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ReserveHoldSourceType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ReserveHoldSourceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for ReserveHoldSourceType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<ReserveHoldSourceType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ReserveHoldSourceType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ReserveHoldSourceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
impl stripe_types::Object for ReserveHold {
    type Id = stripe_reserve::ReserveHoldId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(ReserveHoldId);
