/// A billing meter event adjustment is a resource that allows you to cancel a meter event.
/// For example, you might create a billing meter event adjustment to cancel a meter event that was created in error or attached to the wrong customer.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingMeterEventAdjustment {
    /// Specifies which event to cancel.
    pub cancel: Option<stripe_billing::BillingMeterResourceBillingMeterEventAdjustmentCancel>,
    /// The name of the meter event. Corresponds with the `event_name` field on a meter.
    pub event_name: String,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The meter event adjustment's status.
    pub status: BillingMeterEventAdjustmentStatus,
    /// Specifies whether to cancel a single event or a range of events for a time period.
    /// Time period cancellation is not supported yet.
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: stripe_billing::BillingMeterEventAdjustmentType,
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
                builder: BillingMeterEventAdjustmentBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BillingMeterEventAdjustmentBuilder {
        type Out = BillingMeterEventAdjustment;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "cancel" => Deserialize::begin(&mut self.cancel),
                "event_name" => Deserialize::begin(&mut self.event_name),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "status" => Deserialize::begin(&mut self.status),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                cancel: Deserialize::default(),
                event_name: Deserialize::default(),
                livemode: Deserialize::default(),
                status: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(cancel), Some(event_name), Some(livemode), Some(status), Some(type_)) = (
                self.cancel.take(),
                self.event_name.take(),
                self.livemode,
                self.status,
                self.type_,
            ) else {
                return None;
            };
            Some(Self::Out { cancel, event_name, livemode, status, type_ })
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

    impl ObjectDeser for BillingMeterEventAdjustment {
        type Builder = BillingMeterEventAdjustmentBuilder;
    }

    impl FromValueOpt for BillingMeterEventAdjustment {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BillingMeterEventAdjustmentBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "cancel" => b.cancel = FromValueOpt::from_value(v),
                    "event_name" => b.event_name = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BillingMeterEventAdjustmentStatus {
    Complete,
    Pending,
}
impl BillingMeterEventAdjustmentStatus {
    pub fn as_str(self) -> &'static str {
        use BillingMeterEventAdjustmentStatus::*;
        match self {
            Complete => "complete",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for BillingMeterEventAdjustmentStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingMeterEventAdjustmentStatus::*;
        match s {
            "complete" => Ok(Complete),
            "pending" => Ok(Pending),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for BillingMeterEventAdjustmentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BillingMeterEventAdjustmentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for BillingMeterEventAdjustmentStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<BillingMeterEventAdjustmentStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(BillingMeterEventAdjustmentStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(BillingMeterEventAdjustmentStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingMeterEventAdjustmentStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for BillingMeterEventAdjustmentStatus")
        })
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BillingMeterEventAdjustmentType {
    Cancel,
}
impl BillingMeterEventAdjustmentType {
    pub fn as_str(self) -> &'static str {
        use BillingMeterEventAdjustmentType::*;
        match self {
            Cancel => "cancel",
        }
    }
}

impl std::str::FromStr for BillingMeterEventAdjustmentType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingMeterEventAdjustmentType::*;
        match s {
            "cancel" => Ok(Cancel),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for BillingMeterEventAdjustmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BillingMeterEventAdjustmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for BillingMeterEventAdjustmentType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<BillingMeterEventAdjustmentType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(BillingMeterEventAdjustmentType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(BillingMeterEventAdjustmentType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingMeterEventAdjustmentType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for BillingMeterEventAdjustmentType")
        })
    }
}
