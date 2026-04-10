/// ReservePlans are used to automatically place holds on a merchant's funds until the plan expires.
/// It takes a portion of each incoming Charge (including those resulting from a Transfer from a platform account).
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ReservePlan {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Indicates which party created this ReservePlan.
    pub created_by: ReservePlanCreatedBy,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    /// An unset currency indicates that the plan applies to all currencies.
    pub currency: Option<stripe_types::Currency>,
    /// Time at which the ReservePlan was disabled.
    pub disabled_at: Option<stripe_types::Timestamp>,
    pub fixed_release: Option<stripe_reserve::ReservesReservePlansResourcesFixedRelease>,
    /// Unique identifier for the object.
    pub id: stripe_reserve::ReservePlanId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The percent of each Charge to reserve.
    pub percent: i64,
    pub rolling_release: Option<stripe_reserve::ReservesReservePlansResourcesRollingRelease>,
    /// The current status of the ReservePlan. The ReservePlan only affects charges if it is `active`.
    pub status: ReservePlanStatus,
    /// The type of the ReservePlan.
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: ReservePlanType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ReservePlan {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ReservePlan").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ReservePlanBuilder {
    created: Option<stripe_types::Timestamp>,
    created_by: Option<ReservePlanCreatedBy>,
    currency: Option<Option<stripe_types::Currency>>,
    disabled_at: Option<Option<stripe_types::Timestamp>>,
    fixed_release: Option<Option<stripe_reserve::ReservesReservePlansResourcesFixedRelease>>,
    id: Option<stripe_reserve::ReservePlanId>,
    livemode: Option<bool>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    percent: Option<i64>,
    rolling_release: Option<Option<stripe_reserve::ReservesReservePlansResourcesRollingRelease>>,
    status: Option<ReservePlanStatus>,
    type_: Option<ReservePlanType>,
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

    impl Deserialize for ReservePlan {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ReservePlan>,
        builder: ReservePlanBuilder,
    }

    impl Visitor for Place<ReservePlan> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ReservePlanBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ReservePlanBuilder {
        type Out = ReservePlan;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.created),
                "created_by" => Deserialize::begin(&mut self.created_by),
                "currency" => Deserialize::begin(&mut self.currency),
                "disabled_at" => Deserialize::begin(&mut self.disabled_at),
                "fixed_release" => Deserialize::begin(&mut self.fixed_release),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "percent" => Deserialize::begin(&mut self.percent),
                "rolling_release" => Deserialize::begin(&mut self.rolling_release),
                "status" => Deserialize::begin(&mut self.status),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                created: Deserialize::default(),
                created_by: Deserialize::default(),
                currency: Deserialize::default(),
                disabled_at: Deserialize::default(),
                fixed_release: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                percent: Deserialize::default(),
                rolling_release: Deserialize::default(),
                status: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(created),
                Some(created_by),
                Some(currency),
                Some(disabled_at),
                Some(fixed_release),
                Some(id),
                Some(livemode),
                Some(metadata),
                Some(percent),
                Some(rolling_release),
                Some(status),
                Some(type_),
            ) = (
                self.created,
                self.created_by.take(),
                self.currency.take(),
                self.disabled_at,
                self.fixed_release,
                self.id.take(),
                self.livemode,
                self.metadata.take(),
                self.percent,
                self.rolling_release,
                self.status.take(),
                self.type_.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                created,
                created_by,
                currency,
                disabled_at,
                fixed_release,
                id,
                livemode,
                metadata,
                percent,
                rolling_release,
                status,
                type_,
            })
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

    impl ObjectDeser for ReservePlan {
        type Builder = ReservePlanBuilder;
    }

    impl FromValueOpt for ReservePlan {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ReservePlanBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "created" => b.created = FromValueOpt::from_value(v),
                    "created_by" => b.created_by = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "disabled_at" => b.disabled_at = FromValueOpt::from_value(v),
                    "fixed_release" => b.fixed_release = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "percent" => b.percent = FromValueOpt::from_value(v),
                    "rolling_release" => b.rolling_release = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for ReservePlan {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("ReservePlan", 13)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("created_by", &self.created_by)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("disabled_at", &self.disabled_at)?;
        s.serialize_field("fixed_release", &self.fixed_release)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("percent", &self.percent)?;
        s.serialize_field("rolling_release", &self.rolling_release)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("type", &self.type_)?;

        s.serialize_field("object", "reserve.plan")?;
        s.end()
    }
}
/// Indicates which party created this ReservePlan.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ReservePlanCreatedBy {
    Application,
    Stripe,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ReservePlanCreatedBy {
    pub fn as_str(&self) -> &str {
        use ReservePlanCreatedBy::*;
        match self {
            Application => "application",
            Stripe => "stripe",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ReservePlanCreatedBy {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ReservePlanCreatedBy::*;
        match s {
            "application" => Ok(Application),
            "stripe" => Ok(Stripe),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "ReservePlanCreatedBy");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ReservePlanCreatedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ReservePlanCreatedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ReservePlanCreatedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ReservePlanCreatedBy)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ReservePlanCreatedBy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ReservePlanCreatedBy {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ReservePlanCreatedBy> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ReservePlanCreatedBy::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ReservePlanCreatedBy);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ReservePlanCreatedBy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The current status of the ReservePlan. The ReservePlan only affects charges if it is `active`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ReservePlanStatus {
    Active,
    Disabled,
    Expired,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ReservePlanStatus {
    pub fn as_str(&self) -> &str {
        use ReservePlanStatus::*;
        match self {
            Active => "active",
            Disabled => "disabled",
            Expired => "expired",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ReservePlanStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ReservePlanStatus::*;
        match s {
            "active" => Ok(Active),
            "disabled" => Ok(Disabled),
            "expired" => Ok(Expired),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "ReservePlanStatus");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ReservePlanStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ReservePlanStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ReservePlanStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ReservePlanStatus)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ReservePlanStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ReservePlanStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ReservePlanStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ReservePlanStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ReservePlanStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ReservePlanStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The type of the ReservePlan.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ReservePlanType {
    FixedRelease,
    RollingRelease,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ReservePlanType {
    pub fn as_str(&self) -> &str {
        use ReservePlanType::*;
        match self {
            FixedRelease => "fixed_release",
            RollingRelease => "rolling_release",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ReservePlanType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ReservePlanType::*;
        match s {
            "fixed_release" => Ok(FixedRelease),
            "rolling_release" => Ok(RollingRelease),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "ReservePlanType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ReservePlanType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ReservePlanType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ReservePlanType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ReservePlanType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ReservePlanType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ReservePlanType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ReservePlanType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ReservePlanType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ReservePlanType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ReservePlanType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
impl stripe_types::Object for ReservePlan {
    type Id = stripe_reserve::ReservePlanId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(ReservePlanId);
