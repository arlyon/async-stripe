/// A billing meter event adjustment is a resource that allows you to cancel a meter event.
/// For example, you might create a billing meter event adjustment to cancel a meter event that was created in error or attached to the wrong customer.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingMeterEventAdjustment {
    /// Specifies which event to cancel.
    pub cancel: Option<stripe_billing::BillingMeterResourceBillingMeterEventAdjustmentCancel>,
    /// The name of the meter event. Corresponds with the `event_name` field on a meter.
    pub event_name: String,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// The meter event adjustment's status.
    pub status: BillingMeterEventAdjustmentStatus,
    /// Specifies whether to cancel a single event or a range of events for a time period.
    /// Time period cancellation is not supported yet.
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: stripe_billing::BillingMeterEventAdjustmentType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingMeterEventAdjustment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingMeterEventAdjustment").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BillingMeterEventAdjustmentBuilder {
    cancel: Option<Option<stripe_billing::BillingMeterResourceBillingMeterEventAdjustmentCancel>>,
    event_name: Option<String>,
    livemode: Option<bool>,
    status: Option<BillingMeterEventAdjustmentStatus>,
    type_: Option<stripe_billing::BillingMeterEventAdjustmentType>,
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

    impl Deserialize for BillingMeterEventAdjustment {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingMeterEventAdjustment>,
        builder: BillingMeterEventAdjustmentBuilder,
    }

    impl Visitor for Place<BillingMeterEventAdjustment> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BillingMeterEventAdjustmentBuilder {
                    cancel: Deserialize::default(),
                    event_name: Deserialize::default(),
                    livemode: Deserialize::default(),
                    status: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "cancel" => Deserialize::begin(&mut self.builder.cancel),
                "event_name" => Deserialize::begin(&mut self.builder.event_name),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "status" => Deserialize::begin(&mut self.builder.status),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(cancel), Some(event_name), Some(livemode), Some(status), Some(type_)) = (
                self.builder.cancel.take(),
                self.builder.event_name.take(),
                self.builder.livemode,
                self.builder.status.take(),
                self.builder.type_.take(),
            ) else {
                return Ok(());
            };
            *self.out =
                Some(BillingMeterEventAdjustment { cancel, event_name, livemode, status, type_ });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for BillingMeterEventAdjustment {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("BillingMeterEventAdjustment", 6)?;
        s.serialize_field("cancel", &self.cancel)?;
        s.serialize_field("event_name", &self.event_name)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("type", &self.type_)?;

        s.serialize_field("object", "billing.meter_event_adjustment")?;
        s.end()
    }
}
/// The meter event adjustment's status.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum BillingMeterEventAdjustmentStatus {
    Complete,
    Pending,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl BillingMeterEventAdjustmentStatus {
    pub fn as_str(&self) -> &str {
        use BillingMeterEventAdjustmentStatus::*;
        match self {
            Complete => "complete",
            Pending => "pending",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for BillingMeterEventAdjustmentStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingMeterEventAdjustmentStatus::*;
        match s {
            "complete" => Ok(Complete),
            "pending" => Ok(Pending),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "BillingMeterEventAdjustmentStatus"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for BillingMeterEventAdjustmentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for BillingMeterEventAdjustmentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingMeterEventAdjustmentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(BillingMeterEventAdjustmentStatus)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for BillingMeterEventAdjustmentStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for BillingMeterEventAdjustmentStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<BillingMeterEventAdjustmentStatus> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BillingMeterEventAdjustmentStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingMeterEventAdjustmentStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum BillingMeterEventAdjustmentType {
    Cancel,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl BillingMeterEventAdjustmentType {
    pub fn as_str(&self) -> &str {
        use BillingMeterEventAdjustmentType::*;
        match self {
            Cancel => "cancel",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for BillingMeterEventAdjustmentType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingMeterEventAdjustmentType::*;
        match s {
            "cancel" => Ok(Cancel),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "BillingMeterEventAdjustmentType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for BillingMeterEventAdjustmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for BillingMeterEventAdjustmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingMeterEventAdjustmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(BillingMeterEventAdjustmentType)).finish_non_exhaustive()
    }
}
impl serde::Serialize for BillingMeterEventAdjustmentType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for BillingMeterEventAdjustmentType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<BillingMeterEventAdjustmentType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BillingMeterEventAdjustmentType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingMeterEventAdjustmentType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
